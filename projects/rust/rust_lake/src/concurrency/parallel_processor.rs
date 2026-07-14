use crate::aggregation::Metrics;
use crate::domain::{LogEvent, RustLakeError};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};


/// Orchestrer le traitement parallèle d'un lot d'événements de log en toute sécurité mémoire.
///
/// Le compilateur Rust interdit toute course de données (data race) à la compilation
/// grâce aux traits Send et Sync : Arc<Mutex<T>> est le seul moyen sûr de partager
/// un état mutable entre plusieurs threads gérés par rayon. Aucun bloc unsafe
/// n'est requis pour atteindre cette garantie.
pub struct ParallelProcessor;

impl ParallelProcessor {
    /// Agréger en parallèle un lot d'événements en répartissant le travail
    /// automatiquement sur les cœurs disponibles.
    ///
    /// Chaque thread accumule ses propres métriques locales avant de les fusionner
    /// dans l'état partagé, limitant ainsi la contention sur le verrou Mutex.
    pub fn aggregate(events: &[LogEvent]) -> Result<Metrics, RustLakeError> {
        let shared_metrics = Arc::new(Mutex::new(Metrics::default()));

        events.par_chunks(1000).for_each(|chunk| {
            let mut local_metrics = Metrics::default();

            for event in chunk {
                local_metrics.event_count += 1;
                local_metrics.total_latency_ms += event.latency_ms;
                if event.is_error() {
                    local_metrics.error_count += 1;
                }
            }

            let mut guard = shared_metrics
                .lock()
                .expect("Verrouiller le mutex partagé sans contention bloquante persistante");
            *guard = *guard + local_metrics;
        });

        let final_metrics = *shared_metrics
            .lock()
            .map_err(|_| RustLakeError::LockFailure("échec du verrouillage final".to_string()))?;

        Ok(final_metrics)
    }
}