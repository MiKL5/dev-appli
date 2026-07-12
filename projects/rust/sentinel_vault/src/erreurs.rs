use std::fmt;
use std::error::Error as StdError;


/// Représenter chaque catégorie de défaillance sans jamais recourir aux panics implicites.
#[derive(Debug)]
pub enum VaultError {
    SecretIntrouvable(String),
    ClePropreVide,
    ChiffrementInvalide { attendu: usize, obtenu: usize },
    EchecChiffrement,
    EchecDechiffrement,
    AccesRefuse(u8),
    TrousseauInaccessible(String),
    CleAbsenteDuTrousseau,
}

// Implémenter Display pour produire un message diagnostique lisible par l'utilisateur final.
impl fmt::Display for VaultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VaultError::SecretIntrouvable(id) => write!(f, "Secret '{id}' introuvable"),
            VaultError::ClePropreVide => write!(f, "Clé de chiffrement vide"),
            VaultError::ChiffrementInvalide { attendu, obtenu } => {
                write!(f, "Longueur de clé invalide : attendu {attendu} octets, obtenu {obtenu}")
            }
            VaultError::EchecChiffrement => write!(f, "Échec du chiffrement AES-GCM"),
            VaultError::EchecDechiffrement => write!(f, "Échec du déchiffrement : clé, nonce ou tag invalide"),
            VaultError::AccesRefuse(niveau) => write!(f, "L'accès est refusé : niveau {niveau} requis"),
            VaultError::TrousseauInaccessible(msg) => write!(f, "Le trousseau du système inaccessible car {msg}"),
            VaultError::CleAbsenteDuTrousseau => write!(f, "Aucune clé trouvée dans le trousseau, l'initialisation est requise"),
        }
    }
}

impl StdError for VaultError {}