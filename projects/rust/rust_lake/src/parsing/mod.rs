/// Déclarer les sous-modules responsables de l'analyse syntaxique des données brutes.
pub mod traits;
pub mod csv_parser;

pub use csv_parser::{parse_batch, CsvLogParser};
pub use traits::Parsable;
