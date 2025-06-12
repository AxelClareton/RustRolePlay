use std::io::{stdout, Write};
use crate::zone::Zone;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::personnage::PNJ;

/// Structure contenant la liste des notifications à afficher à l'utilisateur.
pub struct ListeNotifications {
    /// Vecteur de messages récents (notifications) à afficher.
    pub notifications: Vec<String>,
}

/// Variable globale contenant les notifications, protégée par un mutex pour accès concurrent.
/// Utilise `once_cell::sync::Lazy` pour une initialisation paresseuse.
static NOTIFICATIONS: Lazy<Mutex<ListeNotifications>> = Lazy::new(|| {
    Mutex::new(ListeNotifications {
        notifications: Vec::new(),
    })
});

/// Efface le terminal en utilisant les séquences ANSI standard.
/// Cette fonction est utilisée pour simuler un rafraîchissement de l'affichage.
pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}


/// Ajoute un message à la liste des notifications.
///
/// Garde un maximum de 5 notifications ; les plus anciennes sont supprimées automatiquement.
///
/// # Arguments
/// * `message` - Le message à ajouter à la file de notifications.
pub fn ajouter_notification(message: &str) {
    let mut notifications = NOTIFICATIONS.lock().unwrap();
    notifications.notifications.push(message.to_string());
    if notifications.notifications.len() > 5 {
        notifications.notifications.remove(0);
    }
}

/// Ajoute une notification et réaffiche immédiatement la zone avec les PNJs.
///
/// # Arguments
/// * `zone` - La zone courante du joueur.
/// * `message` - Le message de notification à afficher.
/// * `tous_les_pnjs` - Liste complète de tous les PNJs du jeu.
pub fn notifier(zone: &Zone, message: &str, tous_les_pnjs: &[PNJ]) {
    ajouter_notification(message);
    afficher_zone(zone, tous_les_pnjs);
}

/// Affiche les informations de la zone actuelle, les sorties, les PNJs présents,
/// ainsi que les dernières notifications.
///
/// # Arguments
/// * `zone` - Référence à la zone à afficher.
/// * `tous_les_pnjs` - Liste complète de tous les PNJs du jeu.
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
            if pnj.personnage.est_vivant {
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


/// Affiche un message et attend une entrée utilisateur correspondant à un choix valide.
///
/// Affiche aussi une option pour quitter en tapant `'q'`.
///
/// # Arguments
/// * `message` - Le message d'instruction affiché à l'utilisateur.
/// * `choixpossibles` - Liste des chaînes représentant les entrées valides.
///
/// # Retour
/// Renvoie le choix validé de l'utilisateur sous forme de `String`.
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