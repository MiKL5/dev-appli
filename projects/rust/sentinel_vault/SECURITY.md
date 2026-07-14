# 🔒 **Politique de Sécurité — SentinelVault**

<div align="center">

![Security](https://img.shields.io/badge/Security-Policy-red?) 
![Responsible Disclosure](https://img.shields.io/badge/Disclosure-Responsible-orange?) 
![RGPD](https://img.shields.io/badge/RGPD-Art.32-purple?) 
![AES-256-GCM](https://img.shields.io/badge/AES--256--GCM-RustCrypto-2ea44f?)

</div>

## 📌 **Versions supportées**
Version | Statut de support sécurité
---|---|
0.3.x | ✅ Support actif
0.2.x | ⚠️ Correctifs critiques uniquement
< 0.2 | ❌ Non supporté, mise à jour requise
## 🚨 **Signaler une vulnérabilité**
**Ne créez jamais d'issue publique GitHub pour une faille de sécurité.** Toute vulnérabilité potentielle doit être signalée par voie confidentielle, conformément aux principes de divulgation responsable.
### Procédure
1. Envoyer un rapport détaillé à `securite@votre-domaine.fr` (à remplacer par votre contact réel).
2. Inclure une description de la vulnérabilité, les étapes de reproduction, et l'impact estimé (confidentialité, intégrité, disponibilité).
3. Un accusé de réception sera transmis dans un délai de **72 heures ouvrées**.
4. Un correctif ou un plan de remédiation sera communiqué dans un délai de **30 jours calendaires**, selon la criticité (CVSS).

<details>
<summary>📊 Grille de criticité utilisée (cliquer pour développer)</summary>

Score CVSS | Qualification | Délai de correction cible
---|---|---
9.0 – 10.0 | Critique | 72 heures
7.0 – 8.9 | Élevée | 7 jours
4.0 – 6.9 | Moyenne | 30 jours
0.1 – 3.9 | Faible | Prochaine version mineure

</details>

<details>
<summary>🎁 Politique de reconnaissance (cliquer pour développer)</summary>

En l'absence de programme de bug bounty formel, toute contribution responsable sera créditée dans le fichier `CHANGELOG.md` du projet, sauf demande explicite d'anonymat de la part du rapporteur.

</details>

---

## 🔄 **Procédure de rotation des clés**
La clé maîtresse AES-256, stockée dans le trousseau système via le module `trousseau.rs`, doit faire l'objet d'une rotation périodique pour limiter la fenêtre d'exposition en cas de compromission silencieuse.
### **Étapes recommandées**
1. Déchiffrer l'intégralité des secrets existants avec la clé actuelle (`recuperer_cle()`).
2. Générer une nouvelle clé via `generer_et_stocker_cle()`, qui écrase l'entrée précédente du trousseau.
3. Rechiffrer chaque secret avec la nouvelle clé et reconstruire le `Coffre<T>`.
4. Invalider explicitement toute copie résiduelle de l'ancienne clé en mémoire (le trait `Drop` de `CleSensible` s'en charge automatiquement à la sortie de portée).

<details>
<summary>📅 Fréquence recommandée (cliquer pour développer)</summary>

Contexte d'usage | Fréquence de rotation suggérée
---|---
Environnement de production critique | Tous les 90 jours
Environnement de développement/test | Tous les 180 jours
Après tout incident de sécurité suspecté | Immédiate

</details>

> ⚠️ **Point de vigilance** : la rotation de clé sans rechiffrement préalable des secrets existants rend ces derniers **définitivement irrécupérables**, car AES-GCM ne permet aucun déchiffrement partiel avec une clé erronée. Toujours exécuter l'étape 1 avant l'étape 2.

## 🖥️ **Gestion des environnements headless**
Sur un serveur Linux sans session graphique, le Secret Service (D-Bus) est structurellement absent, ce qui impose un mécanisme de repli documenté et maîtrisé.
### **Mécanisme de repli actif**
Le module `trousseau.rs` détecte automatiquement l'indisponibilité du trousseau natif et bascule sur la variable d'environnement `SENTINELVAULT_CLE_HEX`.

<details>
<summary>🔧 Injection sécurisée en production (cliquer pour développer)</summary>

**À proscrire absolument :**
* Commit de la clé dans le dépôt Git, même dans un fichier `.env` local.
* Transmission de la clé en clair via les journaux applicatifs (`stdout`/`stderr`).
**Recommandé :**
* Utiliser un gestionnaire de secrets d'infrastructure (HashiCorp Vault, AWS Secrets Manager, Azure Key Vault).
* Charger la variable via `EnvironmentFile=` dans une unité systemd, avec permissions de fichier restreintes à `600`.
```ini
# /etc/systemd/system/sentinelvault.service
[Service]
EnvironmentFile=/etc/sentinelvault/secrets.env
ExecStart=/usr/local/bin/sentinelvault
```

</details>

## 🛡️ **Bonnes pratiques de déploiement**

Pratique | Statut dans ce projet | Action recommandée avant production
---|---|---
Audit des dépendances | ❌ Non automatisé | Intégrer `cargo audit` en CI/CD
Analyse statique | ❌ Absente | Ajouter `cargo clippy --all-targets`
Tests de sécurité | ❌ Absents | Ajouter des tests unitaires sur `crypto.rs` et `trousseau.rs`
Chiffrement au repos du coffre | ❌ Non implémenté (v0.3) | Prioriser avant tout usage réel
Journalisation des accès | ❌ Absente | Requis pour conformité article 30 RGPD 

<details>
<summary>🧪 Commande d'audit recommandée (cliquer pour développer)</summary>

```bash
cargo install cargo-audit
cargo audit
```

</details>

## ⚖️ **Références réglementaires**
* **RGPD (UE 2016/679), article 32** — Sécurité du traitement : chiffrement et mesures techniques appropriées.
* **RGPD, article 33** — Notification à la CNIL dans un délai de 72 heures en cas de violation de données confirmée impliquant ce composant.
* **RGPD, article 30** — Registre des activités de traitement, incluant la journalisation des accès aux secrets.
* **Recommandations ANSSI** — Guide des mécanismes cryptographiques, préconisant AES-256 et une gestion rigoureuse du cycle de vie des clés.

<details>
<summary>📞 Obligation de notification CNIL (cliquer pour développer)</summary>

En cas de compromission avérée de la clé maîtresse entraînant un accès non autorisé à des données à caractère personnel stockées via ce coffre, le responsable de traitement dispose de **72 heures** pour notifier la CNIL conformément à l'article 33 du RGPD, et doit informer les personnes concernées sans délai si le risque est élevé (article 34).

</details>
<div align="center">

*Cette politique de sécurité doit être révisée à chaque changement majeur d'architecture cryptographique.*

</div>
<div align="center">

[Haut](#-politique-de-sécurité--sentinelvault) · [Fichier d'accueil](readme.md) · [License](license)