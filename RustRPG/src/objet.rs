use std::collections::HashMap;
use std::fmt;
use once_cell::sync::Lazy;
use std::sync::RwLock;
use std::str::FromStr;

/// Représente un objet dans le jeu avec ses caractéristiques générales.#[derive(Debug, Clone)]
pub struct Objet {
    pub id: u8,
    pub nom: String,
    pub poids: u32,
    pub prix: u32,
    pub objet_type: TypeObjet,
}

/// Types possibles d'objets.
///
/// - `Arme` : Objet pouvant infliger des dégâts.
/// - `Equipement` : Objet fournissant une protection et pouvant être équipé à un emplacement donné.
/// - `Soin` : Objet permettant de restaurer de la vie, applicable à un emplacement.
#[derive(Debug, Clone)]
pub enum TypeObjet {
    /// Arme avec dégâts, probabilité et fréquence des dégâts.
    Arme {
        /// Points de dégâts infligés.
        degats: u32,
        /// Probabilité d'infliger les dégâts (entre 0.0 et 1.0).
        proba_degats: f32,
        /// Fréquence d'apparition des dégâts (nombre d'attaques entre dégâts).
        frequence_degats: u8,
    },
    /// Équipement offrant une protection à un emplacement précis du corps.
    Equipement {
        /// Valeur de protection fournie.
        protection: u8,
        /// Emplacement du corps où l'équipement peut être porté.
        emplacement: Emplacement,
    },
    /// Objet de soin permettant de restaurer des points de vie.
    Soin {
        /// Points de vie restaurés.
        vie: u32,
        /// Emplacement du corps auquel l'objet peut être appliqué.
        emplacement: Emplacement,
    },
}

/// Emplacements possibles où un équipement ou un soin peut être appliqué.
///
/// - `Bras`
/// - `Jambe`
/// - `Tete`
/// - `Torse`
/// - `Tous` (applicable à tous les emplacements)
/// - `Aucun` (aucun emplacement spécifique)
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

    /// Convertit une chaîne de caractères en un emplacement.
    ///
    /// # Arguments
    ///
    /// * `s` - Chaîne représentant l'emplacement ("Bras", "Jambe", etc.).
    ///
    /// # Erreurs
    ///
    /// Retourne une erreur si la chaîne ne correspond à aucun emplacement connu.
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

/// Collection globale et thread-safe des objets disponibles dans le jeu.
///
/// Utilise un verrou en lecture/écriture pour la synchronisation.
// Stockage global des objets
pub static OBJETS_DISPONIBLES: Lazy<RwLock<HashMap<u8, Objet>>> = Lazy::new(|| RwLock::new(HashMap::new()));

/// Ajoute un nouvel objet dans la collection globale des objets.
///
/// # Arguments
///
/// * `id` - Identifiant unique de l'objet.
/// * `nom` - Nom de l'objet.
/// * `poids` - Poids de l'objet.
/// * `prix` - Prix de l'objet.
/// * `objet_type` - Type de l'objet (arme, équipement, soin).
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
    /// Vérifie si l'objet est un équipement.
    ///
    /// Renvoie `false` si l'emplacement est `Aucun`.
    pub fn est_equipement(&self) -> bool {
        match &self.objet_type{
            TypeObjet::Equipement { emplacement: Emplacement::Aucun, .. } => false,
            TypeObjet::Equipement { .. } => true,
            _ => false,
        }
    }

    /// Vérifie si l'objet est une arme.
    pub fn est_arme(&self) -> bool {
        matches!(self.objet_type, TypeObjet::Arme { .. })
    }

    /// Vérifie si l'objet est un objet de soin.
    pub fn est_soin(&self) -> bool {
        matches!(self.objet_type, TypeObjet::Soin { .. })
    }

    /// Récupère l'emplacement associé à l'objet s'il existe.
    ///
    /// Retourne `Some(Emplacement)` pour les équipements et soins, sinon `None`.
    pub fn emplacement(&self) -> Option<Emplacement> {
        match &self.objet_type {
            TypeObjet::Equipement { emplacement, .. } => Some(emplacement.clone()),
            TypeObjet::Soin { emplacement, .. } => Some(emplacement.clone()),
            _ => None,
        }
    }

    /// Vérifie si l'objet peut être porté/appliqué à un emplacement cible donné.
    ///
    /// Retourne `true` si l'emplacement est `Tous` ou correspond à `cible`.
    ///
    /// # Arguments
    ///
    /// * `cible` - L'emplacement cible à vérifier.
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