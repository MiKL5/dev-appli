# 🌀 Fibonacci — Calculateur Interactif

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.95-000000?style=flat&logo=rust&logoColor=white)

</div>

Calculateur interactif du n-ième terme de la suite de Fibonacci en Rust.  
Saisie utilisateur via stdin, avec **gestion de l'overflow** via `Option<u32>` et `checked_add`.
## Fonctionnalités
* Saisie du rang au runtime (stdin)
* Calcul itératif sans récursion — pas de stack overflow
* Détection automatique de l'overflow (`u32` saturé au rang 47)
* Retour `None` propre en cas de dépassement de capacité
## Utilisation
```bash
cargo run
```
**Exemple d'interaction :**
```
--- Calculateur de Fibonacci ---
👇 Veuillez entrer le rang (nombre entier) souhaité
10 ✅ Le terme au rang 10 est 55
```

**En cas de dépassement :**
```
⚠️ La limite est atteinte.
Le rang 48 dépasse la capacité du type u32 (Le rang maximum est 47).
```

## Limite connue

Type | Rang max | Valeur max
---|---|---
`u32` | 47 | 2 971 215 073
`u64` | 93 | 12 200 160 415 121 876 738

> Pour dépasser le rang 47, remplacer `u32` par `u64` dans la signature de `calculer_fibonacci`.