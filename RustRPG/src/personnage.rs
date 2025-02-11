use crate::monde::{Zone, Obstacle, Ennemi, temps_deplacement};

#[derive(Debug)]
pub enum Race {
    Humain,
    Elfe,
    Nain,
    Orc,
    Centaur,
}

#[derive(Debug)]
pub enum Classe {
    Guerrier,
    Mage,
    Archer,
    Voleur,
    Paladin,
}

#[derive(Debug, Clone)]
pub enum Membre {
    Sain { nom: String },
    Blessé { nom: String, gravité: u8 },  // Gravité de la blessure
    Perdu { nom: String },
}


#[derive(Debug)]
pub struct Personnage {
    pub nom: String,
    pub niveau: u8,
    pub points_de_vie: u16,
    pub attaque: u8,
    pub defense: u8,
    pub agilite: u8,
    pub race: Race,
    pub classe: Classe,
    pub membres: Vec<Membre>,
}

impl Personnage {
    pub fn attaquer(&self, cible: &mut Personnage) {
        let degats = self.attaque.saturating_sub(cible.defense);
        cible.recevoir_degats(degats);
        println!("{} attaque {} et inflige {} dégâts!", self.nom, cible.nom, degats);
    }

    pub fn recevoir_degats(&mut self, degats: u8) {
        self.points_de_vie = self.points_de_vie.saturating_sub(degats.into());
        if self.points_de_vie == 0 {
            println!("{} est vaincu!", self.nom);
        }
    }

     // Méthode pour appliquer des dégâts à un membre (perdre un membre)
     pub fn perdre_membre(&mut self, index: usize) {
        if let Some(membre) = self.membres.get_mut(index) {
            match membre {
                Membre::Sain { nom } | Membre::Blessé { nom, .. } => {
                    *membre = Membre::Perdu { nom: nom.clone() }; // Cloner le nom et créer la variante Perdu
                }
                Membre::Perdu { .. } => {
                    println!("Ce membre est déjà perdu !");
                }
            }
        }
    }
    

    // Méthode pour guérir un membre blessé
    pub fn guerir_membre(&mut self, index: usize) {
        if let Some(membre) = self.membres.get_mut(index) {
            match membre {
                Membre::Perdu { nom } => {
                    *membre = Membre::Sain { nom: nom.clone() }; // Cloner le nom et créer la variante Sain
                }
                Membre::Sain { .. } | Membre::Blessé { .. } => {
                    println!("Ce membre est déjà sain !");
                }
            }
        }
    }
    

    pub fn initialiser_membres(race: &Race) -> Vec<Membre> {
        match race {
            Race::Humain => vec![
                Membre::Sain { nom: "Bras Droit".to_string() },
                Membre::Sain { nom: "Bras Gauche".to_string() },
                Membre::Sain { nom: "Jambe Droite".to_string() },
                Membre::Sain { nom: "Jambe Gauche".to_string() },
            ],
            Race::Elfe => vec![
                Membre::Sain { nom: "Bras Droit".to_string() },
                Membre::Sain { nom: "Bras Gauche".to_string() },
                Membre::Sain { nom: "Jambe Droite".to_string() },
                Membre::Sain { nom: "Jambe Gauche".to_string() },
            ],
            Race::Nain => vec![
                Membre::Sain { nom: "Bras Droit".to_string() },
                Membre::Sain { nom: "Bras Gauche".to_string() },
                Membre::Sain { nom: "Jambe Droite".to_string() },
                Membre::Sain { nom: "Jambe Gauche".to_string() },
            ],
            Race::Orc => vec![
                Membre::Sain { nom: "Bras Droit".to_string() },
                Membre::Sain { nom: "Bras Gauche".to_string() },
                Membre::Sain { nom: "Jambe Droite".to_string() },
                Membre::Sain { nom: "Jambe Gauche".to_string() },
            ],
            Race::Centaur => vec![
                Membre::Sain { nom: "Bras Droit".to_string() },
                Membre::Sain { nom: "Bras Gauche".to_string() },
                Membre::Sain { nom: "Jambe Droite Avant".to_string() },
                Membre::Sain { nom: "Jambe Gauche Avant".to_string() },
                Membre::Sain { nom: "Jambe Droite Arrière".to_string() },
                Membre::Sain { nom: "Jambe Gauche Arrière".to_string() },
            ],
        }
    }
}

// Déplacement de zone à zone
pub fn deplacer(personnage: &mut Personnage, zone_source: &Zone, zone_destination: &Zone) {
    let distance = 50; // Par exemple, la distance entre les zones est de 50 mètres
    let temps = temps_deplacement(distance, personnage.agilite);
    
    println!("{} se déplace de '{}' à '{}'.", personnage.nom, zone_source.nom, zone_destination.nom);
    println!("Temps de déplacement : {} unités de temps", temps);
    personnage.points_de_vie = personnage.points_de_vie.saturating_sub(temps / 2); // Par exemple, chaque unité de temps coûte des PV
}

pub fn franchir_obstacle(personnage: &mut Personnage, obstacle: &Obstacle) -> bool {
    let reussite = personnage.agilite >= obstacle.difficulte;

    if reussite {
        println!("{} franchit l'obstacle '{}'.", personnage.nom, obstacle.nom);
        true
    } else {
        println!("{} échoue à franchir l'obstacle '{}'.", personnage.nom, obstacle.nom);
        personnage.points_de_vie = personnage.points_de_vie.saturating_sub(5); // Perte de PV si échec
        false
    }
}





impl Classe {
    pub fn modificateurs(&self) -> (i16, i8, i8, i8) {
        match self {
            Classe::Guerrier => (25, 5, 5, -5), 
            Classe::Mage => (-20, -5, 20, 0),   
            Classe::Archer => (-15, -5, 10, 10),  
            Classe::Voleur => (-15, -10, 5, 20),  
            Classe::Paladin => (30, 10, 5, -5),
        }
    }
}


impl Race {
    pub fn modificateurs(&self) -> (i16, i8, i8, i8) {
        match self {
            Race::Humain => (0, 0, 0, 0),  
            Race::Elfe => (-20, -5, 5, 5),   
            Race::Nain => (35, 5, -5, -10), 
            Race::Orc => (50, 5, 10, -20),
            Race::Centaur => (50, 15, 15, 20),
        }
    }
}