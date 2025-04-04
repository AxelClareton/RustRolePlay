use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::error::Error;
use serde_json::Value;
use coffre::Coffre;
use crate::{coffre, inventaire, zone};
use zone::Zone;
use zone::Connexion;
use inventaire::{Inventaire, ObjetInventaire};
use crate::objet::ajouter_objet;
// Structures pour lire le JSON

#[derive(Debug, Deserialize)]
struct ZoneTemporaire {
    #[serde(rename = "id")]
    id_texte: String,
    #[serde(rename = "nom")]
    nom: String,
    #[serde(rename = "desc")]
    description: String,
    #[serde(rename = "prix")]
    prix: String,
    #[serde(rename = "ouvert")]
    ouvert: String,
    connection: Vec<Connexion>,
    #[serde(rename = "objet_zone")]
    objet_zone: Inventaire,
}


#[derive(Debug, Deserialize)]
struct ObjetInventaireTemporaire {
    nombre : u8,
    objet_id: u8,
}

#[derive(Debug, Deserialize)]
struct ObjetTemporaire {
    #[serde(rename = "id")]
    id_texte : String,
    #[serde(rename = "nom")]
    nom: String,
}
#[derive(Debug, Deserialize)]
struct InventaireTemporaire {
    #[serde(rename = "taille")]
    taille_texte: String,
    objets: Vec<ObjetInventaireTemporaire>,
}

#[derive(Debug, Deserialize)]
struct CoffreTemporaire {
    #[serde(rename = "id")]
    id_texte: String,
    #[serde(rename = "id_zone")]
    id_zone_texte: String,
    #[serde(rename = "desc")]
    description: String,
    #[serde(rename = "cle")]
    cle: String,
    #[serde(rename = "ouvert")]
    ouvert: String,
    #[serde(rename = "visible")]
    visible: String,
    inventaire: Vec<InventaireTemporaire>, // Le JSON utilise un tableau
}
pub fn charger_json(chemin: &str)->Result<String, Box<dyn Error>>{
    let contenu = fs::read_to_string(chemin)?;
    Ok(contenu)
}

// Fonction pour charger les zones
pub fn charger_zones() -> Result<Vec<Zone>, Box<dyn Error>> {
    let coffres_totaux: HashMap<u8, Vec<Coffre>> = charger_coffres().expect("⚠️ Impossible de charger les coffres !");
    let contenu = charger_json("src/json/zone.json")?;
    let zones_temp: Vec<ZoneTemporaire> = serde_json::from_str::<Vec<Value>>(&contenu)?
    .into_iter()
    .filter(|zone| zone["type"] == "zone")
    .filter_map(|zone| serde_json::from_value(zone).ok())
    .collect();

    let mut map_temp = HashMap::new();
    for zt in zones_temp {
        map_temp.insert(zt.id_texte.clone(), zt);
    }

    let mut zones_finales = Vec::new();
    for (_, zone_temp) in &map_temp {
        let id_numerique = zone_temp.id_texte.parse::<u8>()?;
        let coffre_zone: Vec<Coffre> = coffres_totaux.get(&id_numerique).cloned().unwrap_or_else(Vec::new);
        let prix_zone = zone_temp.prix.parse::<u8>()?;
        let mut ouvert = true;
        if zone_temp.ouvert == "false" {
            ouvert = false;
        }

        let inventaire = Inventaire {
            taille : 255,
            objets : Vec::new(),
        };
        let mut zone_finale = Zone {
            id: id_numerique,
            nom: zone_temp.nom.clone(),
            prix: prix_zone,
            ouvert: ouvert,
            description: zone_temp.description.clone(),
            connection: zone_temp.connection.clone(),
            coffres: coffre_zone,
            mobs: Vec::new(),
            objet_zone : inventaire,
        };
        zones_finales.push(zone_finale);
    }

    Ok(zones_finales)
}

pub fn charger_coffres() -> Result<HashMap<u8, Vec<Coffre>>, Box<dyn Error>> {
    let contenu = charger_json("src/json/coffre.json")?;
    let coffres_temp: Vec<CoffreTemporaire> = serde_json::from_str(&contenu)?;
    let mut coffre_finales: HashMap<u8, Vec<Coffre>> = HashMap::new();

    for coffre in coffres_temp {
        let id_zone = coffre.id_zone_texte.parse::<u8>()?;
        let id = coffre.id_texte.parse::<u8>()?;
        let mut cle = true;
        if coffre.ouvert == "false" {
            cle = false;
        }
        let mut ouvert = true;
        if coffre.ouvert == "false" {
            ouvert = false;
        }
        let mut visible = true;
        if coffre.visible == "false" {
            visible = false;
        }
        // Récupérer le premier élément de la liste `inventaire`
        let inventaire_temp = coffre.inventaire.get(0).ok_or("Inventaire vide")?;

        let inventaire = Inventaire {
            taille: inventaire_temp.taille_texte.parse::<u8>()?, // Conversion de taille
            objets: inventaire_temp.objets.iter().map(|o| ObjetInventaire {
                nombre: o.nombre,
                objet_id: o.objet_id,
            }).collect(), // Mapper les objets avec leurs quantités
        };

        let c = Coffre {
            id,
            id_zone,
            description: coffre.description.clone(),
            inventaire,
            cle:cle,
            ouvert: ouvert,
            visible : visible,
        };

        coffre_finales.entry(id_zone).or_insert(Vec::new()).push(c);
    }
    Ok(coffre_finales)
}

pub fn charger_objets() -> Result<(), Box<dyn Error>> {
    let contenu = charger_json("src/json/objet.json")?;
    let objets_temp: Vec<ObjetTemporaire> = serde_json::from_str(&contenu)?;
    for objet in objets_temp {
        let id = objet.id_texte.parse::<u8>()?;
        let nom = objet.nom;
        ajouter_objet(id, nom);
    }
    Ok(())
}