use crate::domain::{LogEvent, RustLakeError};
use sha2::{Digest, Sha256};


/// Représenter un pseudonymiseur appliquant un hachage salé aux champs sensibles.
///
/// Conserver le sel en mémoire (et non codé en dur) permet de faire varier
/// la pseudonymisation entre exécutions, limitant les risques de corrélation
/// externe conformément au principe de minimisation du RGPD (article 5.1.c).
pub struct Pseudonymizer {
    salt: String,
}

impl Pseudonymizer {
    /// Construire un nouveau pseudonymiseur à partir d'un sel fourni explicitement.
    ///
    /// Refuser un sel vide au niveau du constructeur afin de garantir à l'appelant
    /// qu'aucune instance invalide ne pourra circuler dans le reste du programme.
    pub fn new(salt: impl Into<String>) -> Result<Self, RustLakeError> {
        let salt = salt.into();
        if salt.is_empty() {
            return Err(RustLakeError::MissingSensitiveField(
                "le sel de hachage ne peut pas être vide".to_string(),
            ));
        }
        Ok(Pseudonymizer { salt })
    }

    /// Hacher une valeur sensible avec le sel interne afin de produire un pseudonyme stable.
    fn hash_value(&self, value: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.salt.as_bytes());
        hasher.update(value.as_bytes());
        let digest = hasher.finalize();
        format!("{:x}", digest)[..16].to_string()
    }

    /// Anonymiser un événement de log en remplaçant l'adresse IP et l'identifiant utilisateur
    /// par leurs pseudonymes respectifs, tout en préservant les champs statistiques utiles.
    pub fn anonymize(&self, mut event: LogEvent) -> LogEvent {
        event.ip_address = self.hash_value(&event.ip_address);
        event.user_id = self.hash_value(&event.user_id);
        event
    }

    /// Anonymiser un lot complet d'événements en une seule passe.
    pub fn anonymize_batch(&self, events: Vec<LogEvent>) -> Vec<LogEvent> {
        events.into_iter().map(|event| self.anonymize(event)).collect()
    }
}