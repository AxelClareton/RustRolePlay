mod moteur;
mod coffre;
mod zone;
mod inventaire;

use zone::Zone;
use moteur::{charger_zones};
use rand::Rng;

fn se_deplacer(zones: &mut Vec<Zone>, current_zone_index: &mut usize, direction: &str) {
    let current_zone = &zones[*current_zone_index];

    // Trouver la connexion
    if let Some(conn) = current_zone.connection.iter().find(|c| c.direction == direction) {
        // Trouver la nouvelle zone via l'ID de la connexion
        if let Some(new_index) = zones.iter().position(|z| z.id == conn.id_dest.parse::<u8>().unwrap()) {
            *current_zone_index = new_index; // Mise à jour de l'index
            zones[*current_zone_index].afficher_zone();
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

    // Trouver l'index de la zone de départ (id == 1)
    let mut current_zone_index = zones.iter_mut().position(|zone| zone.id == 1)
        .expect("⚠️ La zone avec l'id 1 n'a pas été trouvée !");

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

