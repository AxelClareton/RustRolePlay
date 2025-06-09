use crate::inventaire::Inventaire;
use crate::affichage;
use crate::zone::Zone;
use crate::personnage::PNJ;
use crate::personnage::Personnage;

#[derive(Debug, Clone)]
pub struct Coffre {
    pub _id: u8,
    pub _id_zone: u8,
    pub ouvert: bool,
    pub _description: String,
    pub inventaire: Inventaire,
    pub visible: bool,
}

impl Coffre {
    pub fn ouvrir(&mut self, zone: &Zone, joueur: &mut Personnage, pnjs: &Vec<PNJ>) -> Option<()>{
        if !self.ouvert {
            let choix = affichage::faire_choix(
                "Ce coffre est fermé voulez-vous utiliser une clé pour l'ouvrir ? (oui/non)",
                &vec!["oui".to_string(), "non".to_string()]
            );
            match choix.as_str() {
                "oui" => {
                    if !joueur.inventaire.retirer_par_id(12) {
                        affichage::notifier(zone, "❌ Vous n'avez pas de clé !", pnjs);
                        return None;
                    }
                    self.ouvert = true;
                    affichage::notifier(zone, "🔑 Vous utilisez une clé et ouvrez le coffre !", pnjs);
                }
                _ => {
                    println!("Le coffre reste verrouillé !");
                    return None;
                }
            }
        }
        println!("Ouverture du coffre ! ");
        Some(())
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
            ouvert: false,
            _description: "Un coffre".to_string(),
            inventaire: Inventaire { taille: 1, objets: vec![] },
            visible: true,
        };
        assert_eq!(coffre._id, 1);
        assert!(!coffre.ouvert);
    }
}