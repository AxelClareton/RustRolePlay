use rand::seq::SliceRandom;
use rand::prelude::IteratorRandom;
use serde::Deserialize;
use std::fs;
use crate::coffre::Coffre;
use crate::personnage::Mob;
use crate::inventaire::Inventaire;

#[derive(Debug, Deserialize, Clone)]
pub struct Connexion {
    pub direction: String,
    pub id_dest: String,
}
#[derive(Debug, Clone)]
pub struct Zone {
    pub id: u8,
    pub nom: String,
    pub prix: u8,
    pub ouvert: bool,
    pub description: String,
    pub connection: Vec<Connexion>,
    pub coffres: Vec<Coffre>,
    pub mobs: Vec<Mob>,
    pub objet_zone : Inventaire,
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
        println!("Il y a {} coffres dans la zone", self.compter_coffre());
        println!("{}", "-".repeat(30));
    }

    pub fn generer_mobs(&self) -> Vec<Mob> {
        println!("🦇 Génération des mobs en cours...");

        // Charger les mobs disponibles depuis un fichier JSON
        let mobs_disponibles = match Mob::charger_mob("src/json/mob.json") {
            Ok(mobs) => mobs,
            Err(e) => {
                println!("❌ Erreur lors du chargement des mobs : {}", e);
                return vec![]; // Retourner un vecteur vide en cas d'erreur
            }
        };
        
        // Mélanger la liste et prendre un nombre aléatoire de mobs
        let mut rng = rand::thread_rng();
        let nombre_mobs = (1..=3).choose(&mut rng).unwrap_or(1); // Choisir entre 1 et 3 mobs
        let mobs: Vec<Mob> = mobs_disponibles
            .iter()
            .choose_multiple(&mut rng, nombre_mobs) // Choisir plusieurs mobs aléatoires
            .into_iter()
            .map(|p| Mob { personnage: p.clone() }) // Créer un Mob à partir de chaque Personnage
            .collect(); // Collecter les Mobs dans un vecteur

        println!("✅ {} mob(s) généré(s) !", mobs.len());
        //print mobs
        for mob in &mobs {
            println!("🦇 {}", mob.personnage.nom);
        }
        mobs // Retourner le vecteur de Mobs générés
    }

    pub fn compter_coffre(&self) -> usize {
        let mut cpt = 0usize;
        for coffre in self.coffres.clone() {
            if coffre.visible {
                cpt += 1;
            }
        }
        cpt
    }

    pub fn afficher_coffre(&mut self) {
        let nbr = self.compter_coffre();
        println!("Il y a {} coffres dans la zone", nbr);
        if nbr == 0 {
            return
        }
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
                    let coffre = &mut self.coffres[index-1]; // Récupère le coffre sélectionné
                    //coffre.ouvrir();
                    if coffre.ouvert && coffre.inventaire.objets.is_empty() {
                        self.coffres.remove(index-1);
                    }
                }
                _ => {
                    println!("❌ Entrée invalide ! Veuillez entrer un nombre valide.");
                }
            },
        }
    }

    pub fn fouiller_zone(&mut self) {
        let mut cpt :u8 = 0;
        for coffre in &mut self.coffres {
            if !coffre.visible {
                cpt += 1;
                coffre.visible = true;
            }
        }
        println!("Félicitation vous avez trouvé {} coffre(s) !", cpt);
    }

}