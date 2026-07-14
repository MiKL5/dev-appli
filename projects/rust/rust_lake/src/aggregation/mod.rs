/// Déclarer les sous-modules responsables de l'agrégation statistique.
pub mod numeric;
pub mod generic_aggregator;
pub mod metrics;

pub use generic_aggregator::Aggregator;
pub use metrics::Metrics;
pub use numeric::Numeric;