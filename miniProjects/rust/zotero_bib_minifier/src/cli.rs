/*
Définition des arguments en ligne de commande.
la crate `clap` (avec sa macro `derive`) au lieu de parser `std::env::args()`
à la main : évitant de réinventer la gestion des erreurs de saisie,
du `--help` automatique, et des valeurs par défaut.
*/

use clap::Parser;
use std::path::PathBuf;

/// Nettoie un export BibTeX de Zotero en retirant certains champs
#[derive(Parser, Debug)]
#[command(name = "zotero-bib-minifier", version, about)]
pub struct Cli {
    /// Chemin du fichier .bib source (l'export brut de Zotero)
    #[arg(short = 'e', long, default_value = "export_zotero_brut.bib")]
    pub entree: PathBuf,

    /// Chemin du fichier .bib de sortie (nettoyé)
    #[arg(short = 's', long, default_value = "export_zotero_minifie.bib")]
    pub sortie: PathBuf,

    /// Champs à exclure, séparés par des virgules
    #[arg(
        short = 'c',
        long,
        value_delimiter = ',',
        default_value = "abstract,keywords"
    )]
    pub champs: Vec<String>,

    /// Mode simulation : affiche ce qui serait supprimé sans écrire le fichier de sortie
    #[arg(short = 'd', long)]
    pub dry_run: bool,
}
