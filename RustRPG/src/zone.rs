use serde::Deserialize;
use crate::coffre::Coffre;
use crate::inventaire::Inventaire;
use crate::affichage::ajouter_notification;

/// Représente une connexion entre deux zones.
///
/// Une connexion contient une direction (comme "nord") et l'identifiant
/// de la zone destination.
#[derive(Debug, Deserialize, Clone)]
pub struct Connexion {
    pub direction: String,
    pub id_dest: String,
}

/// Représente une zone dans le jeu, incluant ses connexions, ses coffres,
/// et les objets qu'elle contient.
#[derive(Debug, Clone)]
pub struct Zone {
    /// Identifiant unique de la zone.
    pub id: u8,
    /// Nom de la zone.
    pub nom: String,
    /// Indique si la zone est ouverte (accessible).
    pub ouvert: bool,
    /// Description textuelle de la zone.
    pub description: String,
    /// Liste des connexions vers d'autres zones.
    pub connection: Vec<Connexion>,
    /// Liste des coffres présents dans la zone.
    pub coffres: Vec<Coffre>,
    /// Inventaire contenant les objets de la zone.
    pub objet_zone : Inventaire,
    /// Indique si un monstre (mob) est présent dans la zone.
    pub mob_present: bool,
    /// Prix pour débloquer ou accéder à la zone.
    pub prix: u32,
}

impl Zone {
    /// Compte le nombre de coffres visibles dans la zone.
    ///
    /// # Retour
    /// Le nombre de coffres ayant l'attribut `visible` à `true`.
    pub fn compter_coffre(&self) -> usize {
        let mut cpt = 0usize;
        for coffre in self.coffres.clone() {
            if coffre.visible {
                cpt += 1;
            }
        }
        cpt
    }

    /// Révèle tous les coffres non visibles de la zone.
    ///
    /// Met à jour l'état des coffres et affiche une notification avec le
    /// nombre de coffres découverts.
    pub fn fouiller_zone(&mut self) {
        let mut cpt :u8 = 0;
        for coffre in &mut self.coffres {
            if !coffre.visible {
                cpt += 1;
                coffre.visible = true;
            }
        }
        ajouter_notification(&("Vous avez trouvé ".to_owned() + &cpt.to_string() + " coffre(s) ."));
    }

    /// Supprime un coffre de la zone à l'index spécifié.
    ///
    /// # Arguments
    /// * `num` - Index du coffre à supprimer dans la liste.
    pub fn supprimer_coffre(&mut self, num : usize) {
        self.coffres.remove(num);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coffre::Coffre;

    /// Teste la méthode `compter_coffre` pour vérifier que seuls les coffres visibles sont comptés.
    #[test]
    fn test_compter_coffre() {
        let coffres = vec![
            Coffre { _id: 1, _id_zone: 1, ouvert: true, _description: "C1".to_string(), inventaire: Inventaire { taille: 1, objets: vec![] }, visible: true },
            Coffre { _id: 2, _id_zone: 1, ouvert: true, _description: "C2".to_string(), inventaire: Inventaire { taille: 1, objets: vec![] }, visible: false },
        ];
        let zone = Zone {
            id: 1,
            nom: "TestZone".to_string(),
            ouvert: true,
            description: "desc".to_string(),
            connection: vec![],
            coffres,
            objet_zone: Inventaire { taille: 1, objets: vec![] },
            mob_present: false,
            prix: 0,
        };
        assert_eq!(zone.compter_coffre(), 1);
    }
}