use anyhow::{Context, Result};
use clap::Parser;
use rust_lake::anonymization::Pseudonymizer;
use rust_lake::concurrency::ParallelProcessor;
use rust_lake::export::ExecutionReport;
use rust_lake::parsing::parse_batch;
use std::fs;
use std::time::Instant;


/// Définir les arguments acceptés par la ligne de commande de RustLake.
///
/// L'attribut derive(Parser) de clap illustre l'usage de macros génériques
/// pour éliminer le code répétitif de parsing d'arguments.
#[derive(Parser, Debug)]
#[command(name = "rustlake")]
#[command(about = "Moteur ETL pédagogique en Rust pour journaux applicatifs")]
struct Cli {
    /// Chemin vers le fichier CSV source contenant les logs bruts.
    #[arg(short, long)]
    input: String,

    /// Chemin de sortie du rapport JSON généré.
    #[arg(short, long, default_value = "output/report.json")]
    output: String,

    /// Sel utilisé pour la pseudonymisation des champs sensibles.
    #[arg(short, long, default_value = "rustlake-default-salt")]
    salt: String,
}

/// Constituer le point d'entrée principal du binaire.
///
/// anyhow::Context enrichit chaque erreur remontée d'une information
/// contextuelle exploitable par l'utilisateur final, conformément à la
/// convention thiserror (bibliothèque) / anyhow (application).
fn main() -> Result<()> {
    let cli = Cli::parse();
    let start = Instant::now();

    let raw_content = fs::read_to_string(&cli.input)
        .with_context(|| format!("Impossible de lire le fichier d'entrée : {}", cli.input))?;

    let lines: Vec<String> = raw_content.lines().map(|l| l.to_string()).collect();

    let (valid_events, parse_errors) = parse_batch(&lines);
    println!(
        "Lignes valides : {} — Lignes rejetées : {}",
        valid_events.len(),
        parse_errors.len()
    );

    let pseudonymizer = Pseudonymizer::new(cli.salt.clone())
        .context("Impossible d'initialiser le pseudonymiseur RGPD")?;
    let anonymized_events = pseudonymizer.anonymize_batch(valid_events);

    let metrics = ParallelProcessor::aggregate(&anonymized_events)
        .context("Échec de l'agrégation parallèle des événements")?;

    let duration_ms = start.elapsed().as_millis();
    let report = ExecutionReport::new(
        anonymized_events.len(),
        parse_errors.len(),
        duration_ms,
        metrics,
    );

    fs::create_dir_all("output").context("Impossible de créer le dossier de sortie")?;
    report
        .write_to_file(&cli.output)
        .context("Impossible d'écrire le rapport final")?;

    println!("Rapport écrit dans : {}", cli.output);
    println!("Durée totale de traitement : {} ms", duration_ms);

    Ok(())
}
