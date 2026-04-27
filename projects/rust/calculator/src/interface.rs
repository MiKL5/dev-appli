use std::io::{self, Write};
use crate::modeles::Operateur;

/// Affiche le lexique des glyphes mathématiques autorisés.
pub fn afficher_menu() {
    println!("Voici les symboles et fonctions supportés");
    println!(" [+] Addition | [-] Soustraction | [*] Multiplication | [/] Division");
    println!(" [%] Reste | [//] Quotient | [^] Puissance");
    println!(" [sqrt] Racine | [ln] Logarithme népérien | [cos] Cosinus | [sin] Sinus | [abs] Absolu");
}

pub fn saisir_nombre(message: &str, memoire: Option<f64>) -> f64 {
    loop {
        print!("{}", message);
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).ok();
        let saisie = s.trim().to_uppercase();

        if saisie == "MR" {
            if let Some(v) = memoire {
                println!("📥 Rappel mémoriel {}", v);
                return v;
            }
            println!("⚠️ La mémoire est présentement vacante.");
        } else if let Ok(n) = saisie.parse::<f64>() {
            return n;
        } else {
            println!("❌ Saisissez un nombre ou 'MR'.");
        }
    }
}

pub fn choisir_operateur() -> Operateur {
    loop {
        print!("Quel opérateur (+, -, *, /, %, //, ^, sqrt, ln, cos, sin, abs) ? ");
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).ok();

        match s.trim().to_lowercase().as_str() {
            "+"        => return Operateur::Addition,
            "-"        => return Operateur::Soustraction,
            "*"        => return Operateur::Multiplication,
            "/"        => return Operateur::Division,
            "%"        => return Operateur::DivisionEuclidienne,
            "//"       => return Operateur::DivisionEntiere,
            "^" | "**" => return Operateur::Puissance,
            "sqrt"     => return Operateur::RacineCarree,
            "ln"       => return Operateur::LogarithmeNeperien,
            "cos"      => return Operateur::Cosinus,
            "sin"      => return Operateur::Sinus,
            "abs"      => return Operateur::ValeurAbsolue,
            _          => println!("❌ Ce n'est pas le bon."),
        }
    }
}

pub fn gerer_ms(valeur_actuelle: f64, memoire_actuelle: Option<f64>) -> Option<f64> {
    print!("\nVoulez-vous consigner ce résultat (MS) ? (o/n) ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    let rep = s.trim().to_lowercase();
    if ["o", "oui", "y", "yes"].contains(&rep.as_str()) {
        println!("💾 Valeur {} sauvegardée dans les circuits.", valeur_actuelle);
        Some(valeur_actuelle)
    } else {
        memoire_actuelle
    }
}

pub fn demander_continuation() -> bool {
    print!("\nSouhaitez-vous une nouvelle opération ? (o/n) ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    matches!(s.trim().to_lowercase().as_str(), "o" | "oui" | "y" | "yes")
}