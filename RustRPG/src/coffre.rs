use crate::inventaire::Inventaire;
use crate::affichage;
use crate::zone::Zone;
use crate::personnage::PNJ;
use crate::personnage::Personnage;

/// Représente un coffre contenant des objets dans une zone du jeu.
/// Un coffre peut être visible ou non, ouvert ou fermé, et possède un inventaire propre.
#[derive(Debug, Clone)]
pub struct Coffre {
    /// Identifiant unique du coffre.
    pub _id: u8,
    /// Identifiant de la zone dans laquelle se trouve le coffre.
    pub _id_zone: u8,
    /// Indique si le coffre est ouvert (`true`) ou fermé (`false`).
    pub ouvert: bool,
    /// Description du coffre, affichée lors de l'interaction.
    pub _description: String,
    /// Inventaire du coffre contenant les objets qu'il renferme.
    pub inventaire: Inventaire,
    /// Indique si le coffre est visible dans la zone (`true`) ou caché (`false`).
    pub visible: bool,
}

impl Coffre {
    /// Tente d'ouvrir le coffre.
    ///
    /// Si le coffre est fermé, demande au joueur s’il souhaite utiliser une clé
    /// (objet d’ID 12) pour l’ouvrir. Si la clé est disponible dans l'inventaire du joueur,
    /// elle est consommée et le coffre est ouvert.
    ///
    /// Retourne `Some(())` si le coffre a été ouvert avec succès, ou s’il l’était déjà.
    /// Retourne `None` si l’utilisateur annule ou s’il n’a pas de clé.
    ///
    /// # Arguments
    /// - `zone` : Référence à la zone actuelle (pour les notifications).
    /// - `joueur` : Le personnage joueur interagissant avec le coffre.
    /// - `pnjs` : Liste des PNJs présents dans la zone.
    pub fn ouvrir(&mut self, zone: &Zone, joueur: &mut Personnage, pnjs: &Vec<PNJ>) -> Option<()>{
        if !self.ouvert {
            let choix = affichage::faire_choix(
                "Ce coffre est fermé voulez-vous utiliser une clé pour l'ouvrir ? (oui/non)",
                &vec!["oui".to_string(), "non".to_string()]
            );
            match choix.as_str() {
                "oui" => {
                    if !joueur.inventaire.retirer_par_id(12) {
                        affichage::notifier(zone, "❌ Vous n'avez pas de clé !", pnjs);
                        return None;
                    }
                    self.ouvert = true;
                    affichage::notifier(zone, "🔑 Vous utilisez une clé et ouvrez le coffre !", pnjs);
                }
                _ => {
                    println!("Le coffre reste verrouillé !");
                    return None;
                }
            }
        }
        println!("Ouverture du coffre ! ");
        Some(())
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coffre_creation() {
        let coffre = Coffre {
            _id: 1,
            _id_zone: 1,
            ouvert: false,
            _description: "Un coffre".to_string(),
            inventaire: Inventaire { taille: 1, objets: vec![] },
            visible: true,
        };
        assert_eq!(coffre._id, 1);
        assert!(!coffre.ouvert);
    }
}