use crate::inventaire::Inventaire;

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
            println!("Ce coffre est fermé voulez-vous utiliser une clé pour l'ouvrir ?");
            let mut choix = String::new();
            std::io::stdin().read_line(&mut choix).expect("❌ Erreur de lecture !");
            let choix = choix.trim();
            match choix {
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