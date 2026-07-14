use thiserror::Error;


/// Représenter l'ensemble exhaustif des erreurs susceptibles de survenir dans le pipeline RustLake.
///
/// Distinguer chaque famille d'erreur permet au code appelant de réagir de manière ciblée
/// plutôt que de traiter toute défaillance comme une erreur générique indifférenciée.
#[derive(Error, Debug)]
pub enum RustLakeError {
    /// Convertir automatiquement une erreur d'entrée/sortie du système en erreur métier.
    #[error("Impossible d'accéder au fichier source : {0}")]
    IoFailure(#[from] std::io::Error),

    /// Convertir automatiquement une erreur du crate csv en erreur métier.
    #[error("Erreur de lecture CSV : {0}")]
    CsvFailure(#[from] csv::Error),

    /// Signaler qu'une ligne de log ne respecte pas le format attendu.
    #[error("Ligne {line} malformée : {reason}")]
    ParseFailure { line: usize, reason: String },

    /// Signaler l'absence d'un champ sensible requis pour l'anonymisation.
    #[error("Champ sensible manquant pour l'anonymisation : {0}")]
    MissingSensitiveField(String),

    /// Signaler un dépassement de capacité numérique lors d'une agrégation.
    #[error("Dépassement de capacité numérique lors de l'agrégation")]
    OverflowError,

    /// Signaler l'échec d'acquisition d'un verrou partagé entre threads.
    #[error("Impossible de verrouiller la ressource partagée : {0}")]
    LockFailure(String),

    /// Regrouper toute erreur non anticipée sous une variante générique traçable.
    #[error("Erreur inattendue : {0}")]
    Unexpected(String),
}

/// Définir un alias de type pour simplifier les signatures de fonction du crate.
pub type RustLakeResult<T> = Result<T, RustLakeError>;