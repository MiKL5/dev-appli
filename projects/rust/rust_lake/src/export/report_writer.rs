use crate::aggregation::Metrics;
use crate::domain::RustLakeError;
use serde::Serialize;
use std::fs::File;
use std::io::Write;


/// Représenter un rapport d'exécution sérialisable en JSON.
#[derive(Debug, Serialize)]
pub struct ExecutionReport {
    pub lines_processed: usize,
    pub lines_rejected: usize,
    pub duration_ms: u128,
    pub metrics: Metrics,
}

impl ExecutionReport {
    /// Construire un nouveau rapport d'exécution à partir des résultats du pipeline.
    pub fn new(
        lines_processed: usize,
        lines_rejected: usize,
        duration_ms: u128,
        metrics: Metrics,
    ) -> Self {
        ExecutionReport {
            lines_processed,
            lines_rejected,
            duration_ms,
            metrics,
        }
    }

    /// Écrire le rapport au format JSON dans le fichier de destination indiqué.
    ///
    /// Propager l'erreur d'écriture via l'opérateur ? plutôt que de l'ignorer
    /// garantit qu'aucun échec de sauvegarde ne passe inaperçu.
    pub fn write_to_file(&self, path: &str) -> Result<(), RustLakeError> {
        let serialized = serde_json::to_string_pretty(self)
            .map_err(|e| RustLakeError::Unexpected(e.to_string()))?;
        let mut file = File::create(path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}
