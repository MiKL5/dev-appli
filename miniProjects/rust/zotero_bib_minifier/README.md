# **Zotero BibTeX Minifier**<img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg" align="right" height="64"></img>
  
<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white) 
![Cargo](https://img.shields.io/badge/Cargo-000000?style=flat&logo=rust&logoColor=white) 
![Platform](https://img.shields.io/badge/Plateforme-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey?style=flat)

</div>

Outil en ligne de commande nettoyant un fichier `.bib` exporté depuis [Zotero](https://www.zotero.org/) en supprimant les champs `abstract` et `keywords`, souvent volumineux et inutiles à la compilation en LaTeX.
---
Un export Zotero contient parfois des résumés d'articles de plusieurs centaines de mots par référence.  
Ce programme retire ces champs en lisant le fichier ligne par ligne, sans le charger entièrement en mémoire — utile pour de grosses bibliographies.
## **Installation**
```bash
git clone https://github.com/votre-utilisateur/zotero-bib-minifier.git
cd zotero-bib-minifier
cargo build --release
```
<!--L'exécutable sera disponible dans `target/release/zotero-bib-minifier`.-->
## **Utilisation**
```bash
# Avec les chemins par défaut (export_zotero_brut.bib -> export_zotero_minifie.bib)
cargo run --release

# Avec des chemins personnalisés
cargo run --release -- mon_export.bib mon_export_propre.bib
```

<details>
<summary>Exemple</summary>

Fichier d'entrée (`export_zotero_brut.bib`) :
```bibtex
@article{dupont2025,
  title={Une étude passionnante},
  abstract={Ceci est un très long résumé qui ne sert à rien dans LaTeX...},
  keywords={ia, data, recherche},
  year={2025}
}
```
Fichier de sortie (`export_zotero_minifie.bib`) :
```bibtex
@article{dupont2025,
  title={Une étude passionnante},
  year={2025}
}
```

</details>