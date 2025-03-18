use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::RwLock;
use crate::objetType::ObjetType;

// Définition de la structure Objet
#[derive(Debug, Clone)]
pub struct Objet {
    pub id: u8,
    pub nom: String,
    pub poids: u32,
    pub objet_type: ObjetType,
}

// Stockage global des objets
pub static OBJETS_DISPONIBLES: Lazy<RwLock<HashMap<u8, Objet>>> = Lazy::new(|| RwLock::new(HashMap::new()));

pub fn ajouter_objet(id: u8, nom: String, poids: u32, objet_type: ObjetType) {
    let mut objets = OBJETS_DISPONIBLES.write().unwrap();
    objets.insert(id, Objet { id, nom, poids, objet_type });
}