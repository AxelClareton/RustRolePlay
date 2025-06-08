use crate::inventaire::Inventaire;
use crate::affichage;

#[derive(Debug, Clone)]
pub struct Coffre {
    pub _id: u8,
    pub _id_zone: u8,
    pub _cle: bool,
    pub ouvert: bool,
    pub _description: String,
    pub inventaire: Inventaire,
    pub visible: bool,
}

impl Coffre {
    pub fn ouvrir(&mut self, zone: &crate::zone::Zone, pnjs: &Vec<crate::personnage::PNJ>) -> Option<usize>{
        if !self.ouvert {
            let choix = affichage::faire_choix(
                "Ce coffre est fermé voulez-vous utiliser une clé pour l'ouvrir ? (oui/non)",
                &vec!["oui".to_string(), "non".to_string()]
            );
            match choix.as_str() {
                "oui" => {
                    self.ouvert = true;
                    //déduire le prix
                }
                _ => {
                    println!("Coffre non ouvert");
                    return None;
                }
            }
        }
        println!("Ouverture du coffre ! ");
        let obj = self.inventaire.afficher(false, zone, pnjs);
        obj
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coffre_creation() {
        let coffre = Coffre {
            _id: 1,
            _id_zone: 1,
            _cle: false,
            ouvert: false,
            _description: "Un coffre".to_string(),
            inventaire: Inventaire { taille: 1, objets: vec![] },
            visible: true,
        };
        assert_eq!(coffre._id, 1);
        assert!(!coffre.ouvert);
    }
}