use serde::{Deserialize, Serialize};


/// Représenter les niveaux de sévérité possibles d'un événement de log.
///
/// Dériver PartialEq, Eq et Hash permet d'utiliser cette énumération comme clé
/// de regroupement dans une table de hachage lors de l'agrégation par catégorie.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

impl Severity {
    /// Convertir une chaîne brute issue du fichier source en variante typée.
    ///
    /// Retourner une erreur explicite plutôt qu'une valeur par défaut silencieuse
    /// afin de préserver la traçabilité des données malformées.
    pub fn parse(raw: &str) -> Result<Self, String> {
        match raw.trim().to_lowercase().as_str() {
            "info" => Ok(Severity::Info),
            "warning" | "warn" => Ok(Severity::Warning),
            "error" => Ok(Severity::Error),
            "critical" | "crit" => Ok(Severity::Critical),
            other => Err(format!("Sévérité inconnue : {other}")),
        }
    }

    /// Déterminer la pondération d'alerte associée à une sévérité donnée.
    ///
    /// Illustrer un contrôle de flux par correspondance exhaustive : toute
    /// nouvelle variante ajoutée à l'énumération provoquera une erreur de
    /// compilation ici, empêchant les oublis silencieux.
    pub fn alert_weight(&self) -> u8 {
        match self {
            Severity::Info => 0,
            Severity::Warning => 1,
            Severity::Error => 3,
            Severity::Critical => 5,
        }
    }
}

/// Représenter un événement de log unique après analyse syntaxique.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEvent {
    pub timestamp: String,
    pub ip_address: String,
    pub user_id: String,
    pub status_code: u16,
    pub latency_ms: u64,
    pub severity: Severity,
}

impl LogEvent {
    /// Construire un nouvel événement de log à partir de champs déjà validés.
    pub fn new(
        timestamp: String,
        ip_address: String,
        user_id: String,
        status_code: u16,
        latency_ms: u64,
        severity: Severity,
    ) -> Self {
        LogEvent {
            timestamp,
            ip_address,
            user_id,
            status_code,
            latency_ms,
            severity,
        }
    }

    /// Vérifier si l'événement correspond à une erreur applicative (code HTTP >= 400).
    pub fn is_error(&self) -> bool {
        self.status_code >= 400
    }
}
