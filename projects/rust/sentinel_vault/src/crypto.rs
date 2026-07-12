use aes_gcm::{
    aead::{Aead, KeyInit, consts::U12},
    Aes256Gcm, Key, Nonce,
};
use zeroize::{Zeroize, ZeroizeOnDrop};
use crate::erreurs::VaultError;


pub const TAILLE_NONCE: usize = 12; // 96 bits, taille imposée par GCM

/// Encapsuler le chiffreur, pour garantir l'effacement mémoire automatique à la destruction.
#[derive(ZeroizeOnDrop)]
pub struct MoteurCrypto {
    #[zeroize(skip)] // Aes256Gcm ne contient pas la clé en clair après initialisation
    chiffreur: Aes256Gcm,
}

impl MoteurCrypto {
    /// Initialiser le moteur à partir d'une clé de 32 octets exactement.
    pub fn nouveau(cle: &[u8]) -> Result<Self, VaultError> {
        if cle.len() != 32 {
            return Err(VaultError::ChiffrementInvalide { attendu: 32, obtenu: cle.len() });
        }
        let cle_typee: &Key<Aes256Gcm> = cle.try_into()
            .map_err(|_| VaultError::ChiffrementInvalide { attendu: 32, obtenu: cle.len() })?;
        Ok(MoteurCrypto { chiffreur: Aes256Gcm::new(cle_typee) })
    }

    /// Générer un nonce via getrandom, pour s'affranchir des instabilités d'API du crate rand.
    fn generer_nonce(&self) -> Result<[u8; TAILLE_NONCE], VaultError> {
        let mut octets = [0u8; TAILLE_NONCE];
        getrandom::fill(&mut octets).map_err(|_| VaultError::EchecChiffrement)?;
        Ok(octets)
    }

    /// Chiffrer les données et retourner le nonce concaténé au texte chiffré (nonce || ciphertext).
    pub fn chiffrer(&self, clair: &[u8]) -> Result<Vec<u8>, VaultError> {
        let mut nonce_bytes = self.generer_nonce()?;
        // Préciser explicitement U12, pour indiquer la taille du nonce désormais requise depuis 0.11.
        let nonce: &Nonce<U12> = (&nonce_bytes).try_into()
            .map_err(|_| VaultError::EchecChiffrement)?;

        let chiffre = self
            .chiffreur
            .encrypt(nonce, clair)
            .map_err(|_| VaultError::EchecChiffrement)?;

        let mut resultat = Vec::with_capacity(TAILLE_NONCE + chiffre.len());
        resultat.extend_from_slice(&nonce_bytes);
        resultat.extend_from_slice(&chiffre);

        nonce_bytes.zeroize(); // effacer la copie locale du nonce, par discipline systématique
        Ok(resultat)
    }

    /// Extraire le nonce, puis déchiffrer et authentifier le reste du payload.
    pub fn dechiffrer(&self, payload: &[u8]) -> Result<Vec<u8>, VaultError> {
        if payload.len() < TAILLE_NONCE {
            return Err(VaultError::EchecDechiffrement);
        }
        let (nonce_bytes, ciphertext) = payload.split_at(TAILLE_NONCE);
        let nonce: &Nonce<U12> = nonce_bytes.try_into()
            .map_err(|_| VaultError::EchecDechiffrement)?;

        self.chiffreur
            .decrypt(nonce, ciphertext)
            .map_err(|_| VaultError::EchecDechiffrement)
    }
}