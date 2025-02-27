use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::error::Error;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Zone {
    pub id: u8,
    pub nom: String,
    pub description: String,
    pub connection: Vec<ConnexionTemporaire>,
}

// Structures pour lire le JSON
#[derive(Debug, Deserialize, Clone)]
pub struct ConnexionTemporaire {
    pub direction: String,
    pub id_dest: String,
}

#[derive(Debug, Deserialize)]
struct ZoneTemporaire {
    #[serde(rename = "id")]
    id_texte: String,
    #[serde(rename = "nom")]
    nom: String,
    #[serde(rename = "desc")]
    description: String,
    connection: Vec<ConnexionTemporaire>,
}

// Fonction pour charger les zones
pub fn charger_zones() -> Result<Vec<Zone>, Box<dyn Error>> {
    let contenu = fs::read_to_string("src/json/zones/zone.json")?;
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
        let mut zone_finale = Zone {
            id: id_numerique,
            nom: zone_temp.nom.clone(),
            description: zone_temp.description.clone(),
            connection: zone_temp.connection.clone(),
        };

        zones_finales.push(zone_finale);
    }

    Ok(zones_finales)
}
