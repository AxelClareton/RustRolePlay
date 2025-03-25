mod moteur;
mod coffre;
mod zone;
mod inventaire;
mod objet;
mod affichage;

use zone::Zone;
use moteur::{charger_zones};
use rand::Rng;
use crate::moteur::charger_objets;
use std::thread::sleep;
use std::time::Duration;
use inventaire::Inventaire;

fn se_deplacer(zones: &mut Vec<Zone>, current_zone_index: &mut usize, direction: &str) {
    let current_zone = &zones[*current_zone_index];

    // Trouver la connexion
    if let Some(conn) = current_zone.connection.iter().find(|c| c.direction == direction) {
        // Trouver la nouvelle zone via l'ID de la connexion
        if let Some(new_index) = zones.iter().position(|z| z.id == conn.id_dest.parse::<u8>().unwrap()) {
            if zones[new_index].ouvert {
                *current_zone_index = new_index; // Mise à jour de l'index
                affichage::notifier(&zones[*current_zone_index], "Déplacement...");
                sleep(Duration::from_secs(5));
                affichage::notifier(&zones[*current_zone_index],"Vous êtes arrivés dans la zone");
            }
            else {
                let choix = affichage::faire_choix(
                    &format!("La zone {} n'est pas ouverte, voulez-vous l'acheter ? (oui/non)", conn.id_dest),
                    &vec!["oui".to_string(), "non".to_string()]
                );
                match choix.as_str() {
                    "oui" => {
                        zones[new_index].ouvert = true;
                        //déduire le prix
                        *current_zone_index = new_index; // Mise à jour de l'index
                        affichage::notifier(&zones[*current_zone_index], "Déplacement...");
                        sleep(Duration::from_secs(5));
                        affichage::notifier(&zones[*current_zone_index],"Vous êtes arrivés dans la zone");
                    }
                    _ => {
                        affichage::notifier(&zones[*current_zone_index], "Zone non achetée, vous restez dans la même zone");
                    }
                }
            }
        } else {
            affichage::notifier(&zones[*current_zone_index], "⚠️ La zone de destination n'a pas été trouvée !");
        }
    } else {
        affichage::notifier(&zones[*current_zone_index], "❌ Vous êtes arrivé au bout du monde, faites demi-tour !");
    }
}

fn main() {
    // Chargement des zones
    let mut zones = charger_zones().expect("⚠️ Impossible de charger les zones !");
    charger_objets().expect("⚠️ Impossible de charger les objets !");
    // Trouver l'index de la zone de départ (id == 1)
    let mut current_zone_index = zones.iter_mut().position(|zone| zone.id == 1)
        .expect("⚠️ La zone avec l'id 1 n'a pas été trouvée !");

    // ajouter_objet(1, "Épée");
    // ajouter_objet(2, "Potion");
    // ajouter_objet(3, "Bouclier");
    let inventaire = &mut Inventaire {
        taille : 5,
        objets: Vec::new(),
    };

    // Message d'accueil
    affichage::notifier(&zones[current_zone_index], "✨ Bienvenue dans le RustRPG !");
    affichage::afficher_zone(&zones[current_zone_index]);
    let mut rng = rand::rng();
    // Boucle principale du jeu
    loop {
        affichage::notifier(&zones[current_zone_index], "Que voulez-vous faire ? ('d' pour vous déplacer, 'q' pour quitter, 'c' pour fouiller la zone, le numéro du coffre)");

        let mut choix = String::new();
        std::io::stdin().read_line(&mut choix).expect("❌ Erreur de lecture !");
        let choix = choix.trim();
        let nbr_coffres = zones[current_zone_index].compter_coffre();
        match choix {
            "q" => {
                affichage::notifier(&zones[current_zone_index], "👋 Au revoir !");
                break;
            }
            "c" => {
                affichage::notifier(&zones[current_zone_index], "Fouillage de la zone en cours...");
                sleep(Duration::from_secs(5));
                zones[current_zone_index].fouiller_zone();
                affichage::afficher_zone(&zones[current_zone_index]);
            }
            "d" => {
                affichage::notifier(&zones[current_zone_index], "🚪 Vers quelle direction voulez-vous aller ?");
                let mut direction = String::new();
                std::io::stdin().read_line(&mut direction).expect("❌ Erreur de lecture !");
                let direction = direction.trim();

                se_deplacer(&mut zones, &mut current_zone_index, direction);

                if rng.random_range(0..99) < 10 {
                    affichage::notifier(&zones[current_zone_index], "🎉 L'événement rare s'est produit !");
                }
            }
            "nord" | "sud" | "est" | "ouest" => {
                se_deplacer(&mut zones, &mut current_zone_index, choix);
                if rng.random_range(0..99) < 10 {
                    affichage::notifier(&zones[current_zone_index], "🎉 L'événement rare s'est produit !");
                }
            }
            _ => {
                if let Ok(num) = choix.parse::<usize>() {
                    if (1..=nbr_coffres).contains(&num) {
                        let coffre = &mut zones[current_zone_index].coffres[num-1]; // Récupère le coffre sélectionné
                        match coffre.ouvrir() {
                            Some(objet) => {
                                affichage::notifier(&zones[current_zone_index], &format!("Objet trouvé : {}", objet));
                                inventaire.ajouter_objet(objet as u8);
                            },
                            None => affichage::notifier(&zones[current_zone_index], "Aucun objet à récupérer"),
                        }
                        inventaire.afficher();
                    } else {
                        affichage::notifier(&zones[current_zone_index], "❌ Commande inconnue !");
                    }
                } else {
                    affichage::notifier(&zones[current_zone_index], "❌ Commande inconnue !")
                }
            },
        }
    }
}

