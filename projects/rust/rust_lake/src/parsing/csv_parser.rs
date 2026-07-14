use crate::domain::{LogEvent, RustLakeError, Severity};
use crate::parsing::traits::Parsable;


/// Implémenter un analyseur syntaxique dédié au format CSV des journaux applicatifs.
///
/// Le format attendu par colonne est : horodatage, adresse IP, identifiant utilisateur,
/// code de statut HTTP, latence en millisecondes, sévérité.
pub struct CsvLogParser;

impl Parsable<LogEvent> for CsvLogParser {
    /// Analyser une ligne CSV unique en tolérant les erreurs sans interrompre le flux global.
    ///
    /// Chaque échec de conversion est capturé et transformé en variante ParseFailure
    /// afin de préserver le contexte (numéro de ligne, raison précise) pour le diagnostic.
    fn parse_line(&self, raw_line: &str, line_number: usize) -> Result<LogEvent, RustLakeError> {
        let fields: Vec<&str> = raw_line.split(',').map(|f| f.trim()).collect();

        if fields.len() != 6 {
            return Err(RustLakeError::ParseFailure {
                line: line_number,
                reason: format!("nombre de colonnes invalide : {}", fields.len()),
            });
        }

        let status_code: u16 = fields[3].parse().map_err(|_| RustLakeError::ParseFailure {
            line: line_number,
            reason: format!("code de statut invalide : {}", fields[3]),
        })?;

        let latency_ms: u64 = fields[4].parse().map_err(|_| RustLakeError::ParseFailure {
            line: line_number,
            reason: format!("latence invalide : {}", fields[4]),
        })?;

        let severity = Severity::parse(fields[5]).map_err(|reason| RustLakeError::ParseFailure {
            line: line_number,
            reason,
        })?;

        Ok(LogEvent::new(
            fields[0].to_string(),
            fields[1].to_string(),
            fields[2].to_string(),
            status_code,
            latency_ms,
            severity,
        ))
    }
}

/// Analyser un lot complet de lignes brutes en séparant les succès des échecs.
///
/// Retourner un couple (événements valides, erreurs rencontrées) permet au niveau
/// applicatif de décider s'il faut interrompre le traitement ou poursuivre en dégradé,
/// plutôt que d'imposer un comportement figé dans la couche de parsing.
pub fn parse_batch(lines: &[String]) -> (Vec<LogEvent>, Vec<RustLakeError>) {
    let parser = CsvLogParser;
    let mut valid_events = Vec::with_capacity(lines.len());
    let mut collected_errors = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        match parser.parse_line(line, index + 1) {
            Ok(event) => valid_events.push(event),
            Err(error) => collected_errors.push(error),
        }
    }

    (valid_events, collected_errors)
}