use serde::Deserialize;
use std::collections::HashMap;
use std::cmp::Reverse;
use indexmap::IndexMap; // HashMap qui conserve l'ordre d'insertion
#[derive(Debug, Deserialize, Clone)]
pub struct Inventaire {
    pub taille: u8,
    pub objets: Vec<ObjetInventaire>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct ObjetInventaire {
    pub nombre : u8,
    pub objet_id: u8,
}

impl Inventaire {

    pub fn afficher(&mut self) {
        println!("📦 Inventaire (Taille: {}):", self.taille);

        if self.objets.is_empty() {
            println!("  - (vide)");
        } else {
            self.trier_quantite();
            for (index, (obj)) in self.objets.iter().enumerate() {
                println!("  {}: Objet ID {} (x{})", index + 1, obj.objet_id, obj.nombre);
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

    pub fn récupérer_objet(&mut self, index:usize) -> u8 {
        let obj = self.objets[index].objet_id;

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

}