use std::io;
use crate::personnage::{Personnage, Race, Classe};

pub fn menu_principal() {
    println!("\n--- Menu Principal ---");
    println!("1. Créer un personnage");
    println!("2. Quitter");

    let mut choix = String::new();
    io::stdin().read_line(&mut choix).expect("Erreur de lecture");
    let choix = choix.trim();

    match choix {
        "1" => {
            let joueur = creer_personnage();
            println!("Votre personnage : {:?}", joueur);
        }
        "2" => {
            println!("À bientôt !");
        }
        _ => println!("Choix invalide, veuillez réessayer."),
    }
}

pub fn creer_personnage() -> Personnage {
    println!("Entrez le nom de votre personnage :");
    let mut nom = String::new();
    io::stdin().read_line(&mut nom).expect("Erreur de lecture");
    let nom = nom.trim().to_string();

    println!("Choisissez une race :");
    println!("1. Humain\n2. Elfe\n3. Nain\n4. Orc\n5. Centaur");
    let race = match lire_choix() {
        1 => Race::Humain,
        2 => Race::Elfe,
        3 => Race::Nain,
        4 => Race::Orc,
        _ => Race::Centaur,
    };

    println!("Choisissez une classe :");
    println!("1. Guerrier\n2. Mage\n3. Archer\n4. Voleur\n5. Paladin");
    let classe = match lire_choix() {
        1 => Classe::Guerrier,
        2 => Classe::Mage,
        3 => Classe::Archer,
        4 => Classe::Voleur,
        _ => Classe::Paladin,
    };

    // Stats de base
    let mut points_de_vie: u16 = 200;
    let mut attaque: u8 = 30;
    let mut defense: u8 = 20;
    let mut agilite: u8 = 30;

    // Appliquer les modificateurs
    let (mod_pv, mod_def, mod_attaque, mod_agi) = race.modificateurs();
    points_de_vie = ((points_de_vie as i16 + mod_pv).max(0)) as u16;
    attaque = attaque.saturating_add_signed(mod_attaque);
    defense = defense.saturating_add_signed(mod_def);
    agilite = agilite.saturating_add_signed(mod_agi);    

    let (mod_pv, mod_def, mod_attaque, mod_agi) = classe.modificateurs();
    points_de_vie = ((points_de_vie as i16 + mod_pv).max(0)) as u16;
    attaque = attaque.saturating_add_signed(mod_attaque);
    defense = defense.saturating_add_signed(mod_def);
    agilite = agilite.saturating_add_signed(mod_agi);
    
    let membres = Personnage::initialiser_membres(&race);

    Personnage {
        nom,
        niveau: 1,
        points_de_vie,
        attaque,
        defense,
        agilite,
        race,
        classe,
        membres,
    }
}


fn lire_choix() -> u8 {
    let mut choix = String::new();
    io::stdin().read_line(&mut choix).expect("Erreur de lecture");
    choix.trim().parse().unwrap_or(1)
}
