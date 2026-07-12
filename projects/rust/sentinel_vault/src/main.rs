mod erreurs;
mod crypto;
mod coffre;
mod controle_acces;
mod trousseau;

use coffre::Coffre;
use controle_acces::verifier_habilitation;
use zeroize::Zeroizing;


fn main() {
    let cle_maitresse = match trousseau::obtenir_ou_creer_cle() {
        Ok(cle) => Zeroizing::new(cle), // encapsuler immédiatement pour garantir l'effacement automatique
        Err(e) => {
            eprintln!("Impossible d'obtenir la clé maîtresse : {e}");
            std::process::exit(1) // retirer le point-virgule final : laisser `!` comme expression finale du bloc
        }
    };

    let mut coffre: Coffre<String> = match Coffre::nouveau(cle_maitresse) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Échec d'initialisation du coffre : {e}");
            std::process::exit(1);
        }
    };

    if let Err(e) = coffre.stocker("api_openai", &String::from("sk-secret-token")) {
        eprintln!("Échec de stockage : {e}");
    }

    match coffre.recuperer("api_openai") {
        Ok(valeur) => println!("Secret déchiffré : {valeur}"),
        Err(e) => println!("Erreur : {e}"),
    }

    match verifier_habilitation(2, 3, false) {
        Ok(()) => println!("Accès autorisé."),
        Err(e) => println!("Refus : {e}"),
    }
}