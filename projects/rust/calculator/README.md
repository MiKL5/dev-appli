# **Calculatrice Modulaire**<a href="https://github.com/MiKL5/dev-appli/"><img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg" alt="Langage Rusgt" align="right" height="64px"></a>

<div align="center">

![Rust 1.95](https://img.shields.io/badge/Rust-1.95-000000?style=flat-radius&logo=rust&logoColor=white) 
![Cargo](https://img.shields.io/badge/Cargo-Build%20%26%20Test-orange?style=flat&logo=rust&logoColor=white) 
![Opérateurs](https://img.shields.io/badge/Op%C3%A9rateurs-12-blue?style=flat)

</div>

Cette calculatrice CLI évalue une gamme étendue d'opérateurs (binaires et unaires) avec une gestion stricte des erreurs. Elle intègre un mécanisme de mémoire volatile (**MS/MR**) et un moteur de formatage intelligent pour corriger les micro-erreurs de précision des nombres flottants.
---

## 🛠️ Installation et exécution

<details>
<summary>⚙️ Compilation et lancement</summary>

### Prérequis

- **Rust** & **Cargo** — [Ici pour l'installation](https://rustup.rs/)

### Commandes

```bash
# Compilation
cargo build --release

# Lancement
cargo run

# Tests unitaires
cargo test
```

</details>

## 🔢 Les opérateurs supportés
### Les opérateurs binaires
Symbole | Opération | Exemple
---|---|---
`+` | Addition | `3 + 4 = 7`
`-` | Soustraction | `10 - 3 = 7`
`*` | Multiplication | `6 * 7 = 42`
`/` | Division réelle | `7 / 2 = 3.5`
`%` | Reste (Modulo) | `7 % 3 = 1`
`//` | Quotient entier | `7 // 2 = 3`
`^` ou `**` | Puissance | `2 ^ 8 = 256`
### Les opérateurs unaires
Symbole | Opération | Exemple
---|---|---
`sqrt` | Racine carrée | `sqrt(9) = 3`
`ln` | Logarithme naturel | `ln(1) = 0`
`cos` | Cosinus (radians) | `cos(0) = 1`
`sin` | Sinus (radians) | `sin(0) = 0`
`abs` | Valeur absolue | `abs(-5) = 5`
## 🧠 La mémoire MS/MR
* **MS (Memory Store)** : proposé automatiquement après chaque calcul valide.
* **MR (Memory Recall)** : tapez `MR` à la saisie d'un nombre pour injecter la valeur stockée.
* La mémoire est **volatile** (session uniquement) — aucune persistance sur disque.