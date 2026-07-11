/*
   EXERCICE : Moniteur de frappes clavier
   Objectif : manipuler les fonctions, la mutabilité, les types primitifs,
              les tuples, les arrays et la configuration interactive.
*/

mod utils;

use device_query::{DeviceQuery, DeviceState, Keycode}; // import des éléments utile depuis la bibliothèque device_query
use std::thread;                                       // import des outils de langage standard
use std::sync::Arc;                                    // pour partager ce booléen entre le programme et le gestionnaire Ctrl+C
use std::sync::atomic::{AtomicBool, Ordering};         // AJOUT : type booléen partagé entre threads + règles de synchronisation
use std::time::{Duration, Instant};

use crate::utils::{calculer_frequence, demander_configuration, est_touche_speciale}; // import des fonctions depuis utils.rs


fn main() {
    // demander_configuration() renvoie un TUPLE : (activée ?, durée en secondes)
    let (limite_active, duree_limite_secondes): (bool, u64) = demander_configuration(); // l'utilisateur veut-il une limite temporelle ?

    // message différent selon son choix
    if limite_active {
        println!("\nLa surveillance est de {duree_limite_secondes} secondes.");
    } else {
        println!("\nAucune limite 💭 appuie sur Ctrl+C pour voir tes statistiques.");
    }

    // BOOLÉEN PARTAGÉ : true tant que le programme doit continuer à tourner.
    // Arc permet de le partager en toute sécurité entre le thread principal
    // et le thread créé par ctrl+c pour écouter le signal.
    let en_cours = Arc::new(AtomicBool::new(true));

    // on clone le pointeur Arc (pas la valeur !) pour le donner au gestionnaire.
    let en_cours_signal = Arc::clone(&en_cours);

    // on enregistre une fonction (closure) qui s'exécute dès Ctrl+C.
    // Elle se contente de faire passer en_cours à false.
    ctrlc::set_handler(move || {
        en_cours_signal.store(false, Ordering::SeqCst);
        println!("\n\n⏹️  Ctrl+C reçu.\nFin de la surveillance.");
    })
        .expect("⚠️ Il est impossible d'installer le gestionnaire Ctrl+C.");

    // VARIABLE MUTABLE ; le compteur va augmenter à chaque touche pressée
    // C'est un NOMBRE ENTIER non-signé (il ne peut pas être négatif)
    let mut compteur_frappes: u32 = 0;

    // ARRAY FIXE de 5 cases.
    // Option<Keycode> ; aucune touche n'a encore été
    // pressée : chaque case vaut donc "None" (rien) au lancement
    let mut historique: [Option<Keycode>; 5] = [None; 5];

    // garde la trace de la prochaine case à remplir
    // dans le tableau historique (technique du "tableau circulaire")
    let mut index_historique: usize = 0;

    // mémorise les touches détectées à l'itération PRÉCÉDENTE, ne compte que les touches NOUVELLEMENT enfoncées
    let mut touches_precedentes: Vec<Keycode> = Vec::new();

    // device_state permet d'interroger l'état du clavier à tout moment
    let device_state = DeviceState::new();

    // instant_depart mémorise l'heure de démarrage, pour calculer
    // ensuite combien de temps s'est écoulé
    let instant_depart = Instant::now();

    println!("***** Démarrage de la surveillance du clavier *****\n");

    // BOUCLE PRINCIPALE : elle tourne tant que la condition d'arrêt n'est pas remplie
    // "loop" (boucle infinie) combiné à un "break" manuel ; la condition dépend du choix utilisateur
    loop {
        // Si l'utilisateur a activé une limite ET que le temps écoulé dépasse cette limite, on sort de la boucle avec "break"
        if limite_active && instant_depart.elapsed() >= Duration::from_secs(duree_limite_secondes) {
            break;
        }

        // nouvelle condition d'arrêt : Ctrl+C a été détecté (en_cours devenu false).
        if !en_cours.load(Ordering::SeqCst) {
            break;
        }

        // récupère la liste des touches actuellement enfoncées
        let touches_pressees = device_state.get_keys();

        // NOUVEAU FILTRE : on ne garde que les touches présentes dans
        // touches_pressees MAIS absentes de touches_precedentes, c'est-à-dire
        // les touches qui viennent d'être enfoncées à cet instant précis.
        let nouvelles_touches: Vec<&Keycode> = touches_pressees
            .iter()
            .filter(|touche| !touches_precedentes.contains(touche))
            .collect();

        // parcourt uniquement les touches NOUVELLEMENT détectées
        for touche in nouvelles_touches {
            // incrémente le compteur -> MUTATION de variable
            compteur_frappes += 1;

            // ce TUPLE regroupe la touche pressée ET le temps écoulé dans une seule variable composée de deux valeurs
            let evenement: (Keycode, f64) =
                (*touche, instant_depart.elapsed().as_secs_f64());

            // ce BOOLÉEN est vrai si la touche est une touche spéciale
            // (Shift, Ctrl), faux sinon. Calculé via une fonction qui prend un PARAMÈTRE (&touche)
            let est_speciale: bool = est_touche_speciale(touche);

            // affiche les informations frappées
            println!(
                "Touche : {:?} | Temps : {:.2}s | Spéciale : {}",
                evenement.0, evenement.1, est_speciale
            );

            // enregistrement de la touche dans le tableau circulaire.
            // Le "%" (modulo) permet de revenir à la case 0 quand on atteint la cinquième case.
            historique[index_historique % 5] = Some(*touche);
            index_historique += 1;
        }

        // NOUVEAU : on met à jour la mémoire pour le prochain tour de boucle,
        // en remplaçant l'ancien état par l'état actuel.
        touches_precedentes = touches_pressees;

        // une petite pause pour ne pas surcharger le processeur
        thread::sleep(Duration::from_millis(100));
    }

    // la boucle est terminée ; calcule des statistiques finales
    // NOMBRE DÉCIMAL (f64) : durée totale écoulée, à décimales
    let duree_totale = instant_depart.elapsed().as_secs_f64();

    // appel de la fonction calculant la fréquence moyenne
    let frequence = calculer_frequence(compteur_frappes, duree_totale);

    // Rapport final, avec la syntaxe d'IDENTIFIANTS CAPTURÉS
    println!("\n***** Rapport final *****");
    println!("Il y a {compteur_frappes} frappes");
    println!("Ta fréquence moyenne est de {frequence:.2} frappes/seconde");
    println!("Les 5 dernières touches sont {historique:?}");
}