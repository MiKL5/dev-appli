use anyhow::{anyhow, Context, Result};
use copypasta::{ClipboardContext, ClipboardProvider};
use std::io::{self, Write};

/// Représente une couleur RGB(A) avec des composantes 8 bits
/// et une opacité optionnelle comprise entre 0.0 et 1.0.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: Option<f32>,
}

impl Color {
    /// Construit une couleur RGB opaque (sans canal alpha).
    fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: None }
    }

    /// Construit une couleur RGBA.
    fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self { r, g, b, a: Some(a) }
    }

    /// Convertit la couleur en chaîne hexadécimale (sans le '#').
    /// Inclut le canal alpha si présent (format RRGGBBAA).
    fn to_hex(&self) -> String {
        let mut hex = format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b);
        if let Some(a) = self.a {
            let alpha = (a.clamp(0.0, 1.0) * 255.0).round() as u8;
            hex.push_str(&format!("{:02X}", alpha));
        }
        hex
    }

    /// Retourne une représentation lisible "rgb(r, g, b)" ou "rgba(r, g, b, a)".
    fn to_rgb_string(&self) -> String {
        match self.a {
            Some(a) => format!("rgba({}, {}, {}, {:.2})", self.r, self.g, self.b, a),
            None => format!("rgb({}, {}, {})", self.r, self.g, self.b),
        }
    }

    /// Luminance relative perçue (norme ITU-R BT.601), utile pour choisir
    /// une couleur de texte lisible en surimpression sur ce fond.
    fn luminance(&self) -> f32 {
        (0.299 * self.r as f32 + 0.587 * self.g as f32 + 0.114 * self.b as f32) / 255.0
    }

    /// Suggère "noir" ou "blanc" comme couleur de texte contrastante.
    fn contrasting_text(&self) -> &'static str {
        if self.luminance() > 0.5 {
            "noir (#000000)"
        } else {
            "blanc (#FFFFFF)"
        }
    }
}

/// Interprète l'entrée utilisateur, qu'elle soit au format RGB/RGBA
/// ("255, 100, 50" ou "255, 100, 50, 0.8") ou hexadécimal ("#FF6432" ou "FF6432AA").
/// La détection est automatique : aucune commande à mémoriser pour l'utilisateur.
fn parse_color(input: &str) -> Result<Color> {
    let trimmed = input.trim();

    if trimmed.starts_with('#') || is_hex_candidate(trimmed) {
        parse_hex(trimmed)
    } else {
        parse_rgb_list(trimmed)
    }
}

/// Détecte si une chaîne ressemble à un code hexadécimal
/// (6 ou 8 caractères hexadécimaux, sans virgule).
fn is_hex_candidate(s: &str) -> bool {
    let s = s.trim_start_matches('#');
    (s.len() == 6 || s.len() == 8) && s.chars().all(|c| c.is_ascii_hexdigit())
}

/// Parse une chaîne hexadécimale (6 ou 8 caractères) en couleur.
fn parse_hex(input: &str) -> Result<Color> {
    let hex = input.trim_start_matches('#');
    if hex.len() != 6 && hex.len() != 8 {
        anyhow::bail!("Longueur hexadécimale invalide (attendu 6 ou 8 caractères)");
    }
    // Validation stricte AVANT tout découpage par index d'octets : garantit que
    // la chaîne est intégralement ASCII, donc que chaque index utilisé plus bas
    // tombe sur une frontière de caractère valide (un caractère UTF-8 multi-octets
    // au mauvais endroit provoquerait sinon un panic de découpage de chaîne).
    if !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        anyhow::bail!("Caractère non hexadécimal détecté (attendu 0-9, a-f, A-F)");
    }

    let r = u8::from_str_radix(&hex[0..2], 16).context("Composante R hexadécimale invalide")?;
    let g = u8::from_str_radix(&hex[2..4], 16).context("Composante G hexadécimale invalide")?;
    let b = u8::from_str_radix(&hex[4..6], 16).context("Composante B hexadécimale invalide")?;

    if hex.len() == 8 {
        let a_u8 =
            u8::from_str_radix(&hex[6..8], 16).context("Composante A hexadécimale invalide")?;
        Ok(Color::rgba(r, g, b, a_u8 as f32 / 255.0))
    } else {
        Ok(Color::rgb(r, g, b))
    }
}

/// Parse une chaîne "R, G, B" ou "R, G, B, A" en couleur.
fn parse_rgb_list(input: &str) -> Result<Color> {
    let parts: Vec<&str> = input.split(',').map(str::trim).collect();
    if parts.len() != 3 && parts.len() != 4 {
        anyhow::bail!("Nombre de composantes invalide (attendu 3 pour RGB ou 4 pour RGBA)");
    }

    let r = parts[0].parse::<u8>().context("Composante R invalide (0-255)")?;
    let g = parts[1].parse::<u8>().context("Composante G invalide (0-255)")?;
    let b = parts[2].parse::<u8>().context("Composante B invalide (0-255)")?;

    if parts.len() == 4 {
        let a: f32 = parts[3].parse().context("Composante A invalide (0.0-1.0)")?;
        if !(0.0..=1.0).contains(&a) {
            anyhow::bail!("Alpha doit être compris entre 0.0 et 1.0");
        }
        Ok(Color::rgba(r, g, b, a))
    } else {
        Ok(Color::rgb(r, g, b))
    }
}

/// Copie une chaîne dans le presse-papiers du système.
/// Le contexte est recréé à chaque appel plutôt que conservé en mémoire
/// pour toute la durée du programme : cela évite de garder une ressource
/// système ouverte inutilement entre deux copies.
fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut ctx = ClipboardContext::new()
        .map_err(|e| anyhow!("Impossible d'initialiser le presse-papiers : {e}"))?;
    ctx.set_contents(text.to_string())
        .map_err(|e| anyhow!("Échec de la copie dans le presse-papiers : {e}"))
}

/// Affiche une invite, lit une ligne sur l'entrée standard et la retourne nettoyée.
fn lire_ligne(invite: &str) -> Result<String> {
    print!("{invite}");
    io::stdout().flush().context("Échec du flush de stdout")?;
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .context("Échec de la lecture de l'entrée")?;
    Ok(buffer.trim().to_string())
}

fn main() -> Result<()> {
    println!("🎨 Convertisseur de couleurs RGB/RGBA ⇉ Hexadécimal");
    println!("Les formats acceptés sont \"255, 100, 50\" · \"255, 100, 50, 0.8\"");
    println!("Tapez 'q' à tout moment pour quitter.\n");

    loop {
        let input = lire_ligne("> ")?;

        if input.eq_ignore_ascii_case("q") {
            println!("Au revoir !");
            break;
        }
        if input.is_empty() {
            println!("Veuillez entrer une couleur ou 'q' pour quitter.");
            continue;
        }

        let color = match parse_color(&input) {
            Ok(c) => c,
            Err(e) => {
                println!("Erreur : {e}. Exemple valide : 255, 100, 50 ou #FF6432");
                continue;
            }
        };

        // Affiche systématiquement les deux représentations,
        // quel que soit le format saisi par l'utilisateur (conversion bidirectionnelle).
        println!("Hexadécimal : #{}", color.to_hex());
        println!("RGB(A)      : {}", color.to_rgb_string());
        println!("Texte lisible sur ce fond : {}", color.contrasting_text());

        loop {
            println!("\nQue faire ensuite ?");
            println!("1. Convertir une autre couleur");
            println!("2. Copier le code hexadécimal (#{}) dans le presse-papiers", color.to_hex());
            println!("3. Quitter");

            match lire_ligne("Choix (1-3) : ")?.as_str() {
                "1" => break,
                "2" => {
                    let hex = format!("#{}", color.to_hex());
                    match copy_to_clipboard(&hex) {
                        Ok(()) => println!("✅ {hex} copié dans le presse-papiers !"),
                        Err(e) => println!("⚠️ {e}"),
                    }
                }
                "3" | "q" => {
                    println!("À bientôt !");
                    return Ok(());
                }
                _ => println!("Choix invalide."),
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rgb() {
        let c = parse_color("255, 100, 50").unwrap();
        assert_eq!((c.r, c.g, c.b, c.a), (255, 100, 50, None));
    }

    #[test]
    fn test_parse_rgba() {
        let c = parse_color("255, 100, 50, 0.5").unwrap();
        assert_eq!((c.r, c.g, c.b), (255, 100, 50));
        assert!((c.a.unwrap() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_parse_hex_rgb_avec_diese() {
        let c = parse_color("#FF6432").unwrap();
        assert_eq!((c.r, c.g, c.b, c.a), (255, 100, 50, None));
    }

    #[test]
    fn test_parse_hex_rgba_sans_diese() {
        let c = parse_color("FF643280").unwrap();
        assert_eq!((c.r, c.g, c.b), (255, 100, 50));
        assert!(c.a.is_some());
    }

    #[test]
    fn test_roundtrip_hex() {
        let c = Color::rgb(255, 100, 50);
        assert_eq!(c.to_hex(), "FF6432");
    }

    #[test]
    fn test_nombre_composantes_invalide() {
        assert!(parse_color("255, 100").is_err());
    }

    #[test]
    fn test_alpha_hors_bornes() {
        assert!(parse_color("255, 100, 50, 1.5").is_err());
    }

    #[test]
    fn test_hex_caractere_multioctet_ne_panique_pas() {
        // "AÉBCD" fait 6 octets (le 'É' en occupe 2) mais seulement 5 caractères :
        // une version antérieure paniquait ici en découpant au milieu du caractère.
        // On vérifie maintenant un simple retour d'erreur, sans panic.
        assert!(parse_color("#AÉBCD").is_err());
    }

    #[test]
    fn test_contraste_texte() {
        assert_eq!(Color::rgb(0, 0, 0).contrasting_text(), "blanc (#FFFFFF)");
        assert_eq!(Color::rgb(255, 255, 255).contrasting_text(), "noir (#000000)");
    }
}
