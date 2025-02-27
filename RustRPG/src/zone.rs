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

    pub fn compter_coffre(&self) -> usize {
        let mut cpt = 0usize;
        for coffre in self.coffres.clone() {
            if !coffre.vide {
                cpt += 1;
            }
        }
        cpt
    }

    pub fn afficher_coffre(&self) {
        println!("Il y a {} coffres dans la zone", self.compter_coffre());
        println!("Saisir 'q' pour revenir en arrière ou un nombre correspondant au numéro du coffre");
        let mut choix = String::new();
        std::io::stdin().read_line(&mut choix).expect("❌ Erreur de lecture !");
        let choix = choix.trim();
        match choix {
            "q" => {
                println!("Retour en arrière...");
            }
            _ => match choix.parse::<usize>() {
                Ok(index) if index <= self.compter_coffre() => {
                    let coffre = &self.coffres[index-1]; // Récupère le coffre sélectionné
                    coffre.ouvrir();
                }
                _ => {
                    println!("❌ Entrée invalide ! Veuillez entrer un nombre valide.");
                }
            },
        }
    }

}