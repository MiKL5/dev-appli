use rustlake::aggregation::{Aggregator, Metrics};
use rust_lake::anonymization::Pseudonymizer;
use rust_lake::domain::{LogEvent, Severity};
use rust_lake::parsing::parse_batch;


/// Vérifier que le parsing d'une ligne valide produit un événement correctement typé.
#[test]
fn parser_doit_accepter_une_ligne_valide() {
    let lines = vec!["2026-07-12T10:00:00,192.168.1.1,user42,200,120,info".to_string()];
    let (valid_events, errors) = parse_batch(&lines);

    assert_eq!(valid_events.len(), 1);
    assert_eq!(errors.len(), 0);
    assert_eq!(valid_events[0].status_code, 200);
}

/// Vérifier que le parsing d'une ligne malformée produit une erreur explicite plutôt qu'un panic.
#[test]
fn parser_doit_rejeter_une_ligne_malformee() {
    let lines = vec!["ligne,incomplete".to_string()];
    let (valid_events, errors) = parse_batch(&lines);

    assert_eq!(valid_events.len(), 0);
    assert_eq!(errors.len(), 1);
}

/// Vérifier que l'agrégateur générique calcule correctement le total et le maximum.
#[test]
fn aggregateur_generique_doit_calculer_total_et_max() {
    let mut aggregator: Aggregator<u64> = Aggregator::new();
    aggregator.add(10);
    aggregator.add(25);
    aggregator.add(5);

    assert_eq!(aggregator.total, 40);
    assert_eq!(aggregator.max, 25);
    assert_eq!(aggregator.count, 3);
}

/// Vérifier que la surcharge de l'opérateur Add fusionne correctement deux jeux de métriques.
#[test]
fn metrics_doit_se_fusionner_via_operateur_add() {
    let metrics_a = Metrics { error_count: 2, total_latency_ms: 100, event_count: 5 };
    let metrics_b = Metrics { error_count: 3, total_latency_ms: 150, event_count: 5 };

    let combined = metrics_a + metrics_b;

    assert_eq!(combined.error_count, 5);
    assert_eq!(combined.total_latency_ms, 250);
    assert_eq!(combined.event_count, 10);
}

/// Vérifier que la pseudonymisation transforme effectivement les champs sensibles.
#[test]
fn pseudonymiseur_doit_masquer_les_champs_sensibles() {
    let pseudonymizer = Pseudonymizer::new("sel-de-test").expect("Créer le pseudonymiseur");
    let event = LogEvent::new(
        "2026-07-12T10:00:00".to_string(),
        "192.168.1.1".to_string(),
        "user42".to_string(),
        200,
        120,
        Severity::Info,
    );

    let anonymized = pseudonymizer.anonymize(event);

    assert_ne!(anonymized.ip_address, "192.168.1.1");
    assert_ne!(anonymized.user_id, "user42");
}