use crate::modeles::Operateur;

pub fn evaluer(a: f64, b: f64, op: &Operateur) -> Result<f64, String> {
    match op {
        // Opérateurs Binaires
        Operateur::Addition => Ok(a + b),
        Operateur::Soustraction => Ok(a - b),
        Operateur::Multiplication => Ok(a * b),
        Operateur::Division => {
            if b == 0.0 { Err("La division par zéro est indéfinie".into()) } else { Ok(a / b) }
        },
        Operateur::DivisionEuclidienne => Ok(a % b),
        Operateur::DivisionEntiere => {
            if b == 0.0 { Err("Le quotient par zéro est impossible".into()) } else { Ok((a / b).floor()) }
        },
        Operateur::Puissance => Ok(a.powf(b)),

        // Opérateurs Unaires (b est ignoré ici)
        Operateur::ValeurAbsolue => Ok(a.abs()),
        Operateur::Cosinus => Ok(a.cos()),
        Operateur::Sinus => Ok(a.sin()),
        Operateur::LogarithmeNeperien => {
            if a <= 0.0 { Err("Le logarithme exige un réel strictement positif.".into()) }
            else { Ok(a.ln()) }
        },
        Operateur::RacineCarree => {
            if a < 0.0 { Err("La racine carrée d'un négatif requiert les complexes.".into()) }
            else { Ok(a.sqrt()) }
        },
    }
}

pub fn formater_intelligent(n: f64) -> String {
    if n == n.trunc() {
        return format!("{}", n as i64);
    }
    let arrondi = (n * 1e12).round() / 1e12;
    if (n - arrondi).abs() < 1e-13 {
        format!("{}", arrondi)
    } else {
        format!("{}", n)
    }
}


// MODULE DE TESTS UNITAIRES
#[cfg(test)]
mod tests {
    use super::*; // On importe les fonctions evaluer et formater_intelligent

    #[test]
    fn test_addition_simple() {
        let res = evaluer(10.0, 5.0, &Operateur::Addition).unwrap();
        assert_eq!(res, 15.0);
    }

    #[test]
    fn test_polissage_bruit_binaire() {
        // Test critique : 0.1 + 0.2 doit donner "0.3" après polissage
        let somme = 0.1 + 0.2;
        let poli = formater_intelligent(somme);
        assert_eq!(poli, "0.3");
    }

    #[test]
    fn test_division_par_zero() {
        let res = evaluer(10.0, 0.0, &Operateur::Division);
        // On vérifie que le résultat est bien une erreur
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), "Division par zéro indéfinie");
    }

    #[test]
    fn test_racine_negative() {
        let res = evaluer(-4.0, 0.0, &Operateur::RacineCarree);
        assert!(res.is_err());
    }

    #[test]
    fn test_puissance() {
        let res = evaluer(2.0, 3.0, &Operateur::Puissance).unwrap();
        assert_eq!(res, 8.0);
    }

    #[test]
    fn test_formatage_entier() {
        // Un nombre comme 5.0 doit s'afficher "5"
        assert_eq!(formater_intelligent(5.0), "5");
    }
}