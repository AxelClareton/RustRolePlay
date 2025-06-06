use rand::Rng;
use crate::personnage::Personnage;
use crate::objet::{OBJETS_DISPONIBLES, TypeObjet};
use chrono;

pub struct CombatResultat {
    pub vainqueur: Option<String>,
    pub etat_final_joueur: Personnage,
    pub etat_final_mob: Personnage,
}

pub fn combattre(mut p1: Personnage, mut p2: Personnage) -> CombatResultat {
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
        let (_, partie_cible) = parties_cibles[rng.random_range(0..parties_cibles.len())];
        let nom_partie = partie_cible.nom().to_string();

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
            let protection: i32 = partie_cible.equipement().objets.iter().map(|obj_inv| {
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
    CombatResultat {
        vainqueur,
        etat_final_joueur: p1,
        etat_final_mob: p2,
    }
}

// Exemple de combat pour test
pub fn exemple_combat() -> Result<(), Box<dyn std::error::Error>> {
    let joueurs = crate::personnage::Joueur::charger_joueur("src/json/personnage.json")?;
    let joueur = joueurs.into_iter().next().ok_or("Aucun joueur trouvé")?;
    
    let mob = crate::personnage::Mob::creer_mob("Gobelin Sauvage", "Un petit gobelin agressif aux dents pointues")?.personnage;
    
    let resultat = combattre(joueur, mob);
    println!("Vainqueur : {:?}", resultat.vainqueur);
    println!("État final du joueur :\n{}", resultat.etat_final_joueur);
    println!("État final du mob :\n{}", resultat.etat_final_mob);
    Ok(())
}
