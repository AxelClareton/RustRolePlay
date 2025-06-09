use std::collections::HashMap;
use std::fmt;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Emplacement {
    Bras,
    Jambe ,
    Tete,
    Torse,
    Tous,
    Aucun,
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
            "Aucun" => Ok(Emplacement::Aucun),
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
impl fmt::Display for Objet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Objet #{}: {}", self.id, self.nom)?;
        writeln!(f, "- Poids : {}", self.poids)?;
        writeln!(f, "- Prix  : {}", self.prix)?;
        match &self.objet_type {
            TypeObjet::Arme {
                degats,
                proba_degats,
                frequence_degats,
            } => {
                writeln!(f, "- Type : Arme")?;
                writeln!(f, "  - Dégâts           : {}", degats)?;
                writeln!(f, "  - Probabilité      : {:.2}", proba_degats)?;
                writeln!(f, "  - Fréquence dégâts : {}", frequence_degats)?;
            }
            TypeObjet::Equipement {
                protection,
                emplacement,
            } => {
                writeln!(f, "- Type : Équipement")?;
                writeln!(f, "  - Protection  : {}", protection)?;
                writeln!(f, "  - Emplacement : {:?}", emplacement)?;
            }
            TypeObjet::Soin { vie, emplacement } => {
                writeln!(f, "- Type : Soin")?;
                writeln!(f, "  - Restauration de vie : {}", vie)?;
                writeln!(f, "  - Emplacement         : {:?}", emplacement)?;
            }
        }
        Ok(())
    }

}

impl Objet {
    pub fn est_equipement(&self) -> bool {
        match &self.objet_type{
            TypeObjet::Equipement { emplacement: Emplacement::Aucun, .. } => false,
            TypeObjet::Equipement { .. } => true,
            _ => false,
        }
    }

    pub fn est_arme(&self) -> bool {
        matches!(self.objet_type, TypeObjet::Arme { .. })
    }

    pub fn est_soin(&self) -> bool {
        matches!(self.objet_type, TypeObjet::Soin { .. })
    }

    pub fn emplacement(&self) -> Option<Emplacement> {
        match &self.objet_type {
            TypeObjet::Equipement { emplacement, .. } => Some(emplacement.clone()),
            TypeObjet::Soin { emplacement, .. } => Some(emplacement.clone()),
            _ => None,
        }
    }
    
    pub fn est_pour_emplacement(&self, cible: Emplacement) -> bool {
        match self.emplacement() {
            Some(Emplacement::Aucun) => false,
            Some(e) => e == cible || e == Emplacement::Tous,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_objet_est_arme_equipement_soin() {
        let arme = Objet {
            id: 1,
            nom: "Épée".to_string(),
            poids: 2,
            prix: 100,
            objet_type: TypeObjet::Arme { degats: 10, proba_degats: 1.0, frequence_degats: 1 },
        };
        assert!(arme.est_arme());
        assert!(!arme.est_equipement());
        assert!(!arme.est_soin());

        let equip = Objet {
            id: 2,
            nom: "Casque".to_string(),
            poids: 1,
            prix: 50,
            objet_type: TypeObjet::Equipement { protection: 5, emplacement: Emplacement::Tete },
        };
        assert!(equip.est_equipement());
        assert!(!equip.est_arme());
        assert!(!equip.est_soin());

        let soin = Objet {
            id: 3,
            nom: "Potion".to_string(),
            poids: 1,
            prix: 20,
            objet_type: TypeObjet::Soin { vie: 10, emplacement: Emplacement::Tous },
        };
        assert!(soin.est_soin());
        assert!(!soin.est_arme());
        assert!(!soin.est_equipement());
    }
}