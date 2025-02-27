use serde::Deserialize;
use crate::coffre::Coffre;

#[derive(Debug, Deserialize, Clone)]
pub struct Connexion {
    pub direction: String,
    pub id_dest: String,
}
#[derive(Debug, Clone)]
pub struct Zone {
    pub id: u8,
    pub nom: String,
    pub description: String,
    pub connection: Vec<Connexion>,
    pub coffres: Vec<Coffre>,
}

impl Zone {
    pub fn afficher_zone(&self) {
        println!("\n🌍 Vous êtes dans la zone : {}", self.nom);
        println!("{}", "-".repeat(30));
        println!("📜 Description : {}", self.description);
        if self.connection.is_empty() {
            println!("❌ Aucune sortie possible.");
        } else {
            println!("🚪 Sorties possibles :");
            for connexion in &self.connection {
                println!("➡️  Vers '{}'", connexion.direction);
            }
        }
        println!("{}", "-".repeat(30));
    }

    pub fn afficher_coffre(&self) {
        println!("Il y a {} coffres dans la zone", self.coffres.len());

    }

}