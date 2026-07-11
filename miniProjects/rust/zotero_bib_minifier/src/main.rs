/*
Minificateur d'export Zotero (format BibTeX)

Ce petit programme lit un fichier .bib exporté depuis Zotero et retire
les lignes "abstract" et "keywords", qui alourdissent souvent le fichier
sans servir à la compilation LaTeX/BibTeX.

Pourquoi lire en flux (ligne par ligne) au lieu de tout charger d'un coup ?
-> Si le fichier .bib fait plusieurs Mo (bibliothèque avec beaucoup de
résumés), charger tout en mémoire avec `read_to_string` occupe de la
RAM inutilement. Lire ligne par ligne via un BufReader garde une
empreinte mémoire quasi constante, peu importe la taille du fichier.
*/

mod cli;
mod minifier;

use clap::Parser;
use cli::Cli;
use minifier::minifier_bibtex;
// use std::env;
// use std::fs::File;
// use std::io::{self, BufRead, BufReader, Write};
// use std::path::Path;
use std::process::ExitCode;

/// Point d'entrée du programme.
///
/// Utiliser `ExitCode` pour ne pas faire planter le programme (`panic!`)
/// en cas d'erreur : il envoie un code de sortie propre (0 = ok, 1 = erreur)
/// que d'autres scripts ou outils pourront exploiter, en affichant un message clair à l'utilisateur.
fn main() -> ExitCode {
    let cli = Cli::parse();

    let resultat = minifier_bibtex(&cli.entree, &cli.sortie, &cli.champs, cli.dry_run);
    match resultat {
        Ok(rapport) => {
            afficher_rapport(&rapport, &cli);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Erreur : {e}");
            ExitCode::FAILURE
        }
    }
}

// --- AJOUTÉ : fonction d'affichage extraite, avec le champ manquant ---
fn afficher_rapport(rapport: &minifier::RapportMinification, cli: &Cli) {
    if cli.dry_run {
        println!("[Simulation - aucun fichier écrit]");
    }
    println!(
        "Lignes lues : {} | Lignes supprimées : {}",
        rapport.lignes_totales, rapport.lignes_supprimees
    );
    if rapport.encodage_de_repli_utilise {
        println!("Encodage de repli utilisé : oui (Windows-1252 détecté)");
    }
    if rapport.champs_supprimes.is_empty() {
        println!("Aucun champ correspondant trouvé.");
    } else {
        println!("Champs supprimés :");
        for (nom, ligne) in &rapport.champs_supprimes {
            println!("  - '{}' (ligne {})", nom, ligne);
        }
    }
    if !cli.dry_run {
        println!("Fichier nettoyé écrit dans : {}", cli.sortie.display());
    }
} 
