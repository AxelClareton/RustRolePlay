use crate::inventaire::Inventaire;

#[derive(Debug, Clone)]
pub struct Coffre {
    pub id: u8,
    pub id_zone: u8,
    pub description: String,
    pub inventaire: Inventaire,
    pub vide: bool,
}

impl Coffre {
    pub fn ouvrir(&mut self){
        println!("Ouverture du coffre ! ");
        self.inventaire.afficher();
        println!("Saisir 'q' pour revenir en arrière, 'a' pour ajouter un objet ou un nombre correspondant à l'item que vous voulez récupéré");
        let mut choix = String::new();
        std::io::stdin().read_line(&mut choix).expect("❌ Erreur de lecture !");
        let choix = choix.trim();
        match choix {
            "q" => {
                println!("Retour en arrière...");
            }
            "a"=>{
                self.inventaire.ajouter_objet(4);
            }
            _ => match choix.parse::<usize>() {
                Ok(index) if index <= self.inventaire.objets.len() => {
                    let obj = self.inventaire.récupérer_objet(index-1);
                    println!("Vous avez récupérer l'objet {}", obj);
                    if self.inventaire.objets.is_empty() {
                        self.vide = true;
                    }
                }
                _ => {
                    println!("❌ Entrée invalide ! Veuillez entrer un nombre valide.");
                }
            },
        }
    }
}