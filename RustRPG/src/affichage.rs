use std::io::{stdout, Write};
use crate::zone::Zone;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Default)]
pub struct ListeNotifications {
    pub notifications: Vec<String>,
}
// singleton pour gerer la liste de notif
static NOTIFICATIONS: Lazy<Mutex<ListeNotifications>> = Lazy::new(|| Mutex::new(ListeNotifications::default()));

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}



pub fn notifier(zone: &Zone,message: &str) {
    {
        let mut liste_notifications = NOTIFICATIONS.lock().unwrap();
        liste_notifications.notifications.push(message.to_string());
    }
    afficher_zone(zone);


}



pub fn afficher_zone(zone: &Zone) {
    clear_terminal();

    println!("\n🌍 Vous êtes dans la zone : {}", zone.nom);
    println!("{}", "-".repeat(30));
    println!("📜 Description : {}", zone.description);
    if zone.connection.is_empty() {
        println!("❌ Aucune sortie possible.");
    } else {
        println!("🚪 Sorties possibles :");
        for connexion in &zone.connection {
            println!("➡️  Vers '{}'", connexion.direction);
        }
    }
    println!("Il y a {} coffres dans la zone", zone.compter_coffre());
    println!("{}", "-".repeat(30));
    let liste_notifications = NOTIFICATIONS.lock().unwrap();



    if !liste_notifications.notifications.is_empty() {
        let len = liste_notifications.notifications.len();
        for notif in liste_notifications.notifications[len.saturating_sub(3)..].iter() {
            println!("🔔  - {}", notif);
        }
    }
}

pub fn faire_choix(message: & str, choixpossibles: &Vec<String>) -> String {
    loop {
        println!("{}", message);
        println!("⏎ Tapez 'q' pour quitter.");

        let mut choix = String::new();
        std::io::stdin()
            .read_line(&mut choix)
            .expect("❌ Erreur de lecture !");

        let choix = choix.trim();

        if choix.eq_ignore_ascii_case("q") {
            return String::new();
        }

        if choixpossibles.contains(&choix.to_string()) {
            return choix.to_string();
        } else {
            println!("❌ Choix invalide. Veuillez réessayer !\n");
        }
    }
}