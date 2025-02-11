use crate::objet::Objet;  
use crate::personnage::Personnage;


#[derive(Debug, Clone)] // Ajout de Clone pour pouvoir cloner les zones dans les liaisons
pub struct Zone {
    pub id: u8,
    pub nom: String,
    pub description: String,
    pub obstacles: Vec<Obstacle>,
    pub ennemis: Vec<Ennemi>,
    pub objets: Vec<Objet>,
    pub liaisons: Vec<Zone>, // Zones accessibles depuis cette zone
}

#[derive(Debug, Clone)]
pub struct Obstacle {
    pub nom: String,
    pub description: String,
    pub difficulte: u8, // difficulté pour le franchir
}

#[derive(Debug, Clone)]
pub enum Comportement {
    Pacifique,
    Hostile,
    Fuite, // Peut fuir ou se défendre s'il est blessé
}

#[derive(Debug, Clone)]
pub struct Ennemi {
    pub nom: String,
    pub points_de_vie: u16,
    pub attaque: u8,
    pub comportement: Comportement, // Comportement de l'ennemi
}

impl Ennemi {
    pub fn changer_comportement(&mut self, comportement: Comportement) {
        self.comportement = comportement;
    }

    // Attaque de l'ennemi
    pub fn attaquer(&self, personnage: &mut Personnage) {
        if let Comportement::Hostile = self.comportement {
            personnage.points_de_vie = personnage.points_de_vie.saturating_sub(self.attaque.into());
            println!("L'ennemi {} vous attaque et vous inflige {} points de dégâts", self.nom, self.attaque);
        }
    }
}


pub fn temps_deplacement(distance: u16, agilité: u8) -> u16 {
    // Calcul du temps de déplacement. Plus l'agilité est faible, plus il faut de temps.
    let temps_base = distance * 10; // Temps de base sans agilité
    let penalite_agilite = (100 - agilité) as u16; // Plus l'agilité est faible, plus la pénalité est élevée

    temps_base + penalite_agilite
}


impl Zone {
    pub fn afficher_details(&self) {
        println!("Vous êtes dans la zone : {}", self.nom);
        println!("{}", self.description);
    }

    pub fn afficher_objets(&self) {
        if !self.objets.is_empty() {
            println!("Objets présents dans la zone :");
            for objet in &self.objets {
                println!("{}", objet.nom);
            }
        } else {
            println!("Aucun objet dans cette zone.");
        }
    }
}