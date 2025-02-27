mod moteur;
mod coffre;
mod zone;
mod inventaire;

use zone::Zone;
use moteur::{charger_zones};


fn se_deplacer<'a>(zones: &'a [Zone], current_zone: &mut &'a Zone, direction: &str) {
    // Cherche la connexion dans la zone actuelle
    if let Some(conn) = current_zone.connection.iter().find(|c| c.direction == direction) {
        // Trouve la zone de destination via l'id de la connexion
        if let Some(nouvelle_zone) = zones.iter().find(|z| z.id == conn.id_dest.parse::<u8>().unwrap()) {
            *current_zone = nouvelle_zone;
            current_zone.afficher_zone();
        } else {
            println!("⚠️ La zone de destination n'a pas été trouvée !");
        }
    } else {
        println!("❌ Vous êtes arrivé au bout du monde, faites demi tour !");
    }
}

fn main() {
    // Chargement des zones
    let zones = charger_zones().expect("⚠️ Impossible de charger les zones !");
    //println!("{:?}", zones);
    // Trouver la zone de départ (id == 1)
    let mut current_zone = zones.iter().find(|zone| zone.id == 1)
        .expect("⚠️ La zone avec l'id 1 n'a pas été trouvée !");

    // Message d'accueil
    println!("✨ Bienvenue dans le RustRPG !");
    current_zone.afficher_zone();

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
                current_zone.afficher_coffre()
            }
            "d" => {
                println!("🚪 Vers quelle direction voulez-vous aller ?");
                let mut direction = String::new();
                std::io::stdin().read_line(&mut direction).expect("❌ Erreur de lecture !");
                let direction = direction.trim();

                se_deplacer(&zones, &mut current_zone, direction);
            }
            "nord" => {
                se_deplacer(&zones, &mut current_zone, "nord");
            }
            "sud" => {
                se_deplacer(&zones, &mut current_zone, "sud");
            }
            "est" => {
                se_deplacer(&zones, &mut current_zone, "est");
            }
            "ouest" => {
                se_deplacer(&zones, &mut current_zone, "ouest");
            }
            _ => println!("❌ Commande inconnue !"),
        }
    }
}
