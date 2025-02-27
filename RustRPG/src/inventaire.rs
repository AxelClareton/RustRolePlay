use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Inventaire {
    pub taille: u8,
    pub objets: Vec<u8>,
}

impl Inventaire {
    pub fn afficher(&self) {
        println!("📦 Inventaire (Taille: {}):", self.taille);

        if self.objets.is_empty() {
            println!("  - (vide)");
        } else {
            for (index, objet) in self.objets.iter().enumerate() {
                println!("  {}: Objet ID {}", index + 1, objet);
            }
        }
    }

}