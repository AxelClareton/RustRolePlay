use std::io::{stdout, Write};
use crate::zone::Zone;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::personnage::PNJ;

pub struct ListeNotifications {
    pub notifications: Vec<String>,
}

static NOTIFICATIONS: Lazy<Mutex<ListeNotifications>> = Lazy::new(|| {
    Mutex::new(ListeNotifications {
        notifications: Vec::new(),
    })
});

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}

pub fn ajouter_notification(message: &str) {
    let mut notifications = NOTIFICATIONS.lock().unwrap();
    notifications.notifications.push(message.to_string());
    if notifications.notifications.len() > 5 {
        notifications.notifications.remove(0);
    }
}

pub fn notifier(zone: &Zone, message: &str, tous_les_pnjs: &[PNJ]) {
    ajouter_notification(message);
    afficher_zone(zone, tous_les_pnjs);
}

pub fn afficher_zone(zone: &Zone, tous_les_pnjs: &[PNJ]) {
    clear_terminal();

    println!("\n🌍 Vous êtes dans la zone : {}", zone.nom);
    println!("------------------------------");
    println!("📜 Description : {}", zone.description);
    println!("🚪 Sorties possibles :");
    for conn in &zone.connection {
        println!("➡️  Vers '{}'", conn.direction);
    }
    println!("Il y a {} coffres dans la zone", zone.compter_coffre());
    
    let pnjs_dans_la_zone: Vec<&PNJ> = tous_les_pnjs
        .iter()
        .filter(|p| p.zone_id == zone.id as u32)
        .collect();

    if !pnjs_dans_la_zone.is_empty() {
        println!("👥 PNJ présents :");
        for pnj in pnjs_dans_la_zone {
            if (pnj.personnage.est_vivant){
                println!("- {}", pnj.personnage.nom);
            }
        }
    }
    println!("------------------------------");

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
            return "q".to_string();
        }

        if choixpossibles.contains(&choix.to_string()) {
            return choix.to_string();
        } else {
            println!("❌ Choix invalide. Veuillez réessayer !\n");
        }
    }
}