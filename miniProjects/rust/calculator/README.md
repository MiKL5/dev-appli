# 🧮 Calculatrice Rust

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.95-000000?style=flat&logo=rust&logoColor=white)

</div>

Calculatrice interactive en ligne de commande, développée en Rust.  
Prend en charge 6 opérateurs arithmétiques, une **mémoire persistante (MS/MR)** par session, et un formatage intelligent des résultats pour corriger le bruit binaire IEEE 754.
## Fonctionnalités
* 6 opérateurs : `+` `-` `*` `/` `%` `//`
* Gestion de la **division par zéro** via `Result<f64, String>`
* **Mémoire de session** : stockage (MS) et rappel (MR) d'une valeur `Option<f64>`
* Formatage intelligent — suppression du bruit flottant IEEE 754 (arrondi à 10⁻¹²)
* Boucle de calcul persistante avec sortie explicite
## Opérateurs
Symbole | Opération
---|---
`+` | Addition
`-` | Soustraction
`*` | Multiplication
`/` | Division réelle
`%` | Division euclidienne
`//` | Quotient entier
## Utilisation
```bash
cargo run
```
**Exemple de session :**
```
--- Calculatrice Rust Haute Précision ---

--- Nouvelle session de calcul ---
Entrez le premier nombre (ou 'MR') 👉 10
Entrez le second nombre (ou 'MR') 👉 3
[+] Addition | [-] Soustraction | [*] Multiplication
[/] Division | [%] Division euclidienne | [//] Quotient Entier

👉 Quel opérateur voulez-vous ? /
✅ Résultat : 3.333333333333
Voulez-vous stocker ce résultat en mémoire (MS) ? (o/n) 👉 o
💾 Valeur 3.333333333333 sauvegardée.
```
## Architecture
Fonction | Rôle
---|---
`evaluer()` | Dispatch des opérations + gestion des erreurs
`formater_intelligent()` | Correction du bruit IEEE 754
`gerer_sauvegarde()` | Mise à jour de la mémoire (MS)
`saisir_nombre()` | Saisie + rappel mémoire (MR)
`choisir_operateur()` | Parsing du symbole vers l'enum `Operateur`