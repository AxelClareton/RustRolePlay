use serde::{Serialize, Deserialize};
use std::cmp::Reverse;
use crate::objet::OBJETS_DISPONIBLES;
use std::sync::RwLockReadGuard;
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
    pub fn afficher(&mut self) -> Option<usize> {
        if self.objets.is_empty(){
            println!("📦 Malheureusement le coffre est vide");
            return None
        }
        println!("📦 Inventaire (Taille: {}):", self.taille);
        let objets_all: RwLockReadGuard<_> = OBJETS_DISPONIBLES.read().unwrap();
        if self.objets.is_empty() {
            println!("  - (vide)");
            None
        } else {
            self.trier_quantite();
            for (index, obj) in self.objets.iter().enumerate() {
                //println!("  {}: Objet ID {} (x{})", index + 1, obj.objet_id, obj.nombre);
                if let Some(o) = objets_all.get(&obj.objet_id) {
                    println!("  {} : {} (x{})", index + 1, o.nom, obj.nombre);
                } else {
                    println!("  Objet inconnu (ID: {})", obj.objet_id);
                }
            }
            println!("Saisir 'q' pour revenir en arrière, 't' pour utilisier un objet ou le nombre correspondant à l'item que vous voulez récupéré");
            let mut choix = String::new();
            std::io::stdin().read_line(&mut choix).expect("❌ Erreur de lecture !");
            let choix = choix.trim();
            match choix {
                "q" => {
                    println!("Retour en arrière...");
                    None
                }
                "t" => {
                    self.afficher();
                    None
                }
                _ => match choix.parse::<u8>() {
                    Ok(index) if index <= self.objets.len() as u8  => {
                        let obj = self.récupérer_objet((index-1) as usize);
                        println!("Vous avez récupérer l'objet {}", obj);
                        Some(obj)
                    }
                    _ => {
                        println!("❌ Entrée invalide ! Veuillez entrer un nombre valide.");
                        None
                    }
                },
            }
        }
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

    pub fn récupérer_objet(&mut self, index:usize) -> usize {
        let obj:usize = self.objets[index].objet_id as usize;
        self.objets[index].nombre -= 1;
        if self.objets[index].nombre == 0 {
            self.objets.remove(index);
        }
        self.trier_quantite();

        obj
    }

    pub fn trier_quantite(&mut self){
        self.objets.sort_by_key(|obj| Reverse(obj.nombre));
    }

    pub fn tout_recuperer(&mut self, inventaire: &mut Inventaire){
        //
        self.objets.extend(inventaire.objets.drain(..));
        inventaire.objets = Vec::new();
    }
}