use crate::crypto::MoteurCrypto;
use crate::erreurs::VaultError;
use std::marker::PhantomData;
use zeroize::{Zeroize, Zeroizing};


pub trait Chiffrable {
    fn vers_octets(&self) -> Vec<u8>;
    fn depuis_octets(donnees: &[u8]) -> Self;
}

impl Chiffrable for String {
    fn vers_octets(&self) -> Vec<u8> { self.as_bytes().to_vec() }
    fn depuis_octets(donnees: &[u8]) -> Self { String::from_utf8_lossy(donnees).into_owned() }
}

pub struct Coffre<T: Chiffrable> {
    secrets: Vec<(String, Vec<u8>)>,
    moteur: MoteurCrypto,
    _marker: PhantomData<T>,
}

impl<T: Chiffrable> Coffre<T> {
    /// Accepter la clé sous forme Zeroizing pour garantir son effacement même en cas d'échec d'initialisation.
    pub fn nouveau(cle: Zeroizing<Vec<u8>>) -> Result<Self, VaultError> {
        Ok(Coffre {
            secrets: Vec::new(),
            moteur: MoteurCrypto::nouveau(&cle)?, // la clé s'efface automatiquement à la fin de cette fonction
            _marker: PhantomData,
        })
    }

    pub fn stocker(&mut self, identifiant: &str, valeur: &T) -> Result<(), VaultError> {
        let mut payload = valeur.vers_octets(); // représentation en clair temporaire, sensible
        let chiffre = self.moteur.chiffrer(&payload)?;
        payload.zeroize();
        self.secrets.push((identifiant.to_string(), chiffre));
        Ok(())
    }

    pub fn recuperer(&self, identifiant: &str) -> Result<T, VaultError> {
        for (id, payload) in &self.secrets {
            if id == identifiant {
                let clair = self.moteur.dechiffrer(payload)?;
                let resultat = T::depuis_octets(&clair);
                return Ok(resultat);
            }
        }
        Err(VaultError::SecretIntrouvable(identifiant.to_string()))
    }
}