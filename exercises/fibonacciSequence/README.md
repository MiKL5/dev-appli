# Fibonacci Iterative Engine

<div align="center">

![Rust 1.95](https://img.shields.io/badge/Rust-1.95-000000?style=flat-radius&logo=rust&logoColor=white)

</div>

Calcul du n-ième terme de la suite de Fibonacci via une approche **itérative** en Rust, sans récursion — évitant ainsi tout risque de stack overflow.
## Fonctionnement
La fonction `calculer_fibonacci(n: u32) -> u32` traite trois cas via `match` :
* `0` → retourne `0`
* `1` → retourne `1`
* `n > 1` → itère avec deux accumulateurs jusqu'au rang souhaité
## Utilisation
```bash
cargo run
```
La valeur du rang est définie directement dans `main()` :
```rust
let rang: u32 = 10; // Modifier ici pour changer le rang
```
**Sortie attendue :**

```
👉 Le terme de Fibonacci au rang 10 est 55
```