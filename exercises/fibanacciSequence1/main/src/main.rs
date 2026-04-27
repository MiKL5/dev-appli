use std::io;

fn main() {
    println!("--- Calculateur de Fibonacci ---");
    println!("👇 Veuillez entrer le rang (nombre entier) souhaité");

    let mut input_utilisateur = String::new();

    io::stdin()
        .read_line(&mut input_utilisateur)
        .expect("Erreur de lecture");

    let rang: u32 = match input_utilisateur.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ Entrez un entier positif.");
            return;
        }
    };

    // On traite directement le résultat de la fonction
    match calculer_fibonacci(rang) {
        Some(valeur) => println!("✅ Le terme au rang {} est {}", rang, valeur),
        None => println!("⚠️ La limite est atteinte. \nLe rang {} dépasse la capacité du type u32 (Le rang maximum est 47).", rang),
    }
}

fn calculer_fibonacci(n: u32) -> Option<u32> {
    match n {
        0 => Some(0),
        1 => Some(1),
        _ => {
            let mut a: u32 = 0;
            let mut b: u32 = 1;
            for _ in 2..=n {
                // L'opérateur ? renvoie None immédiatement en cas d'overflow
                let temp = a.checked_add(b)?;
                a = b;
                b = temp;
            }
            Some(b)
        }
    }
}