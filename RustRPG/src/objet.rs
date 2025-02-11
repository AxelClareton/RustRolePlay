use crate::personnage::Personnage;

#[derive(Debug, Clone)]
pub struct Effet {
    pub type_effet: String,  // Ex: "soin", "bonus attaque"
    pub valeur: u8,
}

#[derive(Debug, Clone)]
pub struct Objet {
    pub nom: String,
    pub description: String,
    pub type_objet: String, // "clé", "arme", "potion"
    pub est_ouvert: bool, // Pour des objets comme les portes ou coffres
    pub est_ferme: bool, // Pour une porte fermée à clé
}

impl Objet {
    pub fn ouvrir(&mut self) {
        if self.est_ferme {
            println!("La porte est fermée à clé.");
        } else {
            self.est_ouvert = true;
            println!("Vous avez ouvert {}.", self.nom);
        }
    }

    pub fn fermer(&mut self) {
        self.est_ouvert = false;
        println!("Vous avez fermé {}.", self.nom);
    }

    pub fn utiliser(&mut self, personnage: &mut Personnage) {
        match self.type_objet.as_str() {
            "clé" => {
                println!("Vous utilisez la clé sur la porte.");
                self.est_ferme = false; // Déverrouille la porte
            },
            "potion" => {
                println!("Vous utilisez une potion de soin.");
                personnage.points_de_vie += 20; // Exemple de soin
            },
            _ => println!("Cet objet ne peut pas être utilisé ici."),
        }
    }
}
