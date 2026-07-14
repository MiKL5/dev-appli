use std::ops::Add;


/// Représenter un jeu de métriques agrégées issues d'un lot de logs.
///
/// Dériver Copy et Clone permet de manipuler cette structure par valeur sans
/// coût de gestion mémoire significatif, illustrant un choix pertinent
/// entre copie et emprunt pour des types de petite taille fixe.
#[derive(Debug, Clone, Copy, Default)]
pub struct Metrics {
    pub error_count: u64,
    pub total_latency_ms: u64,
    pub event_count: u64,
}

impl Add for Metrics {
    type Output = Metrics;

    /// Surcharger l'opérateur + afin de fusionner deux jeux de métriques.
    ///
    /// Cette surcharge illustre concrètement le trait std::ops::Add et permet
    /// d'écrire ensuite `metrics_a + metrics_b` dans le code d'orchestration,
    /// à la manière d'une opération de type map-reduce.
    fn add(self, other: Metrics) -> Metrics {
        Metrics {
            error_count: self.error_count + other.error_count,
            total_latency_ms: self.total_latency_ms + other.total_latency_ms,
            event_count: self.event_count + other.event_count,
        }
    }
}

impl Metrics {
    /// Calculer la latence moyenne, en gérant explicitement le cas de division par zéro.
    ///
    /// Retourner un Option<f64> plutôt qu'un panic illustre une gestion défensive
    /// des erreurs arithmétiques sans recourir à des valeurs sentinelles ambiguës.
    pub fn average_latency(&self) -> Option<f64> {
        if self.event_count == 0 {
            None
        } else {
            Some(self.total_latency_ms as f64 / self.event_count as f64)
        }
    }
}