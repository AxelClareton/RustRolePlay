mod personnage;
mod objet;
mod monde;
mod interaction;

use interaction::menu_principal;
use personnage::{Personnage};
use monde::{Zone, Ennemi, Comportement};


fn main() {
    // 1. Création du personnage via le menu
    let joueur = menu_principal();

    // 2. Définir des zones précréées
    let mut zone1 = Zone {
        id: 1,
        nom: "Entrée du Donjon".to_string(),
        description: "Une entrée sombre, avec des portes en bois.".to_string(),
        obstacles: vec![],
        ennemis: vec![],
        objets: vec![],
        liaisons: vec![],
    };

    let mut zone2 = Zone {
        id: 2,
        nom: "Salle des Ennemis".to_string(),
        description: "Une grande salle avec plusieurs ennemis.".to_string(),
        obstacles: vec![],
        ennemis: vec![
            Ennemi { nom: "Goblin".to_string(), points_de_vie: 30, attaque: 5, comportement: Comportement::Hostile },
        ],
        objets: vec![],
        liaisons: vec![],
    };

    let mut zone3 = Zone {
        id: 3,
        nom: "Forêt Sombre".to_string(),
        description: "Une forêt dense avec des créatures étranges.".to_string(),
        obstacles: vec![],
        ennemis: vec![
            Ennemi { nom: "Loup Sauvage".to_string(), points_de_vie: 20, attaque: 7, comportement: Comportement::Hostile },
        ],
        objets: vec![],
        liaisons: vec![],
    };

    // 3. Créer les liaisons réciproques
    zone1.liaisons.push(zone2.clone());
    zone2.liaisons.push(zone1.clone()); // Zone2 est aussi liée à Zone1

    zone2.liaisons.push(zone3.clone());
    zone3.liaisons.push(zone2.clone()); // Zone3 est aussi liée à Zone2

    // 4. Boucle pour permettre au joueur de se déplacer
    let mut current_zone = zone1; // Zone de départ
    loop {
        // Afficher les détails de la zone actuelle
        current_zone.afficher_details();

        // Menu de déplacement
        println!("\nQue voulez-vous faire ?");
        println!("1. Se déplacer vers une autre zone");
        println!("2. Quitter");

        let mut choix = String::new();
        std::io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        let choix = choix.trim();

        match choix {
            "1" => {
                // Afficher les zones disponibles pour se déplacer
                println!("Vers quelle zone voulez-vous vous déplacer ?");
                if !current_zone.liaisons.is_empty() {
                    for (i, zone) in current_zone.liaisons.iter().enumerate() {
                        println!("{} - {}", i + 1, zone.nom);
                    }

                    let mut zone_choisie = String::new();
                    std::io::stdin().read_line(&mut zone_choisie).expect("Erreur de lecture");
                    let zone_choisie: usize = zone_choisie.trim().parse().unwrap_or(1);

                    // Sélectionner la zone et changer la zone actuelle
                    match current_zone.liaisons.get(zone_choisie - 1) {
                        Some(zone) => {
                            current_zone = zone.clone();
                            println!("Vous vous êtes déplacé vers : {}", current_zone.nom);
                        }
                        None => println!("Zone invalide."),
                    }
                } else {
                    println!("Aucune zone disponible pour le déplacement.");
                }
            }
            "2" => {
                println!("Merci d'avoir joué !");
                break;
            }
            _ => println!("Choix invalide, veuillez réessayer."),
        }
    }
}
