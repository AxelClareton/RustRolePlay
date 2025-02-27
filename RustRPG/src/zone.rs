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