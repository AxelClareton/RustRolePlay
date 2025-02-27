use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Inventaire {
    pub taille: u8,
    pub objets: Vec<u8>,
}