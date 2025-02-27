use crate::inventaire::Inventaire;

#[derive(Debug, Clone)]
pub struct Coffre {
    pub id: u8,
    pub id_zone: u8,
    pub description: String,
    pub inventaire: Inventaire,
    pub vide: bool,
}

impl Coffre {
    pub fn ouvrir(&self){
        println!("Ouverture du coffre ! ");
        self.inventaire.afficher();
    }
}