use serde::Deserialize;
use crate::coffre::Coffre;
use crate::inventaire::Inventaire;
use crate::affichage::notifier;
use crate::personnage::Mob;
use crate::personnage::PNJ;
use crate::affichage::ajouter_notification;

#[derive(Debug, Deserialize, Clone)]
pub struct Connexion {
    pub direction: String,
    pub id_dest: String,
}

#[derive(Debug, Clone)]
pub struct Zone {
    pub id: u8,
    pub nom: String,
    pub ouvert: bool,
    pub description: String,
    pub connection: Vec<Connexion>,
    pub coffres: Vec<Coffre>,
    pub objet_zone : Inventaire,
    pub mob_present: bool,
    pub prix: u32,
}

impl Zone {
    pub fn compter_coffre(&self) -> usize {
        let mut cpt = 0usize;
        for coffre in self.coffres.clone() {
            if coffre.visible {
                cpt += 1;
            }
        }
        cpt
    }

    pub fn fouiller_zone(&mut self, tous_les_pnjs: &[PNJ]) {
        let mut cpt :u8 = 0;
        for coffre in &mut self.coffres {
            if !coffre.visible {
                cpt += 1;
                coffre.visible = true;
            }
        }
        ajouter_notification(&("Vous avez trouvé ".to_owned() + &cpt.to_string() + " coffre(s) ."));
    }

    pub fn supprimer_coffre(&mut self, num : usize) {
        self.coffres.remove(num);
    }
}