fn main() {
    // Nous définissons ici le rang souhaité
    let rang: u32 = 10;
    let resultat = calculer_fibonacci(rang);

    // Affichage formaté avec une macro d'interpolation
    println!("👉 Le terme de Fibonacci au rang {} est {}", rang, resultat);
}

/// Calcule le n-ième terme de la suite de Fibonacci de manière itérative.
/// Cette approche prévient tout risque d'overflow de la pile.
fn calculer_fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            // La boucle 'for' en Rust utilise des itérateurs de manière idiomatale
            for _ in 2..=n {
                let temporaire = a + b;
                a = b;
                b = temporaire;
            }
            b // En Rust, la dernière expression sans point-virgule est la valeur de retour
        }
    }
}