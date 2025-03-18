use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::inventaire::Inventaire;


// PartieDuCorps
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PartieDuCorps {
    nom: String,
    etat: DateTime<Utc>,
    equipement: crate::inventaire::Inventaire,
}

impl PartieDuCorps {
    pub fn est_saine(&self) -> bool {
        Utc::now() >= self.etat
    }
}

// Personnage
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Personnage {
    pub id: u32,
    pub nom: String,
    pub description: String,
    pub force: u8,
    pub inventaire: crate::inventaire::Inventaire,
    pub parties_du_corps: Vec<PartieDuCorps>,
    pub argent: u32,
}

impl Personnage {
    pub fn sauvegarder_json(&self, fichier: &str) -> io::Result<()> {
        let mut personnages = Personnage::charger_depuis_json(fichier)?;
        personnages.push(self.clone());
        let json = serde_json::to_string_pretty(&personnages)?;
        let mut file = File::create(fichier)?;
        file.write_all(json.as_bytes())
    }

    pub fn charger_depuis_json(fichier: &str) -> io::Result<Vec<Personnage>> {
        let mut file = match File::open(fichier) {
            Ok(file) => file,
            Err(_) => return Ok(vec![]),
        };
        let mut contenu = String::new();
        file.read_to_string(&mut contenu)?;
        let personnages: Vec<Personnage> = serde_json::from_str(&contenu)?;
        Ok(personnages)
    }

    pub fn prochain_id(fichier: &str) -> io::Result<u32> {
        let personnages = Personnage::charger_depuis_json(fichier)?;
        let max_id = personnages.iter().map(|p| p.id).max().unwrap_or(0);
        Ok(max_id + 1)
    }

    pub fn ajouter_argent(&mut self, montant: u32) {
        self.argent += montant;
    }

    pub fn retirer_argent(&mut self, montant: u32) {
        if self.argent < montant {
            println!("Vous n'avez pas assez d'argent !");
        } else {
            self.argent -= montant;
        }
    }
}

// Joueur
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Joueur {
    pub personnage: Personnage,
}

// PNJ et Mobs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PNJ {
    personnage: Personnage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mob {
    personnage: Personnage,
}

impl PNJ {
    pub fn creer_pnj(nom: &str, description: &str) -> io::Result<Self> {
        let prochain_id = Personnage::prochain_id("src/json/pnj.json")?;
        let inventaire = Inventaire { taille: 10, objets: vec![] };
        let inventaire_corps = crate::inventaire::Inventaire { taille: 1, objets: vec![] };
        let parties_du_corps = vec![
            PartieDuCorps { nom: "Bras droit".to_string(), etat: Utc::now(), equipement: inventaire_corps.clone() },
            PartieDuCorps { nom: "Jambe gauche".to_string(), etat: Utc::now(), equipement: inventaire_corps.clone() },
            PartieDuCorps { nom: "Tête".to_string(), etat: Utc::now(), equipement: inventaire_corps.clone() },
            PartieDuCorps { nom: "Torse".to_string(), etat: Utc::now(), equipement: inventaire_corps.clone() },
            PartieDuCorps { nom: "Bras gauche".to_string(), etat: Utc::now(), equipement: inventaire_corps.clone() },
            PartieDuCorps { nom: "Jambe droite".to_string(), etat: Utc::now(), equipement: inventaire_corps.clone() },
        ];

        let mut rng: ThreadRng = rand::thread_rng();
        let valeur = rng.random_range(80..120);
        let valeur2 = rng.random_range(0..20);

        println!("Valeur: {}", valeur);

        let personnage = Personnage {
            id: prochain_id,
            nom: nom.to_string(),
            description: description.to_string(),
            force: valeur,
            inventaire,
            parties_du_corps,
            argent: valeur2,
        };

        personnage.sauvegarder_json("src/json/pnj.json")?;

        Ok(PNJ { personnage })
    }

    pub fn charger_pnj(fichier: &str) -> io::Result<Vec<Personnage>> {
        let mut file = match File::open(fichier) {
            Ok(file) => file,
            Err(_) => return Ok(vec![]),
        };
        let mut contenu = String::new();
        file.read_to_string(&mut contenu)?;
        let PNJ: Vec<Personnage> = serde_json::from_str(&contenu)?;
        Ok(PNJ)
    }
}

impl Mob {
    pub fn creer_mob(nom: &str, description: &str) -> io::Result<Self> {
        let prochain_id = Personnage::prochain_id("src/json/mob.json")?;
        let inventaire = Inventaire { taille: 10, objets: vec![] };
        let inventaire_corps = crate::inventaire::Inventaire { taille: 1, objets: vec![] };
        let parties_du_corps = vec![
            PartieDuCorps { nom: "Bras droit".to_string(), etat: Utc::now(), equipement: inventaire_corps.clone() },
            PartieDuCorps { nom: "Jambe gauche".to_string(), etat: Utc::now(), equipement: inventaire_corps.clone() },
            PartieDuCorps { nom: "Tête".to_string(), etat: Utc::now(), equipement: inventaire_corps.clone() },
            PartieDuCorps { nom: "Torse".to_string(), etat: Utc::now(), equipement: inventaire_corps.clone() },
            PartieDuCorps { nom: "Bras gauche".to_string(), etat: Utc::now(), equipement: inventaire_corps.clone() },
            PartieDuCorps { nom: "Jambe droite".to_string(), etat: Utc::now(), equipement: inventaire_corps.clone() },
        ];

        let mut rng: ThreadRng = rand::thread_rng();
        let valeur = rng.random_range(80..120);
        let valeur2 = rng.random_range(0..20);

        println!("Valeur: {}", valeur);

        let personnage = Personnage {
            id: prochain_id,
            nom: nom.to_string(),
            description: description.to_string(),
            force: valeur,
            inventaire,
            parties_du_corps,
            argent: valeur2,
        };

        personnage.sauvegarder_json("src/json/mob.json")?;

        Ok(Mob { personnage })
    }

    pub fn charger_mob(fichier: &str) -> io::Result<Vec<Personnage>> {
        let mut file = match File::open(fichier) {
            Ok(file) => file,
            Err(_) => return Ok(vec![]),
        };
        let mut contenu = String::new();
        file.read_to_string(&mut contenu)?;
        let mob: Vec<Personnage> = serde_json::from_str(&contenu)?;
        Ok(mob)
    }
}


impl Joueur {
    pub fn creer_joueur(nom: &str, description: &str) -> io::Result<Self> {
        let prochain_id = Personnage::prochain_id("src/json/personnage.json")?;
        let inventaire = Inventaire { taille: 10, objets: vec![] };
        let inventaire_corps = crate::inventaire::Inventaire { taille: 1, objets: vec![] };
        let parties_du_corps = vec![
            PartieDuCorps { nom: "Bras droit".to_string(), etat: Utc::now(), equipement: inventaire_corps.clone() },
            PartieDuCorps { nom: "Jambe gauche".to_string(), etat: Utc::now(), equipement: inventaire_corps.clone() },
            PartieDuCorps { nom: "Tête".to_string(), etat: Utc::now(), equipement: inventaire_corps.clone() },
            PartieDuCorps { nom: "Torse".to_string(), etat: Utc::now(), equipement: inventaire_corps.clone() },
            PartieDuCorps { nom: "Bras gauche".to_string(), etat: Utc::now(), equipement: inventaire_corps.clone() },
            PartieDuCorps { nom: "Jambe droite".to_string(), etat: Utc::now(), equipement: inventaire_corps.clone() },
        ];

        let mut rng: ThreadRng = rand::thread_rng();
        let valeur = rng.random_range(80..120);
        let valeur2 = rng.random_range(0..20);

        println!("Valeur: {}", valeur);

        let personnage = Personnage {
            id: prochain_id,
            nom: nom.to_string(),
            description: description.to_string(),
            force: valeur,
            inventaire,
            parties_du_corps,
            argent: valeur2,
        };

        personnage.sauvegarder_json("src/json/personnage.json")?;

        Ok(Joueur { personnage })
    }

    pub fn charger_joueur(fichier: &str) -> io::Result<Vec<Personnage>> {
        let mut file = match File::open(fichier) {
            Ok(file) => file,
            Err(_) => return Ok(vec![]),
        };
        let mut contenu = String::new();
        file.read_to_string(&mut contenu)?;
        let personnages: Vec<Personnage> = serde_json::from_str(&contenu)?;
        Ok(personnages)
    }
}
