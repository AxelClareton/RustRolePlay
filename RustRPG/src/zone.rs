use serde::Deserialize;
use crate::coffre::Coffre;
use crate::inventaire::Inventaire;
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

    pub fn fouiller_zone(&mut self) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coffre::Coffre;

    #[test]
    fn test_compter_coffre() {
        let coffres = vec![
            Coffre { _id: 1, _id_zone: 1, ouvert: true, _description: "C1".to_string(), inventaire: Inventaire { taille: 1, objets: vec![] }, visible: true },
            Coffre { _id: 2, _id_zone: 1, ouvert: true, _description: "C2".to_string(), inventaire: Inventaire { taille: 1, objets: vec![] }, visible: false },
        ];
        let zone = Zone {
            id: 1,
            nom: "TestZone".to_string(),
            ouvert: true,
            description: "desc".to_string(),
            connection: vec![],
            coffres,
            objet_zone: Inventaire { taille: 1, objets: vec![] },
            mob_present: false,
            prix: 0,
        };
        assert_eq!(zone.compter_coffre(), 1);
    }
}