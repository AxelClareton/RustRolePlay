use crate::inventaire::Inventaire;
use crate::affichage;

#[derive(Debug, Clone)]
pub struct Coffre {
    pub id: u8,
    pub id_zone: u8,
    pub cle: bool,
    pub ouvert: bool,
    pub description: String,
    pub inventaire: Inventaire,
    pub visible: bool,
}

impl Coffre {
    pub fn ouvrir(&mut self) -> Option<usize>{
        if !self.ouvert {
            let choix = affichage::faire_choix(
                "Ce coffre est fermé voulez-vous utiliser une clé pour l'ouvrir ?",
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
        let obj = self.inventaire.afficher();
        obj
    }

}