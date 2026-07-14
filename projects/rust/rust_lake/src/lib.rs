//! RustLake — moteur ETL pédagogique en Rust pour l'analyse de journaux applicatifs.
//!
//! Ce crate est structuré en modules indépendants afin d'isoler chaque
//! responsabilité et de faciliter la lecture pédagogique du code source.

pub mod domain;
pub mod parsing;
pub mod anonymization;
pub mod aggregation;
pub mod concurrency;
pub mod export;