use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::error::Error;
use rand::Rng;
use rand::prelude::IndexedRandom;
use serde_json::Value;
use coffre::Coffre;
use crate::{coffre, inventaire, zone};
use zone::Zone;
use zone::Connexion;
use inventaire::{Inventaire};
use crate::objet::{ajouter_objet, TypeObjet, OBJETS_DISPONIBLES};

/// Structure temporaire représentant une zone chargée depuis un JSON.
///
/// Les champs sont sérialisés/désérialisés avec des noms spécifiques via `serde`.
#[derive(Debug, Deserialize)]
struct ZoneTemporaire {
    /// Identifiant de la zone (texte).
    #[serde(rename = "id")]
    id_texte: String,
    /// Nom de la zone.
    #[serde(rename = "nom")]
    nom: String,
    /// Description de la zone.
    #[serde(rename = "desc")]
    description: String,
    /// Prix associé à la zone (texte).
    #[serde(rename = "prix")]
    prix: String,
    #[serde(rename = "ouvert")]
    /// Indicateur si la zone est ouverte (texte "true" ou "false").
    ouvert: String,
    /// Liste des connexions vers d'autres zones.
    connection: Vec<Connexion>,
    /// Inventaire d'objets présents dans la zone.
    #[serde(rename = "objet_zone")]
    _objet_zone: Inventaire,
    /// Indique si des mobs sont présents dans la zone.
    #[serde(default)]
    mob_present: bool,
}

/// Structure temporaire représentant un objet dans un inventaire chargé depuis JSON.
#[derive(Debug, Deserialize)]
struct ObjetInventaireTemporaire {
    /// Quantité de l'objet.
    _nombre : u8,
    /// Identifiant de l'objet.
    _objet_id: u8,
}

/// Structure temporaire représentant un inventaire dans le JSON.
#[derive(Debug, Deserialize)]
struct InventaireTemporaire {
    /// Taille de l'inventaire (texte).
    _taille_texte: String,
    /// Liste des objets présents dans l'inventaire.
    _objets: Vec<ObjetInventaireTemporaire>,
}

/// Structure temporaire représentant un coffre chargé depuis JSON.
#[derive(Debug, Deserialize)]
struct CoffreTemporaire {
    /// Identifiant du coffre (texte).
    #[serde(rename = "id")]
    id_texte: String,
    /// Identifiant de la zone contenant ce coffre (texte).
    #[serde(rename = "id_zone")]
    id_zone_texte: String,
    #[serde(rename = "desc")]
    /// Description du coffre.
    description: String,
    /// Indique si le coffre est ouvert (texte "true" ou "false").
    _ouvert: String,
    /// Indique si le coffre est visible (texte "true" ou "false").
    _visible: String,
    /// Inventaire du coffre (liste d'inventaires, JSON utilise un tableau).
    _inventaire: Vec<InventaireTemporaire>, // Le JSON utilise un tableau
}


/// Charge le contenu d'un fichier JSON en une chaîne de caractères.
///
/// # Arguments
///
/// * `chemin` - Chemin vers le fichier JSON.
///
/// # Erreurs
///
/// Retourne une erreur si le fichier ne peut pas être lu.
pub fn charger_json(chemin: &str)->Result<String, Box<dyn Error>>{
    let contenu = fs::read_to_string(chemin)?;
    Ok(contenu)
}


/// Charge et retourne la liste des zones du jeu à partir du fichier JSON.
///
/// Cette fonction charge également les coffres associés à chaque zone.
///
/// # Erreurs
///
/// Retourne une erreur si le chargement ou la conversion échoue.
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
        let mut ouvert = true;
        if zone_temp.ouvert == "false" {
            ouvert = false;
        }

        let inventaire = Inventaire {
            taille : 255,
            objets : Vec::new(),
        };
        let prix = zone_temp.prix.parse::<u32>()?;
        let zone_finale = Zone {
            id: id_numerique,
            nom: zone_temp.nom.clone(),
            ouvert: ouvert,
            description: zone_temp.description.clone(),
            connection: zone_temp.connection.clone(),
            coffres: coffre_zone,
            objet_zone : inventaire,
            mob_present: zone_temp.mob_present,
            prix,
        };
        zones_finales.push(zone_finale);
    }

    Ok(zones_finales)
}


/// Charge et retourne un dictionnaire des coffres par zone.
///
/// # Erreurs
///
/// Retourne une erreur si le chargement ou la conversion échoue.
pub fn charger_coffres() -> Result<HashMap<u8, Vec<Coffre>>, Box<dyn Error>> {
    let contenu = charger_json("src/json/coffre.json")?;
    let coffres_temp: Vec<CoffreTemporaire> = serde_json::from_str(&contenu)?;
    let mut coffre_finales: HashMap<u8, Vec<Coffre>> = HashMap::new();

    for coffre in coffres_temp {
        let id_zone = coffre.id_zone_texte.parse::<u8>()?;
        let id = coffre.id_texte.parse::<u8>()?;
        let mut ouvert = true;
        if coffre._ouvert == "false" {
            ouvert = false;
        }
        let mut visible = true;
        if coffre._visible == "false" {
            visible = false;
        }
        // Récupérer le premier élément de la liste `inventaire`
        // let inventaire_temp = coffre.inventaire.get(0).ok_or("Inventaire vide")?;
        //
        // let inventaire = Inventaire {
        //     taille: inventaire_temp.taille_texte.parse::<u8>()?, // Conversion de taille
        //     objets: inventaire_temp.objets.iter().map(|o| ObjetInventaire {
        //         nombre: o.nombre,
        //         objet_id: o.objet_id,
        //     }).collect(), // Mapper les objets avec leurs quantités
        // };
        let inventaire = Inventaire {
            taille: 10,
            objets: Vec::new(),
        };

        let c = Coffre {
            _id: id,
            _id_zone: id_zone,
            _description: coffre.description.clone(),
            inventaire,
            ouvert: ouvert,
            visible : visible,
        };

        coffre_finales.entry(id_zone).or_insert(Vec::new()).push(c);

    }

    for coffres in coffre_finales.values_mut() {
        remplir_coffres(coffres);
    }

    Ok(coffre_finales)
}


/// Remplit aléatoirement les coffres avec des objets disponibles.
///
/// Chaque coffre reçoit un nombre aléatoire d'objets (entre 1 et 5),
/// avec au maximum 2 exemplaires de chaque objet.
///
/// # Arguments
///
/// * `coffres` - Slice mutable des coffres à remplir.
pub fn remplir_coffres(coffres: &mut [Coffre]){
    let objets_disponibles = OBJETS_DISPONIBLES.read().unwrap();
    let mut rng = rand::rng();
    let ids_objets: Vec<u8> = objets_disponibles.keys().cloned().collect();

    for coffre in coffres.iter_mut() {
        let nb_objets = rng.random_range(1..=5);
        let mut tirages: HashMap<u8, u8> = HashMap::new();
        let mut total_ajout = 0;

        while total_ajout < nb_objets {
            if let Some(&objet_id) = ids_objets.choose(&mut rng) {
                let compteur = tirages.entry(objet_id).or_insert(0);
                if *compteur < 2 {
                   *compteur += 1;
                    total_ajout += 1;
                    coffre.inventaire.ajouter_objet(objet_id);
                }
            }
        }

        coffre.inventaire.taille = 5;
    }

}

/// Charge les objets depuis le fichier JSON et les ajoute à la collection globale.
///
/// # Erreurs
///
/// Retourne une erreur si le chargement ou la conversion échoue.
pub fn charger_objets() -> Result<(), Box<dyn Error>> {
    let contenu = charger_json("src/json/objet.json")?;
    let objets_temp: Vec<serde_json::Value> = serde_json::from_str(&contenu)?;
    for objet_val in objets_temp {
        let id = objet_val["id"].as_str().unwrap().parse::<u8>()?;
        let nom = objet_val["nom"].as_str().unwrap().to_string();
        let poids = objet_val["poids"].as_str().unwrap().parse::<u32>()?;
        let prix = objet_val["prix"].as_str().unwrap().parse::<u32>()?;

        let objet_type_val = &objet_val["objet_type"];

        let objet_type = if let Some(ar) = objet_type_val.get("Arme") {
            TypeObjet::Arme {
                frequence_degats: ar["frequence_degats"].as_str().unwrap().parse()?,
                proba_degats: ar["proba_degats"].as_str().unwrap().parse()?,
                degats: ar["degats"].as_str().unwrap().parse()?,
            }
        } else if let Some(eq) = objet_type_val.get("Equipement") {
            TypeObjet::Equipement {
                protection: eq["protection"].as_str().unwrap().parse()?,
                emplacement: eq["emplacement"].as_str().unwrap().parse()?,
            }
        } else if let Some(soin) = objet_type_val.get("Soin") {
            TypeObjet::Soin {
                vie: soin["vie"].as_str().unwrap().parse()?,
                emplacement: soin["emplacement"].as_str().unwrap().parse()?,
            }
        } else {
            return Err("Objet inconnu ou type manquant".into());
        };

        ajouter_objet(id, nom, poids, prix, objet_type);
    }
    Ok(())
}