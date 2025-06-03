use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::RwLock;
use std::str::FromStr;

// Définition de la structure Objet
#[derive(Debug, Clone)]
pub struct Objet {
    pub id: u8,
    pub nom: String,
    pub poids: u32,
    pub prix: u32,
    pub objet_type: TypeObjet,
}

// Définition des différents types d'objets
#[derive(Debug, Clone)]
pub enum TypeObjet {
    Arme {
        degats: u32,
        proba_degats: f32,
        frequence_degats: u8
    },
    Equipement {
        protection: u8,
        emplacement: Emplacement,
    },
    Soin {
        vie: u32,
        emplacement: Emplacement,
    },
}

#[derive(Debug, Clone)]
pub enum Emplacement {
    Bras,
    Jambe ,
    Tete,
    Torse,
    Tous,
}

impl FromStr for Emplacement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Bras" => Ok(Emplacement::Bras),
            "Jambe" => Ok(Emplacement::Jambe),
            "Tete" => Ok(Emplacement::Tete),
            "Torse" => Ok(Emplacement::Torse),
            "Tous" => Ok(Emplacement::Tous),
            _ => Err(format!("Emplacement inconnu : {}", s)),
        }
    }
}

// Stockage global des objets
pub static OBJETS_DISPONIBLES: Lazy<RwLock<HashMap<u8, Objet>>> = Lazy::new(|| RwLock::new(HashMap::new()));

pub fn ajouter_objet(id: u8, nom: String, poids: u32, prix: u32, objet_type: TypeObjet) {
    let mut objets = OBJETS_DISPONIBLES.write().unwrap();
    objets.insert(id, Objet { id, nom, poids, prix, objet_type });
}