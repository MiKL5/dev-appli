use keyring::Entry;
use zeroize::{Zeroize, Zeroizing};
use crate::erreurs::VaultError;


const NOM_SERVICE: &str = "sentinelvault"; // Nommer le service pour identifier notre application dans le trousseau (visible par l'utilisateur).
const NOM_COMPTE: &str = "cle-maitresse"; // Nommer le compte pour distinguer plusieurs coffres si nécessaire (ici un seul suffit).
const TAILLE_CLE: usize = 32; // 32 octets exigés pour AES-256
const VARIABLE_ENV_REPLI: &str = "SENTINELVAULT_CLE_HEX"; // repli pour environnements headless

/// Ouvrir une entrée nommée dans le trousseau natif du système d'exploitation.
fn ouvrir_entree() -> Result<Entry, VaultError> {
    Entry::new(NOM_SERVICE, NOM_COMPTE)
        .map_err(|e| VaultError::TrousseauInaccessible(e.to_string()))
}

/// Encoder des octets bruts en chaîne hexadécimale, car keyring stocke des chaînes de texte.
fn octets_vers_hex(octets: &[u8]) -> String {
    octets.iter().map(|o| format!("{o:02x}")).collect()
}

/// Décoder une chaîne hexadécimale en octets bruts, en validant chaque paire de caractères.
fn hex_vers_octets(hex: &str) -> Result<Vec<u8>, VaultError> {
    if hex.len() % 2 != 0 {
        return Err(VaultError::TrousseauInaccessible("format hexadécimal invalide".into()));
    }
    let mut resultat = Vec::with_capacity(hex.len() / 2);
    let mut caracteres = hex.chars();
    // Consommer les caractères deux par deux via une boucle while explicite.
    while let (Some(a), Some(b)) = (caracteres.next(), caracteres.next()) {
        let octet_str: String = [a, b].iter().collect();
        let octet = u8::from_str_radix(&octet_str, 16)
            .map_err(|_| VaultError::TrousseauInaccessible("caractère hexadécimal invalide".into()))?;
        resultat.push(octet);
    }
    Ok(resultat)
}

/// Générer une clé aléatoire cryptographiquement sûre, puis l'enregistrer dans le trousseau.
pub fn generer_et_stocker_cle() -> Result<Vec<u8>, VaultError> {
    let mut cle = vec![0u8; TAILLE_CLE];
    // Puiser directement dans l'entropie du système d'exploitation, sans intermédiaire versionné.
    getrandom::fill(&mut cle).map_err(|_| VaultError::TrousseauInaccessible("échec du générateur aléatoire système".into()))?;

    let entree = ouvrir_entree()?;
    let mut cle_hex = octets_vers_hex(&cle); // représentation intermédiaire, elle aussi sensible
    entree
        .set_password(&cle_hex)
        .map_err(|e| VaultError::TrousseauInaccessible(e.to_string()))?;

    cle_hex.zeroize(); // effacer la chaîne hexadécimale dès qu'elle n'est plus nécessaire
    Ok(cle)
}

/// Récupérer la clé existante depuis le trousseau, sans jamais la faire transiter par un fichier.
pub fn recuperer_cle() -> Result<Vec<u8>, VaultError> {
    let entree = ouvrir_entree()?;
    match entree.get_password() {
        Ok(hex) => {
            let hex_protege = Zeroizing::new(hex); // s'auto-efface à la sortie de portée, même en cas d'erreur
            hex_vers_octets(&hex_protege)
        }
        Err(keyring::Error::NoEntry) => Err(VaultError::CleAbsenteDuTrousseau),
        Err(e) => Err(VaultError::TrousseauInaccessible(e.to_string())),
    }
}

/// Détecter si le trousseau système est réellement joignable avant toute opération.
fn trousseau_disponible() -> bool {
    match Entry::new(NOM_SERVICE, NOM_COMPTE) {
        Ok(entree) => {
            // Tenter une lecture à blanc pour vérifier la joignabilité effective du démon.
            match entree.get_password() {
                Ok(_) => true,
                Err(keyring::Error::NoEntry) => true, // le service répond, la clé n'existe simplement pas encore
                Err(_) => false, // le démon est injoignable (headless, D-Bus absent, etc.)
            }
        }
        Err(_) => false,
    }
}

/// Lire la clé depuis une variable d'environnement, solution de repli pour serveurs sans trousseau.
fn recuperer_cle_via_env() -> Result<Vec<u8>, VaultError> {
    let hex = Zeroizing::new(
        std::env::var(VARIABLE_ENV_REPLI).map_err(|_| VaultError::CleAbsenteDuTrousseau)?,
    );
    hex_vers_octets(&hex)
}

/// Orchestrer une hiérarchie de stratégies : trousseau natif, puis variable d'environnement.
pub fn obtenir_ou_creer_cle() -> Result<Vec<u8>, VaultError> {
    if trousseau_disponible() {
        println!("[INFO] Trousseau système détecté, utilisation du stockage natif.");
        return match recuperer_cle() {
            Ok(cle) => Ok(cle),
            Err(VaultError::CleAbsenteDuTrousseau) => generer_et_stocker_cle(),
            Err(autre) => Err(autre),
        };
    }

    println!("[AVERTISSEMENT] Trousseau indisponible (environnement headless probable).");
    println!("[AVERTISSEMENT] Repli sur la variable d'environnement {VARIABLE_ENV_REPLI}.");
    recuperer_cle_via_env()
}

/// Supprimer la clé du trousseau, utile pour les procédures de rotation ou de désinstallation.
#[allow(dead_code)] // réservée à la future procédure de rotation des clés
pub fn effacer_cle() -> Result<(), VaultError> {
    let entree = ouvrir_entree()?;
    entree
        .delete_credential()
        .map_err(|e| VaultError::TrousseauInaccessible(e.to_string()))
}