use std::io::{self, Write};

/// Représente les opérations arithmétiques avec une sémantique rigoureuse.
#[derive(Debug)]
enum Operateur {
    Addition,
    Soustraction,
    Multiplication,
    Division,
    DivisionEuclidienne, // Le reste (résidu)
    DivisionEntiere,    // Le quotient (tronqué)
}

fn main() {
    println!("--- Calculatrice Rust Haute Précision ---");

    // INITIALISATION DE LA MÉMOIRE :
    // On utilise Option<f64> : 'None' signifie que la mémoire est vide,
    // 'Some(f64)' indique qu'une valeur y est séquestrée.
    let mut memoire: Option<f64> = None;

    // L'usage de 'loop' ici instaure une structure itérative pérenne.
    loop {
        println!("\n--- Nouvelle session de calcul ---");

        // Les saisies doivent être à l'intérieur de la boucle pour être renouvelées.
        // On passe désormais la 'memoire' en argument pour permettre le rappel (MR).
        let a = saisir_nombre("Entrez le premier nombre (ou 'MR') 👉 ", memoire);
        let b = saisir_nombre("Entrez le second nombre (ou 'MR') 👉 ", memoire);

        afficher_menu();
        let op = choisir_operateur();

        // Évaluation et gestion du résultat via le pattern matching.
        match evaluer(a, b, op) {
            Ok(res) => {
                let affichage = formater_intelligent(res);
                println!("\n✅ Résultat : {}", affichage);

                // GESTION DU STOCKAGE (MS) :
                // On met à jour l'état de la mémoire après chaque calcul réussi.
                memoire = gerer_sauvegarde(res, memoire);
            }
            Err(e) => println!("\n❌ Erreur : {}", e),
        }

        // Sortie de boucle si l'utilisateur souhaite cesser l'activité.
        if !demander_continuation() {
            println!("Fin du programme. Au revoir !");
            break;
        }
    }
}

/// Évalue l'opération avec une gestion préventive des indéterminations.
fn evaluer(a: f64, b: f64, op: Operateur) -> Result<f64, String> {
    match op {
        Operateur::Addition => Ok(a + b),
        Operateur::Soustraction => Ok(a - b),
        Operateur::Multiplication => Ok(a * b),
        Operateur::Division => {
            if b == 0.0 { Err("Division par zéro indéfinie".into()) } else { Ok(a / b) }
        }
        Operateur::DivisionEuclidienne => Ok(a % b),
        Operateur::DivisionEntiere => {
            if b == 0.0 { Err("Division entière par zéro impossible".into()) } else { Ok((a / b).floor()) }
        }
    }
}

/// Procède à un "polissage" cosmétique du résultat pour masquer le bruit binaire (IEEE 754).
fn formater_intelligent(n: f64) -> String {
    if n == n.trunc() {
        return format!("{}", n as i64);
    }

    let arrondi = (n * 1e12).round() / 1e12;
    if (n - arrondi).abs() < 1e-13 {
        return format!("{}", arrondi);
    }

    format!("{}", n)
}

// --- Logique de Mémoire ---

/// Gère l'opération MS (Memory Store).
fn gerer_sauvegarde(valeur_actuelle: f64, memoire_actuelle: Option<f64>) -> Option<f64> {
    print!("Voulez-vous stocker ce résultat en mémoire (MS) ? (o/n) 👉 ");
    io::stdout().flush().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();

    let rep = s.trim().to_lowercase();
    if rep == "o" || rep == "oui" || rep == "y" || rep == "yes" {
        println!("💾 Valeur {} sauvegardée.", valeur_actuelle);
        Some(valeur_actuelle)
    } else {
        memoire_actuelle // On conserve l'ancienne mémoire si on ne sauvegarde pas
    }
}

// --- Fonctions utilitaires d'interface ---

/// Capture la saisie numérique ou le rappel mémoire (MR).
fn saisir_nombre(message: &str, memoire: Option<f64>) -> f64 {
    loop {
        print!("{}", message);
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Échec de lecture");

        let saisie = s.trim().to_uppercase();

        // GESTION DU RAPPEL MÉMOIRE (MR) :
        if saisie == "MR" {
            match memoire {
                Some(v) => {
                    println!("📥 Valeur rappelée : {}", v);
                    return v;
                }
                None => println!("⚠️ La mémoire est actuellement vide."),
            }
        } else {
            // Tentative de conversion classique
            match saisie.parse::<f64>() {
                Ok(n) => return n,
                Err(_) => println!("❌ Veuillez saisir un nombre réel ou 'MR'."),
            }
        }
    }
}

fn afficher_menu() {
    println!("\nSymboles autorisés :");
    println!(" [+] Addition | [-] Soustraction | [*] Multiplication");
    println!(" [/] Division | [%] Division euclidienne | [//] Quotient Entier");
}

fn choisir_operateur() -> Operateur {
    loop {
        print!("\n👉 Quel opérateur voulez-vous ? ");
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Échec de lecture");
        match s.trim() {
            "+" =>  return Operateur::Addition,
            "-" =>  return Operateur::Soustraction,
            "*" =>  return Operateur::Multiplication,
            "/" =>  return Operateur::Division,
            "%" =>  return Operateur::DivisionEuclidienne,
            "//" => return Operateur::DivisionEntiere,
            _    => println!("❌ Symbole non reconnu. Veuillez respecter la nomenclature."),
        }
    }
}

fn demander_continuation() -> bool {
    loop {
        print!("\nVoulez-vous effectuer un autre calcul ? (o/n) 👉 ");
        io::stdout().flush().unwrap();

        let mut reponse = String::new();
        io::stdin().read_line(&mut reponse).expect("Erreur de lecture");

        match reponse.trim().to_lowercase().as_str() {
            "o" | "oui" | "y" | "yes" => return true,
            "n" | "non" | "no"        => return false,
            _ => println!("Veuillez répondre par 'o' ou 'n'."),
        }
    }
}