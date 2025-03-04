use crate::inventaire::Inventaire;

#[derive(Debug, Clone)]
pub struct Coffre {
    pub id: u8,
    pub id_zone: u8,
    pub prix: u8,
    pub ouvert: bool,
    pub description: String,
    pub inventaire: Inventaire,
    pub visible: bool,
}

impl Coffre {
    pub fn ouvrir(&mut self) -> Option<usize>{
        if !self.ouvert {
            println!("Voulez vous acheter ce coffe pour {}? (oui pour acheter, autres réponses pour non)", self.prix);
            let mut choix = String::new();
            std::io::stdin().read_line(&mut choix).expect("❌ Erreur de lecture !");
            let choix = choix.trim();
            match choix {
                "oui" => {
                    self.ouvert = true;
                    //déduire le prix
                }
                _ => {
                    println!("Coffre non acheté");
                    return None;
                }
            }
        }
        println!("Ouverture du coffre ! ");
        self.inventaire.afficher();
        println!("Saisir 'q' pour revenir en arrière, 'a' pour ajouter un objet ou un nombre correspondant à l'item que vous voulez récupéré");
        let mut choix = String::new();
        std::io::stdin().read_line(&mut choix).expect("❌ Erreur de lecture !");
        let choix = choix.trim();
        match choix {
            "q" => {
                println!("Retour en arrière...");
                None
            }
            _ => match choix.parse::<u8>() {
                Ok(index) if index <= self.inventaire.objets.len() as u8  => {
                    let obj = self.inventaire.récupérer_objet((index-1) as usize);
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