use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::RwLock;

// Définition de la structure Objet
#[derive(Debug, Clone)]
pub struct Objet {
    pub id: u8,
    pub nom: String,
}

// Stockage global des objets
pub static OBJETS_DISPONIBLES: Lazy<RwLock<HashMap<u8, Objet>>> = Lazy::new(|| RwLock::new(HashMap::new()));

pub fn ajouter_objet(id: u8, nom: String) {
    let mut objets = OBJETS_DISPONIBLES.write().unwrap();
    objets.insert(id, Objet { id, nom });
}