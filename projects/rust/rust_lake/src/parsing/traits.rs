use crate::domain::RustLakeError;


/// Définir un contrat générique pour tout type capable de se construire à partir
/// d'une ligne de texte brute.
///
/// Introduire ce trait permet d'illustrer la généricité par contrat en Rust :
/// n'importe quelle future source de données (CSV, JSON Lines, flux réseau)
/// pourra implémenter Parsable<T> sans modifier le code appelant.
pub trait Parsable<T> {
    /// Analyser une ligne brute et produire soit la structure attendue, soit une erreur typée.
    fn parse_line(&self, raw_line: &str, line_number: usize) -> Result<T, RustLakeError>;
}