# 🛡️ SentinelVault<a href="../"><img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg" alt="Langage Rusgt" align="right" height="64px"></a>

**Gestionnaire de secrets sécurisé en Rust — Chiffrement AES-256-GCM & Trousseau Système**
---
<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=rounded&logo=rust&logoColor=white) 
![AES-256-GCM](https://img.shields.io/badge/AES--256--GCM-RustCrypto-2ea44f?style=rounded) 
![Keyring](https://img.shields.io/badge/Keyring-OS%20Native%20Store-orange?style=rounded) 
![Cross Platform](https://img.shields.io/badge/Cross--Platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey?style=rounded) 
![Memory Safety](https://img.shields.io/badge/Memory%20Safety-Guaranteed-brightgreen?style=rounded) 
![Zero Unsafe](https://img.shields.io/badge/unsafe-0%25-success?style=rounded) 
![License](https://img.shields.io/badge/License-MIT-blue?style=rounded) 
![Cargo](https://img.shields.io/badge/Cargo-DEA584?style=rounded&logo=rust&logoColor=black) 
![RGPD](https://img.shields.io/badge/RGPD-Compliant-purple?style=rouned)

</div>

---

## 🎯 **Présentation**
**SentinelVault** est une application pédagogique en ligne de commande écrite en Rust, conçue pour illustrer un coffre-fort de secrets applicatif. Elle chiffre chaque secret via **AES-256-GCM** (algorithme authentifié de la famille AEAD) et délègue la gestion de la clé maîtresse au **trousseau natif du système d'exploitation** (Keychain macOS, Credential Manager Windows, Secret Service Linux), garantissant qu'aucune clé sensible ne transite jamais en clair sur le disque ni dans le code source versionné.

<details>
<summary>🎓 Pourquoi Rust pour un projet de sécurité ? (cliquer pour développer)</summary>

Rust garantit, dès la compilation, l'absence de classes entières de vulnérabilités mémoire (buffer overflow, use-after-free, data races) grâce à son système d'ownership et de borrowing vérifié statiquement par le compilateur, sans recourir à un garbage collector à l'exécution. Cette propriété en fait un choix privilégié pour les applications manipulant des données sensibles, où une faille mémoire pourrait exposer des secrets en clair.

</details>

## 🎓 **Objectifs pédagogiques**

Pilier Rust | Illustration dans le projet
---|---
Types génériques | Structure `Coffre<T: Chiffrable>` paramétrée par un trait borné
Opérateurs | Opérateurs bit à bit (`^`), logiques (`&&`, `\|\|`, `!`), arithmétiques et de comparaison
Contrôles de flux | `match` exhaustif, `for`, `while`, `while let`, `if let`
Gestion mémoire & sécurité | Ownership, borrowing, trait `Drop` pour l'effacement mémoire, zéro `unsafe`
Gestion des erreurs | Enum `VaultError` exhaustif, `Result<T, E>`, opérateur `?`, conversions via `map_err`
## 🏗️ **Architecture du projet**

```
sentinelvault/
├── Cargo.toml
├── README.md
├── SECURITY.md
└── src/
    ├── main.rs             # Point d'entrée et orchestration
    ├── erreurs.rs          # Taxonomie exhaustive des erreurs (VaultError)
    ├── crypto.rs           # Moteur de chiffrement AES-256-GCM
    ├── coffre.rs           # Structure générique Coffre<T> et trait Chiffrable
    ├── controle_acces.rs   # Vérification des habilitations
    └── trousseau.rs        # Interface avec le trousseau système (keyring)
```

<details>
<summary>📦 Détail des responsabilités de chaque module (cliquer pour développer)</summary>

Module | Responsabilité unique
---|---
`erreurs.rs` | Centraliser toutes les erreurs possibles sous un type unique `VaultError`
`crypto.rs` | Encapsuler exclusivement la logique AES-256-GCM (nonce, chiffrement, déchiffrement)
`coffre.rs` | Gérer le stockage en mémoire des secrets chiffrés, indépendamment du type stocké
`controle_acces.rs` | Appliquer la logique d'habilitation utilisateur (RBAC minimal)
`trousseau.rs` | Isoler toute dépendance au système d'exploitation pour la gestion de la clé maîtresse

</details>


## ⚙️ **Prérequis**
* Rust ≥ 1.85 (édition 2024) — [installation via rustup](https://www.rust-lang.org/tools/install)
* Cargo (fourni avec Rust)

<details>
<summary>🐧 Prérequis spécifiques à Linux (cliquer pour développer)</summary>

Le module `trousseau.rs` requiert un fournisseur **Secret Service** actif via D-Bus (spécification freedesktop.org). Sur un poste de bureau GNOME ou KDE, ce service est démarré automatiquement (`gnome-keyring-daemon` ou KWallet). Sur un **serveur headless sans session graphique**, ce démon est absent : l'application bascule alors automatiquement sur la variable d'environnement `SENTINELVAULT_CLE_HEX` comme mécanisme de repli documenté dans `SECURITY.md`.
```bash
# Installer gnome-keyring sur une distribution Debian/Ubuntu si nécessaire
sudo apt install gnome-keyring dbus-x11
```

</details>

<details>
<summary>🪟 Prérequis Windows / 🍎 macOS (cliquer pour développer)</summary>

Aucune installation supplémentaire n'est requise : Windows utilise nativement le **Credential Manager**, et macOS utilise nativement le **Keychain**, tous deux accessibles directement par le crate `keyring`.

</details>

## 🚀 **Installation**
```bash
git clone https://github.com/votre-utilisateur/sentinelvault.git
cd sentinelvault
cargo build --release
```
### Dépendances (Cargo.toml)
```toml
[dependencies]
aes-gcm = "0.10"
rand = "0.8"
keyring = "3"
```

---

## 💻 **Utilisation**
```bash
cargo run --release
```
Sortie attendue lors d'une première exécution (génération automatique de la clé maîtresse) :
```
[INFO] Trousseau système détecté, utilisation du stockage natif.
Secret déchiffré : sk-secret-token
Refus : Accès refusé : habilitation niveau 3 requise
```

<details>
<summary>🧪 Exécuter les tests unitaires (cliquer pour développer)</summary>

```bash
cargo test
```

</details>

<details>
<summary>🔧 Configurer le repli headless via variable d'environnement (cliquer pour développer)</summary>

```bash
export SENTINELVAULT_CLE_HEX="<64 caractères hexadécimaux représentant 32 octets>"
cargo run --release
```

</details>

## 🔐 **Sécurité cryptographique**
Aspect | Choix technique | Justification
---|---|---
Algorithme | AES-256-GCM | Chiffrement authentifié (AEAD), norme NIST, accéléré matériellement (AES-NI)
Taille de clé | 256 bits (32 octets) | Résistance maximale recommandée par l'ANSSI
Nonce | 96 bits, généré via `OsRng` | Unicité garantie par entropie système, jamais réutilisé
Stockage de la clé | Trousseau OS natif | Chiffrement par les identifiants de session utilisateur
Effacement mémoire | Trait `Drop` personnalisé | Écrasement des octets sensibles à la sortie de portée

> ⚠️ **Avertissement** : ce projet est à visée pédagogique. Toute mise en production doit faire l'objet d'un audit de sécurité indépendant, incluant une revue de la gestion des dépendances (`cargo audit`) et une analyse d'impact relative à la protection des données (AIPD/PIA).

## 🧯 **Gestion des erreurs**
Toutes les défaillances sont modélisées par l'enum exhaustif `VaultError`, éliminant tout recours à des panics implicites en environnement de production :
```rust
pub enum VaultError {
    SecretIntrouvable(String),
    ClePropreVide,
    ChiffrementInvalide { attendu: usize, obtenu: usize },
    EchecChiffrement,
    EchecDechiffrement,
    AccesRefuse(u8),
    TrousseauInaccessible(String),
    CleAbsenteDuTrousseau,
}
```

<details>
<summary>Pourquoi ne pas utiliser <code>unwrap()</code> ou <code>panic!()</code> ? (cliquer pour développer)</summary>

Dans un contexte de sécurité, un arrêt brutal du programme (`panic!`) peut constituer en soi une vulnérabilité de disponibilité (déni de service). La propagation via `Result<T, VaultError>` et l'opérateur `?` impose un traitement explicite de chaque cas d'échec par l'appelant, conformément au principe de programmation défensive.

</details>

---

## ⚖️ **Conformité RGPD et éthique**

Ce projet applique plusieurs principes issus du Règlement Général sur la Protection des Données (UE 2016/679) et des recommandations de l'ANSSI :

* **Article 5 (minimisation)** : aucune donnée sensible n'est conservée au-delà du strict nécessaire, illustré par l'effacement mémoire via `Drop`.
* **Article 32 (sécurité du traitement)** : chiffrement fort AES-256-GCM et délégation de la clé au trousseau OS constituent des mesures techniques appropriées.
* **Article 25 (privacy by design)** : l'absence de clé codée en dur et la modularisation stricte traduisent une sécurité pensée dès la conception.

<details>
<summary>📜 Note sur le règlement IA-Act (UE 2024/1689) (cliquer pour développer)</summary>

Ce projet ne relève pas du champ d'application du règlement IA-Act, ne mettant en œuvre aucun système d'intelligence artificielle au sens de l'article 3. Il pourrait néanmoins servir de brique de sécurisation des secrets dans une architecture plus large intégrant des systèmes d'IA à risque, auquel cas une revue de conformité spécifique devrait être menée.

</details>

## 📄 Licence
Distribué sous licence MIT. Voir le fichier [`LICENSE`](license) pour plus de détails.

<div align="center">

[Haut](#️-sentinelvault) · [License](license) · [Sécurité](security.md)