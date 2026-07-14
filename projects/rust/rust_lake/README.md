# **RustLake**<a href="../"><img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg" alt="Langage Rusgt" align="right" height="64px"></a>
# **Moteur ETL pédagogique en Rust pour journaux applicatifs**

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=rounded&logo=rust&logoColor=white) 
![Cargo](https://img.shields.io/badge/Cargo-DE4816?style=rounded&logo=rust&logoColor=white) 
![Serde](https://img.shields.io/badge/Serde-9B59B6?style=rounded) 
![Thiserror](https://img.shields.io/badge/thiserror-4B8BBE?style=rounded) 
![Anyhow](https://img.shields.io/badge/anyhow-2C3E50?style=roundede) 
![Rayon](https://img.shields.io/badge/Rayon-F39C12?style=rounded) 
![Clap](https://img.shields.io/badge/Clap-16A085?style=rounded) 
![RGPD](https://img.shields.io/badge/RGPD-Conformit%C3%A9-27AE60?style=rounded) 
![License MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=rounded)

</div>

RustLake est un projet pédagogique destiné à consolider les fondamentaux du langage Rust
(types génériques, opérateurs, contrôles de flux, gestion de la mémoire et sécurité, gestion
des erreurs) à travers un cas d'usage réaliste de Big Data : l'ingestion, l'anonymisation et
l'agrégation de journaux applicatifs volumineux.
---
## 🏗️ **Architecture**
```sh
rustlake/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── domain/          # LogEvent, Severity, RustLakeError
│   ├── parsing/         # trait Parsable<T>, CsvLogParser
│   ├── anonymization/   # Pseudonymizer (hachage salé RGPD)
│   ├── aggregation/     # Aggregator<T: Numeric>, Metrics
│   ├── concurrency/     # ParallelProcessor (rayon + Arc<Mutex<T>>)
│   └── export/          # ExecutionReport (JSON)
└── tests/
    └── integration_tests.rs
```

<details>
<summary>📚 Notions Rust illustrées par module</summary>

Module | Notion
---|---
`aggregation::numeric` | Types génériques bornés par trait (`T: Numeric`)
`aggregation::metrics` | Surcharge de l'opérateur `Add`
`domain::log_event` | Contrôle de flux par `match` exhaustif
`concurrency::parallel_processor` | Sécurité mémoire via `Arc<Mutex<T>>`, absence d'`unsafe`
`domain::errors` | Gestion typée des erreurs via `thiserror`
`main.rs` | Contextualisation des erreurs via `anyhow::Context`

</details>

## 🚀 **Utilisation**
```bash
cargo build --release
cargo run -- --input data/logs.csv --output output/report.json --salt mon-sel-secret
cargo test
```
## 🔒 **Conformité RGPD**

<details>
<summary>Détail des mesures de conformité implémentées</summary>

* Pseudonymisation systématique des adresses IP et identifiants utilisateurs par hachage salé (SHA-256) dès l'ingestion (module `anonymization`).
* Minimisation des données conservées en clair, conformément à l'article 5.1.c du RGPD.
* Traçabilité complète de chaque exécution via le rapport JSON généré (`export::ExecutionReport`).

</details>

## 📄 **Licence**

Distribué sous licence MIT. Voir le fichier [`LICENSE`](license) pour le détail.

<div align="center">

[Haut](#rustlake) · [License](license)