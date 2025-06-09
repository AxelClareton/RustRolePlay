use serde::{Serialize, Deserialize};
use std::cmp::Reverse;
use std::fmt;
use crate::objet::OBJETS_DISPONIBLES;
use std::sync::RwLockReadGuard;
use crate::affichage;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inventaire {
    pub taille: u8,
    pub objets: Vec<ObjetInventaire>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObjetInventaire {
    pub nombre : u8,
    pub objet_id: u8,
}

impl Inventaire {
    pub fn afficher(&mut self, est_joueur : bool, zone: &crate::zone::Zone, pnjs: &Vec<crate::personnage::PNJ>) -> Option<usize> {
        if self.objets.is_empty(){
            if est_joueur {
                affichage::notifier(zone, "📦 Votre inventaire est vide", pnjs);
            }
            else {
                affichage::notifier(zone, "📦 Malheureusement c'est vide", pnjs);
            }
            return None
        }
        let mut message = format!("📦 Inventaire (Taille: {}):\n", self.taille);
        let objets_all: RwLockReadGuard<_> = OBJETS_DISPONIBLES.read().unwrap();
        self.trier_quantite();
        for (index, obj) in self.objets.iter().enumerate() {
            if let Some(o) = objets_all.get(&obj.objet_id) {
                message.push_str(&format!("  {} : {} (x{})\n", index + 1, o.nom, obj.nombre));
            } else {
                message.push_str(&format!("  Objet inconnu (ID: {})\n", obj.objet_id));
            }
        }
        if est_joueur {
            message.push_str("Saisir 'q' pour fermer l'inventaire, ou le nombre correspondant à l'item que vous voulez utiliser\n");
            message.push_str("Entrez votre choix :");
            let mut choix_possibles: Vec<String> = (1..=self.objets.len()).map(|i| i.to_string()).collect();
            choix_possibles.push("q".to_string());
            let choix = affichage::faire_choix(&message, &choix_possibles);
            match choix.as_str() {
                "q" => {
                    affichage::notifier(zone, "Fermeture de l'inventaire...", pnjs);
                    None
                }
                _ => match choix.parse::<u8>() {
                    Ok(index) if index > 0 && (index as usize) <= self.objets.len() => {
                        Some((index - 1) as usize)
                    }
                    _ => {
                        affichage::notifier(zone, "❌ Entrée invalide ! Veuillez entrer un nombre valide.", pnjs);
                        None
                    }
                },
            }
        } else {
            message.push_str("Saisir 'q' pour fermer le coffre, ou le nombre correspondant à l'item que vous voulez récupérer\n");
            message.push_str("Entrez votre choix :");
            let mut choix_possibles: Vec<String> = (1..=self.objets.len()).map(|i| i.to_string()).collect();
            choix_possibles.push("q".to_string());
            let choix = affichage::faire_choix(&message, &choix_possibles);
            match choix.as_str() {
                "q" => {
                    affichage::notifier(zone, "Fermeture du coffre...", pnjs);
                    None
                }
                _ => match choix.parse::<u8>() {
                    Ok(index) if index > 0 && (index as usize) <= self.objets.len() => {
                        let obj_id = self.objets[index as usize - 1].objet_id;
                        let obj = self.récupérer_objet((index - 1) as usize);
                        let objets_all = OBJETS_DISPONIBLES.read().unwrap();
                        let nom_objet = objets_all.get(&obj_id).map(|o| o.nom.clone()).unwrap_or_else(|| format!("ID {}", obj_id));
                        affichage::notifier(zone, &format!("Vous avez récupéré l'objet : {}", nom_objet), pnjs);
                        Some(obj)
                    }
                    _ => {
                        affichage::notifier(zone, "❌ Entrée invalide ! Veuillez entrer un nombre valide.", pnjs);
                        None
                    }
                },
            }
        }
    }

    pub fn afficher_inventaire_zone_et_coffre(&mut self, zone: &crate::zone::Zone, joueur: &mut crate::personnage::Personnage, pnjs: &Vec<crate::personnage::PNJ>) -> Option<()> {
        use std::io;

        if self.objets.is_empty() {
            affichage::notifier(zone, "📦 Malheureusement c'est vide", pnjs);
            return None
        }
        let mut message = format!("📦 Inventaire (Taille: {}):\n", self.taille);
        let objets_all: RwLockReadGuard<_> = OBJETS_DISPONIBLES.read().unwrap();
        self.trier_quantite();
        for (index, obj) in self.objets.iter().enumerate() {
            if let Some(o) = objets_all.get(&obj.objet_id) {
                message.push_str(&format!("  {} : {} (x{})\n", index + 1, o.nom, obj.nombre));
            } else {
                message.push_str(&format!("  Objet inconnu (ID: {})\n", obj.objet_id));
            }
        }

        message.push_str("Saisir 'q' pour fermer, ou entrez le nombre correspondant à l'item que vous voulez récupérer\n");
        message.push_str("Entrez votre choix :");
        let mut choix_possibles: Vec<String> = (1..=self.objets.len()).map(|i| i.to_string()).collect();
        choix_possibles.push("q".to_string());
        let choix = affichage::faire_choix(&message, &choix_possibles);
        if choix == "q" {
            affichage::notifier(zone, "Fermeture de l'inventaire...", pnjs);
            return None
        }

        let index = choix.parse::<usize>().unwrap() - 1;
        let disponible = self.objets[index].nombre;

        println!("Combien voulez-vous récupérer ? (max {})", disponible);
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("❌ Erreur de lecture !");
        let qty = match buf.trim().parse::<u8>() {
            Ok(n) if n > 0 && n <= disponible => n,
            _ => {
                affichage::notifier(zone, "❌ Quantité invalide.", pnjs);
                return None;
            }
        };

        // 5. Vérification de la place dans l'inventaire du joueur
        let place_actuelle: u8 = joueur
            .inventaire
            .objets
            .iter()
            .map(|o| o.nombre)
            .sum();
        if place_actuelle + qty > joueur.inventaire.taille {
            affichage::notifier(zone, "❌ Pas assez de place dans votre inventaire !", pnjs);
            return None;
        }

        // 6. On retire qty fois de la zone et on ajoute dans l'inventaire du joueur
        let obj_id = self.objets[index].objet_id;
        for _ in 0..qty {
            let retire = self.récupérer_objet(index);
            joueur.inventaire.ajouter_objet(retire as u8);
        }

        // 7. Notification finale
        let nom = objets_all
            .get(&obj_id)
            .map(|o| o.nom.clone())
            .unwrap_or_else(|| format!("ID {}", obj_id));
        let msg = format!("✅ Vous récupérez {} x{}", nom, qty);
        affichage::notifier(zone, &msg, pnjs);

        Some(())
    }

    pub fn ajouter_objet(&mut self, id: u8){
        for objet in &mut self.objets {
            if objet.objet_id == id {
                objet.nombre += 1;
                return
            }
        }
        let new_obj = ObjetInventaire {
            nombre : 1,
            objet_id : id,
        };
        self.objets.insert(0, new_obj);
        self.trier_quantite();
    }

    pub fn récupérer_objet(&mut self, index: usize) -> usize {
        let obj: usize = self.objets[index].objet_id as usize;
        let _o: &ObjetInventaire = &self.objets[index];
        self.objets[index].nombre -= 1;
        if self.objets[index].nombre == 0 {
            self.objets.remove(index);
        }
        self.trier_quantite();

        obj
    }

    pub fn récupérer_objet_2(&mut self, index: usize) -> ObjetInventaire {
        let objet = self.objets[index].clone();
        self.objets[index].nombre -= 1;
        if self.objets[index].nombre == 0 {
            self.objets.remove(index);
        }
        self.trier_quantite();

        objet
    }

    pub fn trier_quantite(&mut self){
        self.objets.sort_by_key(|obj| Reverse(obj.nombre));
    }
}

impl fmt::Display for Inventaire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.objets.is_empty() {
            writeln!(f, "    (aucun objet)")?;
        } else {
            for objet in &self.objets {
                if let Some(o) = OBJETS_DISPONIBLES.read().unwrap().get(&(objet.objet_id)){
                    writeln!(f, "    - {}", o.nom)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ajouter_et_recuperer_objet() {
        let mut inv = Inventaire { taille: 5, objets: vec![] };
        inv.ajouter_objet(1);
        assert_eq!(inv.objets.len(), 1);
        assert_eq!(inv.objets[0].nombre, 1);
        inv.ajouter_objet(1);
        assert_eq!(inv.objets[0].nombre, 2);
        let id = inv.récupérer_objet(0);
        assert_eq!(id, 1);
        assert_eq!(inv.objets[0].nombre, 1);
    }

    #[test]
    fn test_trier_quantite() {
        let mut inv = Inventaire { taille: 5, objets: vec![
            ObjetInventaire { nombre: 1, objet_id: 2 },
            ObjetInventaire { nombre: 3, objet_id: 1 },
        ]};
        inv.trier_quantite();
        assert_eq!(inv.objets[0].objet_id, 1);
        assert_eq!(inv.objets[0].nombre, 3);
    }
}
