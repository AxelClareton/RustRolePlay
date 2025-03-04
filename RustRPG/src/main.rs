mod moteur;
mod coffre;
mod zone;
mod inventaire;
mod objet;
use objet::{ajouter_objet, OBJETS_DISPONIBLES};
use zone::Zone;
use moteur::{charger_zones};
use rand::Rng;
use crate::moteur::charger_objets;

fn se_deplacer(zones: &mut Vec<Zone>, current_zone_index: &mut usize, direction: &str) {
    let current_zone = &zones[*current_zone_index];

    // Trouver la connexion
    if let Some(conn) = current_zone.connection.iter().find(|c| c.direction == direction) {
        // Trouver la nouvelle zone via l'ID de la connexion
        if let Some(new_index) = zones.iter().position(|z| z.id == conn.id_dest.parse::<u8>().unwrap()) {
            if zones[new_index].ouvert {
                *current_zone_index = new_index; // Mise à jour de l'index
                zones[*current_zone_index].afficher_zone();
            }
            else {
                println!("Voulez vous acheter cette zone pour {}? (oui pour acheter, autres réponses pour non)", zones[new_index].prix);
                let mut choix = String::new();
                std::io::stdin().read_line(&mut choix).expect("❌ Erreur de lecture !");
                let choix = choix.trim();
                match choix {
                    "oui" => {
                        zones[new_index].ouvert = true;
                        //déduire le prix
                        *current_zone_index = new_index; // Mise à jour de l'index
                        zones[*current_zone_index].afficher_zone();
                    }
                    _ => {
                        println!("Zone non acheté, vous restez dans la même zone)");
                    }
                }
            }

        } else {
            println!("⚠️ La zone de destination n'a pas été trouvée !");
        }
    } else {
        println!("❌ Vous êtes arrivé au bout du monde, faites demi-tour !");
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


    // Message d'accueil
    println!("✨ Bienvenue dans le RustRPG !");
    zones[current_zone_index].afficher_zone();
    let mut rng = rand::rng();
    // Boucle principale du jeu
    loop {
        println!("Que voulez-vous faire ? ('d' pour vous déplacer, 'q' pour quitter, 'c' pour fouiller la zone)");

        let mut choix = String::new();
        std::io::stdin().read_line(&mut choix).expect("❌ Erreur de lecture !");
        let choix = choix.trim();

        match choix {
            "q" => {
                println!("👋 Au revoir !");
                break;
            }
            "c" => {
                zones[current_zone_index].afficher_coffre();
            }
            "d" => {
                println!("🚪 Vers quelle direction voulez-vous aller ?");
                let mut direction = String::new();
                std::io::stdin().read_line(&mut direction).expect("❌ Erreur de lecture !");
                let direction = direction.trim();

                se_deplacer(&mut zones, &mut current_zone_index, direction);


                if rng.random_range(0..99) < 10 {
                    println!("🎉 L'événement rare s'est produit !");
                } else {
                    println!("❌ Rien ne se passe cette fois.");
                }

            }
            "nord" | "sud" | "est" | "ouest" => {
                se_deplacer(&mut zones, &mut current_zone_index, choix);
                if rng.random_range(0..99) < 10 {
                    println!("🎉 L'événement rare s'est produit !");
                }
            }
            _ => println!("❌ Commande inconnue !"),
        }
    }
}

