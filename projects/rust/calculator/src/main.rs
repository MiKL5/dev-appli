mod modeles;
mod logic;
mod interface;

fn main() {
    println!("--- Calculatrice Modulaire de Niveau Doctoral ---");
    let mut memoire: Option<f64> = None;

    loop {
        println!("\n--- Nouvelle Séquence de Calcul ---");

        // 1. On sollicite d'abord l'opérateur
        interface::afficher_menu();
        let op = interface::choisir_operateur();

        // 2. Capture du premier opérande
        let a = interface::saisir_nombre("Quel est le premier nombre ou MR ? ", memoire);

        // 3. Capture conditionnelle du second opérande
        let mut b = 0.0;
        if !op.est_unaire() {
            b = interface::saisir_nombre("Quel est le second ? ", memoire);
        }

        // 4. Évaluation
        match logic::evaluer(a, b, &op) {
            Ok(res) => {
                let txt = logic::formater_intelligent(res);
                println!("✅ Le résultat est {}", txt);
                memoire = interface::gerer_ms(res, memoire);
            }
            Err(e) => println!("❌ Erreur épistémologique {}", e),
        }

        if !interface::demander_continuation() {
            println!("À bientôt !");
            break;
        }
    }
}