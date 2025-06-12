use rand::Rng;
use crate::personnage::Personnage;
use crate::objet::{OBJETS_DISPONIBLES, TypeObjet};
use crate::affichage;

/// Résultat d'un combat entre deux personnages.
pub struct CombatResultat {
    /// Nom du vainqueur du combat (ou `None` en cas d'égalité).
    pub _vainqueur: Option<String>,
    /// État final du joueur après le combat.
    pub etat_final_joueur: Personnage,
    /// État final du monstre/ennemi après le combat.
    pub _etat_final_mob: Personnage,
}

/// Simule un combat entre deux personnages (joueur et mob).
///
/// Le combat se déroule en tours jusqu'à ce qu'un des personnages ne puisse plus se battre.
/// À chaque tour, un attaquant est choisi aléatoirement, qui utilise une arme (ou ses mains) pour attaquer une partie du corps de l'adversaire.
/// Les protections de l'adversaire réduisent les dégâts.
///
/// # Arguments
/// * `p1` - Le personnage joueur.
/// * `p2` - Le personnage ennemi ou mob.
/// * `zone` - La zone dans laquelle le combat a lieu.
/// * `tous_les_pnjs` - Liste des PNJs pour mise à jour de l'affichage après le combat.
///
/// # Retour
/// Retourne un `CombatResultat` contenant le vainqueur (s'il y en a un) et l'état final des deux personnages.
pub fn combattre(mut p1: Personnage, mut p2: Personnage, zone: &crate::zone::Zone, tous_les_pnjs: &[crate::personnage::PNJ]) -> CombatResultat {
    let mut rng = rand::rng();
    let mut attaquant = if rng.random_bool(0.5) { 0 } else { 1 };
    let mut tour = 0;
    
    //tant que les joueurs peuvent se battre on continue le combat
    while p1.peut_se_battre() && p2.peut_se_battre() {
        let (att, def) = if attaquant == 0 {
            (&mut p1, &mut p2)
        } else {
            (&mut p2, &mut p1)
        };

        let arme = att.parties_du_corps.iter()
            .filter(|p| p.nom().to_lowercase().contains("bras"))
            .flat_map(|bras| bras.equipement().objets.iter())
            .find_map(|obj_inv| {
                let objets = OBJETS_DISPONIBLES.read().unwrap();
                objets.get(&obj_inv.objet_id).and_then(|o| match &o.objet_type {
                    TypeObjet::Arme { .. } => Some(o.clone()),
                    _ => None,
                })
            });

        // partie du corps qui va etre toucher
        let parties_cibles: Vec<_> = def.parties_du_corps.iter().enumerate().filter(|(_, p)| !p.est_morte()).collect();
        if parties_cibles.is_empty() {
            break;
        }
        let (index_cible, _) = parties_cibles[rng.random_range(0..parties_cibles.len())];
        let nom_partie = def.parties_du_corps[index_cible].nom().to_string();

        // calcul les degats
        let (degats, proba, nom_arme) = if let Some(arme) = &arme {
            if let TypeObjet::Arme { degats, proba_degats, .. } = arme.objet_type {
                (degats as i32 + att.force_effective() as i32 / 10, proba_degats, arme.nom.clone())
            } else {
                (att.force_effective() as i32 / 10, 1.0, "Mains nues".to_string())
            }
        } else {
            (att.force_effective() as i32 / 10, 1.0, "Mains nues".to_string())
        };
        
        if rng.random_bool(proba as f64) {
            let protection: i32 = def.parties_du_corps[index_cible].equipement().objets.iter().map(|obj_inv| {
                let objets = OBJETS_DISPONIBLES.read().unwrap();
                objets.get(&obj_inv.objet_id).and_then(|o| match &o.objet_type {
                    TypeObjet::Equipement { protection, .. } => Some(*protection as i32),
                    _ => None,
                }).unwrap_or(0)
            }).sum();
            let degats_finals = if protection > 0 {
                let diviseur = (1.5 * protection as f32).floor().max(1.0);
                ((degats as f32) / diviseur).floor() as u32
            } else {
                degats.max(0) as u32
            };
            if degats_finals > 0 {
                def.gerer_blessure(&nom_partie, degats_finals);
            }
        }
        println!("Tour {tour} : {} attaque {} avec {} sur {} (dégâts: {})", att.nom, def.nom, nom_arme, nom_partie, degats);
        let partie_cible = &def.parties_du_corps[index_cible];
        println!("  -> {} de {} : {}/{} HP, état : {}", nom_partie, def.nom, partie_cible.vie_actuelle(), partie_cible.vie_max(), partie_cible.etat());

        attaquant = 1 - attaquant;
        tour += 1;
    }
    // À la fin du combat, on applique la règle de soin sur les deux personnages
    p1.soigner_apres_combat();
    p2.soigner_apres_combat();
    let vainqueur = if p1.peut_se_battre() && !p2.peut_se_battre() {
        Some(p1.nom.clone())
    } else if p2.peut_se_battre() && !p1.peut_se_battre() {
        Some(p2.nom.clone())
    } else {
        None
    };
    affichage::afficher_zone(zone, tous_les_pnjs);
    CombatResultat {
        _vainqueur: vainqueur,
        etat_final_joueur: p1,
        _etat_final_mob: p2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::personnage::{Personnage, PartieDuCorps};

    #[test]
    fn test_combatresultat_structure() {
        let p1 = Personnage {
            id: 1,
            nom: "A".to_string(),
            description: "desc".to_string(),
            force: 10,
            inventaire: crate::inventaire::Inventaire { taille: 1, objets: vec![] },
            parties_du_corps: vec![PartieDuCorps::new("Tête".to_string(), 10)],
            argent: 0,
            est_vivant: true,
        };
        let p2 = p1.clone();
        let res = CombatResultat {
            _vainqueur: Some("A".to_string()),
            etat_final_joueur: p1,
            _etat_final_mob: p2,
        };
        assert_eq!(res._vainqueur, Some("A".to_string()));
    }
}
