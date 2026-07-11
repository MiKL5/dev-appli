/*
   Module utilitaire : logique de traitement des frappes clavier
   et configuration interactive du programme.
*/

use device_query::Keycode;
use std::io::{self, Write};


/// Détermine si une touche appartient aux touches "spéciales".
/// Le mot-clé `pub` rend la fonction accessible depuis main.rs.
pub fn est_touche_speciale(touche: &Keycode) -> bool {
    matches!(
        touche,
        Keycode::LShift | Keycode::RShift | Keycode::LControl | Keycode::RControl
    )
}

/// Calcule la fréquence moyenne de frappe (frappes/seconde).
pub fn calculer_frequence(nb_frappes: u32, duree_secondes: f64) -> f64 {
    if duree_secondes == 0.0 {
        return 0.0;
    }
    nb_frappes as f64 / duree_secondes
}

/// FONCTION "CHEF D'ORCHESTRE" :
/// demande à l'utilisateur s'il souhaite activer une limite de durée, et laquelle
/// APPELLE en interne demander_booleen() puis demander_duree_secondes().
/// Renvoie un TUPLE (limite_active: bool, duree_choisie: u64).
pub fn demander_configuration() -> (bool, u64) {
    // Étape 1 : on appelle la fonction auxiliaire demander_booleen()
    let limite_active: bool = demander_booleen(
        "Veux-tu limiter la durée d'écoute ? (o/n) 👉 "
    );

    if !limite_active {
        // L'utilisateur ne veut pas de limite : on arrête ici,
        // demander_duree_secondes() n'est JAMAIS appelée dans ce cas.
        return (false, 0);
    }

    // Étape 2 : on appelle la fonction auxiliaire demander_duree_secondes()
    // UNIQUEMENT si limite_active vaut true (grâce au "return" ci-dessus).
    let duree_choisie: u64 = demander_duree_secondes(
        "Quelle durée d'écoute souhaites-tu (en secondes) 👉 "
    );

    (true, duree_choisie)
}

/// FONCTION AUXILIAIRE :
/// affiche un message et lit une réponse o/n, renvoie un BOOLÉEN.
/// appelée uniquement par demander_configuration().
fn demander_booleen(message: &str) -> bool {
    loop {
        print!("{message}");
        io::stdout().flush().unwrap();

        let mut reponse = String::new();
        io::stdin()
            .read_line(&mut reponse)
            .expect("⚠️ L'entrée est illisible.");

        match reponse.trim().to_lowercase().as_str() {
            "o" | "oui" | "y" | "yes" => return true,
            "n" | "non" | "no" => return false,
            _ => println!("😕 Je ne comprends pas. \nRépond par 'o', 'oui', 'y', 'yes', 'n', 'no' ou 'non'."),
        }
    }
}

/// FONCTION AUXILIAIRE :
/// affiche un message et lit une durée en secondes en NOMBRE ENTIER
/// appelée uniquement par demander_configuration(),
/// et seulement si l'utilisateur a répondu "oui" à demander_booleen().
fn demander_duree_secondes(message: &str) -> u64 {
    loop {
        print!("{message}");
        io::stdout().flush().unwrap();

        let mut entree = String::new();
        io::stdin()
            .read_line(&mut entree)
            .expect("⚠️ L'entrée est illisible.");

        match entree.trim().parse::<u64>() {
            Ok(valeur) if valeur > 0 => return valeur,
            Ok(_) => println!("🕐 La durée doit être strictement positive."),
            Err(_) => println!("Saisi un nombre entier valide."),
        }
    }
}