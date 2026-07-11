/*
Logique métier : lecture, filtrage et écriture du fichier BibTeX.
Un champ BibTeX peut s'étaler sur plusieurs lignes si son contenu est long.
Pour le détecter en entier, il faut compter les accolades ouvrantes et fermantes
pour savoir quand le champ se termine, quel que soit le nombre de lignes.
*/

use encoding_rs::WINDOWS_1252;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Cursor, Read, Write};
use std::path::Path;

/// Résultat du traitement, utile pour informer l'utilisateur sans dupliquer la logique d'affichage dans plusieurs endroits.
#[derive(Debug, Default)]
pub struct RapportMinification {
    pub lignes_totales: usize,
    pub lignes_supprimees: usize,
    /// Aperçu des champs supprimés (nom du champ + numéro de la ligne de départ)
    pub champs_supprimes: Vec<(String, usize)>,
    pub encodage_de_repli_utilise: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Delimiteur {
    Accolade,
    Guillemet,
}

struct ChampMultiligne {
    delimiteur: Delimiteur,
    profondeur_accolades: i32,
    nom_champ: String,
    ligne_debut: usize,
}

/// Nettoie un fichier BibTeX en retirant les champs listés dans `champs_exclus`.
///
/// Si `dry_run` vaut `true`, aucun fichier de sortie n'est écrit : la fonction analyse
/// le fichier et remonte un rapport, permettant de vérifier ce qui *serait* supprimé
/// avant de le faire réellement.
///
/// # Erreurs possibles
/// * Fichier d'entrée absent ou illisible
/// * Fichier de sortie impossible à créer (sauf en dry-run)
/// * Contenu du fichier non-UTF-8
/// * Accolades déséquilibrées dans un champ exclu (fichier .bib mal formé)
pub fn minifier_bibtex<P: AsRef<Path>>(
    entree: P,
    sortie: P,
    champs_exclus: &[String],
    dry_run: bool,
) -> io::Result<RapportMinification> {
    let entree = entree.as_ref();
    let sortie = sortie.as_ref();

    if !entree.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("le fichier d'entrée '{}' n'existe pas", entree.display()),
        ));
    }

    let (lecteur, encodage_de_repli_utilise): (Box<dyn BufRead>, bool) = ouvrir_flux_texte(entree)?;

    // En dry-run, ne pas ouvrir de fichier de sortie ; inutile de créer un fichier vide ou d'écraser un fichier existant.
    let mut ecrivain: Option<BufWriter<File>> = if dry_run {
        None
    } else {
        Some(BufWriter::new(File::create(sortie)?))
    };

    let mut rapport = RapportMinification {
        encodage_de_repli_utilise,
        ..RapportMinification::default()
    };

    let mut en_cours: Option<ChampMultiligne> = None;

    for (index, ligne_resultat) in lecteur.lines().enumerate() {
        let numero_ligne = index + 1;
        rapport.lignes_totales += 1;

        let ligne = ligne_resultat.map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("erreur de lecture à la ligne {numero_ligne} : {e}"),
            )
        })?;

        if let Some(champ) = en_cours.as_mut() {
            rapport.lignes_supprimees += 1;
            match champ_se_referme_sur_cette_ligne(champ, &ligne) {
                Some(true) => { en_cours = None; }
                Some(false) => {}
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "accolades déséquilibrées autour de la ligne {numero_ligne} \
                     (champ '{}' commencé ligne {})",
                            champ.nom_champ, champ.ligne_debut
                        ),
                    ));
                }
            }
            continue;
        }

        // n'est pas au milieu d'un champ exclu : vérifier si la ligne en démarre un.
        let ligne_nettoyee = ligne.trim();
        if let Some(nom_champ) = champ_exclu_detecte(ligne_nettoyee, champs_exclus) {
            demarrer_suppression_champ(&ligne, nom_champ, numero_ligne, &mut en_cours);
            rapport.lignes_supprimees += 1;
            rapport.champs_supprimes.push((
                en_cours.as_ref().map(|c| c.nom_champ.clone()).unwrap_or_default(),
                numero_ligne,
            ));
            continue;
        }

        // Ligne normale à conserver.
        if let Some(w) = ecrivain.as_mut() {
            writeln!(w, "{}", ligne)?;
        }
    }

    if let Some(champ) = en_cours {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "fin de fichier atteinte alors que le champ '{}' (ligne {}) \
             n'est jamais refermé",
                champ.nom_champ, champ.ligne_debut
            ),
        ));
    }

    if let Some(mut w) = ecrivain {
        w.flush()?;
    }

    Ok(rapport)
}

fn ouvrir_flux_texte(chemin: &Path) -> io::Result<(Box<dyn BufRead>, bool)> {
    let mut octets = Vec::new();
    File::open(chemin)?.read_to_end(&mut octets)?;
    match String::from_utf8(octets.clone()) {
        Ok(texte) => Ok((Box::new(Cursor::new(texte.into_bytes())), false)),
        Err(_) => {
            let (texte_decode, _, _) = WINDOWS_1252.decode(&octets);
            Ok((Box::new(Cursor::new(texte_decode.into_owned().into_bytes())), true))
        }
    }
}

fn detecter_delimiteur(ligne: &str) -> Option<Delimiteur> {
    let position_egal = ligne.find('=')?;
    let apres_egal = ligne[position_egal + 1..].trim_start();
    match apres_egal.chars().next()? {
        '{' => Some(Delimiteur::Accolade),
        '"' => Some(Delimiteur::Guillemet),
        _ => None,
    }
}

fn demarrer_suppression_champ(
    ligne: &str,
    nom_champ: String,
    numero_ligne: usize,
    en_cours: &mut Option<ChampMultiligne>,
) {
    let delimiteur = detecter_delimiteur(ligne).unwrap_or(Delimiteur::Accolade);
    let se_ferme_immediatement = match delimiteur {
        Delimiteur::Accolade => compter_solde_accolades(ligne) <= 0,
        Delimiteur::Guillemet => compter_guillemets_non_echappes(ligne) % 2 == 0,
    };
    if !se_ferme_immediatement {
        *en_cours = Some(ChampMultiligne {
            delimiteur,
            profondeur_accolades: compter_solde_accolades(ligne),
            nom_champ,
            ligne_debut: numero_ligne,
        });
    }
}

fn champ_se_referme_sur_cette_ligne(champ: &mut ChampMultiligne, ligne: &str) -> Option<bool> {
    match champ.delimiteur {
        Delimiteur::Accolade => {
            champ.profondeur_accolades += compter_solde_accolades(ligne);
            if champ.profondeur_accolades < 0 { None } else { Some(champ.profondeur_accolades == 0) }
        }
        Delimiteur::Guillemet => {
            Some(compter_guillemets_non_echappes(ligne) % 2 == 1)
        }
    }
}

/// Vérifie si une ligne (déjà "trim"ée) démarre un des champs à exclure.
///
/// Exiger que le nom du champ soit suivi (après d'éventuels espaces) d'un signe "=",
/// pour éviter de matcher par erreur un champ dont le nom commence par les mêmes lettres.
fn champ_exclu_detecte(ligne: &str, champs_exclus: &[String]) -> Option<String> {
    let ligne_minuscule = ligne.to_lowercase();

    champs_exclus.iter().find_map(|champ| {
        let champ_minuscule = champ.trim().to_lowercase();
        if champ_minuscule.is_empty() {
            return None;
        }

        if ligne_minuscule.starts_with(&champ_minuscule) {
            let reste = ligne_minuscule[champ_minuscule.len()..].trim_start();
            if reste.starts_with('=') {
                return Some(champ.trim().to_string());
            }
        }
        None
    })
}

/// Calculer le solde d'accolades d'une ligne : nombre de "{" moins nombre de "}".
/// Un solde positif signifie qu'on ouvre plus qu'on ne ferme (champ qui continue),
/// un solde négatif signifierait un déséquilibre (erreur potentielle).
fn compter_solde_accolades(ligne: &str) -> i32 {
    let octets = ligne.as_bytes();
    let mut solde = 0i32;
    for (i, &octet) in octets.iter().enumerate() {
        if i > 0 && octets[i - 1] == b'\\' { continue; }
        match octet {
            b'{' => solde += 1,
            b'}' => solde -= 1,
            _ => {}
        }
    }
    solde
}

fn compter_guillemets_non_echappes(ligne: &str) -> i32 {
    let octets = ligne.as_bytes();
    let mut compte = 0i32;
    for (i, &octet) in octets.iter().enumerate() {
        if octet == b'"' && !(i > 0 && octets[i - 1] == b'\\') {
            compte += 1;
        }
    }
    compte
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn champs_defaut() -> Vec<String> {
        vec!["abstract".to_string(), "keywords".to_string()]
    }

    #[test]
    fn supprime_champ_simple_une_ligne() {
        let entree = "test_simple_in.bib";
        let sortie = "test_simple_out.bib";
        fs::write(
            entree,
            "@article{ex,\n  title={Titre},\n  abstract={Un résumé},\n  year={2026}\n}",
        )
        .unwrap();

        let rapport = minifier_bibtex(entree, sortie, &champs_defaut(), false).unwrap();
        let contenu = fs::read_to_string(sortie).unwrap();

        assert_eq!(rapport.lignes_supprimees, 1);
        assert!(!contenu.contains("abstract"));
        assert!(contenu.contains("title={Titre}"));

        fs::remove_file(entree).unwrap();
        fs::remove_file(sortie).unwrap();
    }

    #[test]
    fn supprime_champ_multi_lignes() {
        let entree = "test_multi_in.bib";
        let sortie = "test_multi_out.bib";
        fs::write(
            entree,
            "@article{ex,\n  title={Titre},\n  abstract={Un résumé\n  qui continue\n  sur plusieurs lignes},\n  year={2026}\n}",
        )
        .unwrap();

        let rapport = minifier_bibtex(entree, sortie, &champs_defaut(), false).unwrap();
        let contenu = fs::read_to_string(sortie).unwrap();

        assert_eq!(rapport.lignes_supprimees, 3);
        assert!(!contenu.contains("résumé"));
        assert!(contenu.contains("year={2026}"));

        fs::remove_file(entree).unwrap();
        fs::remove_file(sortie).unwrap();
    }

    #[test]
    fn dry_run_ne_cree_pas_de_fichier() {
        let entree = "test_dry_in.bib";
        let sortie = "test_dry_out.bib";
        fs::write(entree, "@article{ex,\n  abstract={x},\n  year={2026}\n}").unwrap();

        // On s'assure que le fichier de sortie n'existe pas avant le test.
        let _ = fs::remove_file(sortie);

        let rapport = minifier_bibtex(entree, sortie, &champs_defaut(), true).unwrap();

        assert_eq!(rapport.lignes_supprimees, 1);
        assert!(!Path::new(sortie).exists());

        fs::remove_file(entree).unwrap();
    }

    #[test]
    fn erreur_si_accolades_non_fermees() {
        let entree = "test_erreur_in.bib";
        let sortie = "test_erreur_out.bib";
        fs::write(
            entree,
            "@article{ex,\n  abstract={Ce champ ne se ferme jamais\n  year={2026}\n}",
        )
        .unwrap();

        let resultat = minifier_bibtex(entree, sortie, &champs_defaut(), false);
        assert!(resultat.is_err());

        fs::remove_file(entree).unwrap();
    }
}
