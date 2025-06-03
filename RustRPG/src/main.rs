mod moteur;
mod coffre;
mod zone;
mod inventaire;
mod objet;
mod personnage;
mod affichage;
use zone::Zone;
use moteur::{charger_zones};
use rand::Rng;
use crate::moteur::charger_objets;
use std::thread::sleep;
use std::time::Duration;
use inventaire::Inventaire;
use personnage::Joueur;
use personnage::Personnage;
use personnage::PNJ;
use personnage::Mob;

fn se_deplacer(zones: &mut Vec<Zone>, current_zone_index: &mut usize, direction: &str) {
    let current_zone = &zones[*current_zone_index];

    // Trouver la connexion
    if let Some(conn) = current_zone.connection.iter().find(|c| c.direction == direction) {
        // Trouver la nouvelle zone via l'ID de la connexion
        if let Some(new_index) = zones.iter().position(|z| z.id == conn.id_dest.parse::<u8>().unwrap()) {
            if zones[new_index].ouvert {
                *current_zone_index = new_index; // Mise à jour de l'index
                affichage::notifier(&zones[*current_zone_index], "Déplacement...");
                sleep(Duration::from_secs(5));
                affichage::notifier(&zones[*current_zone_index],"Vous êtes arrivés dans la zone");
            }
            else {
                let choix = affichage::faire_choix(
                    &format!("La zone {} n'est pas ouverte, voulez-vous l'acheter ? (oui/non)", conn.id_dest),
                    &vec!["oui".to_string(), "non".to_string()]
                );
                match choix.as_str() {
                    "oui" => {
                        zones[new_index].ouvert = true;
                        //déduire le prix
                        *current_zone_index = new_index; // Mise à jour de l'index
                        affichage::notifier(&zones[*current_zone_index], "Déplacement...");
                        sleep(Duration::from_secs(5));
                        affichage::notifier(&zones[*current_zone_index],"Vous êtes arrivés dans la zone");
                    }
                    _ => {
                        affichage::notifier(&zones[*current_zone_index], "Zone non achetée, vous restez dans la même zone");
                    }
                }
            }
        } else {
            affichage::notifier(&zones[*current_zone_index], "⚠️ La zone de destination n'a pas été trouvée !");
        }
    } else {
        affichage::notifier(&zones[*current_zone_index], "❌ Vous êtes arrivé au bout du monde, faites demi-tour !");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Chargement des zones
    let mut zones = charger_zones().expect("⚠️ Impossible de charger les zones !");
    charger_objets().expect("⚠️ Impossible de charger les objets !");
    // Trouver l'index de la zone de départ (id == 1)
    let mut current_zone_index = zones.iter_mut().position(|zone| zone.id == 1)
        .expect("⚠️ La zone avec l'id 1 n'a pas été trouvée !");

    // ajouter_objet(1, "Épée");
    // ajouter_objet(2, "Potion");
    // ajouter_objet(3, "Bouclier");

    let inventaire = &mut Inventaire {
        taille : 5,
        objets: Vec::new(),
    };
    //Initiliasation du personnage avec l'id 1 au cas où il n'y a pas de personnage.
    let personnages = Joueur::charger_joueur("src/json/personnage.json")?;
    let mut perso_joueur : Personnage = personnages.into_iter().find(|j| j.id == 1).expect("No player found with this ID");

    loop {
        let choix_perso = affichage::faire_choix(
            "Choisissez quoi faire (1 créer perso, 2 charger perso) : ",
            &vec!["1".to_string(), "2".to_string(), "admin".to_string()]
        );
    
        match choix_perso.as_str() {
            "1" => {
                println!("Entrez le nom de votre personnage : ");
                let mut nom = String::new();
                std::io::stdin().read_line(&mut nom).expect("❌ Erreur de lecture !");
                let nom = nom.trim();
    
                println!("Décrivez votre personnage : ");
                let mut description = String::new();
                std::io::stdin().read_line(&mut description).expect("❌ Erreur de lecture !");
                let description = description.trim();
    
                let joueur = Joueur::creer_joueur(nom, description)?;
                let joueur_id = joueur.personnage.id;
                let personnages = Joueur::charger_joueur("src/json/personnage.json")?;
                let joueur = personnages.into_iter().find(|j| j.id == joueur_id);
                println!("Joueur créé: {:#?}", joueur);
                perso_joueur = joueur.expect("Aucun personnage trouvé avec cet ID.");
                break;
            }
            "2" => {
                let personnages = Joueur::charger_joueur("src/json/personnage.json")?;
                if personnages.is_empty() {
                    println!("⚠️ Aucun personnage trouvé.");
                    continue;
                }
    
                println!("Liste des personnages disponibles :");
                for personnage in &personnages {
                    println!("ID: {}, Nom: {}", personnage.id, personnage.nom);
                }
    
                println!("Entrez l'ID du personnage que vous souhaitez charger :");
                let mut id_choisi = String::new();
                std::io::stdin().read_line(&mut id_choisi).expect("❌ Erreur de lecture !");
                let id_choisi: u32 = id_choisi.trim().parse().expect("❌ Erreur de lecture de l'ID");
    
                if let Some(joueur) = personnages.into_iter().find(|j| j.id == id_choisi) {
                    println!("Joueur chargé : {:#?}", joueur);
                    perso_joueur = joueur;
                    break;
                } else {
                    println!("❌ Aucun personnage trouvé avec cet ID.");
                }
            }
            "admin" => {
                loop {
                    let choix_type = affichage::faire_choix(
                        "Choisissez le type de personnage à créer (1 PNJ, 2 Mob, 3 Retour) : ",
                        &vec!["1".to_string(), "2".to_string(), "3".to_string()]
                    );
    
                    match choix_type.as_str() {
                        "1" => {
                            println!("Entrez le nom du PNJ : ");
                            let mut nom = String::new();
                            std::io::stdin().read_line(&mut nom).expect("❌ Erreur de lecture !");
                            let nom = nom.trim();
    
                            println!("Décrivez le PNJ : ");
                            let mut description = String::new();
                            std::io::stdin().read_line(&mut description).expect("❌ Erreur de lecture !");
                            let description = description.trim();
    
                            match PNJ::creer_pnj(nom, description) {
                                Ok(pnj) => println!("✅ PNJ créé : {:#?}", pnj),
                                Err(e) => println!("❌ Erreur lors de la création du PNJ : {}", e),
                            }
                        }
                        "2" => {
                            println!("Entrez le nom du Mob : ");
                            let mut nom = String::new();
                            std::io::stdin().read_line(&mut nom).expect("❌ Erreur de lecture !");
                            let nom = nom.trim();
    
                            println!("Décrivez le Mob : ");
                            let mut description = String::new();
                            std::io::stdin().read_line(&mut description).expect("❌ Erreur de lecture !");
                            let description = description.trim();
    
                            match Mob::creer_mob(nom, description) {
                                Ok(mob) => println!("✅ Mob créé : {:#?}", mob),
                                Err(e) => println!("❌ Erreur lors de la création du Mob : {}", e),
                            }
                        }
                        "3" => {
                            println!("🔙 Retour au menu principal.");
                            break;
                        }
                        _ => println!("❌ Option inconnue !"),
                    }
                }
                continue; // Revient au choix du personnage après avoir quitté "admin"
            }
            _ => println!("❌ Option inconnue !"),
        }
    }
    
    // Message d'accueil
    affichage::notifier(&zones[current_zone_index], "✨ Bienvenue dans le RustRPG !");
    affichage::afficher_zone(&zones[current_zone_index]);
    let mut rng = rand::rng();
    // Boucle principale du jeu
    loop {
        let mut nbr_coffres = zones[current_zone_index].compter_coffre();
        let mut options = vec![
            "d".to_string(), // se déplacer
            "q".to_string(), // quitter
            "c".to_string(), // fouiller la zone
            "i".to_string(), // autre option
            "t".to_string(), // autre option
        ];
        for i in 1..=nbr_coffres {
            options.push(i.to_string());
        }



        let choix = affichage::faire_choix(
            "Que voulez-vous faire ? ('d' pour vous déplacer, 'q' pour quitter, 'c' pour fouiller la zone, le numéro du coffre) :",&options

        );
        match choix.as_str() {
            "q" => {
                  affichage::notifier(&zones[current_zone_index], "👋 Au revoir !");
                  break Ok(());
              }
            "i" => {
                println!("Votre inventaire : ");
                match perso_joueur.inventaire.afficher(){
                    Some(obj)=> {
                        let choix_utiliser = affichage::faire_choix(
                            "Voulez vous utiliser l'objet ? o/n",
                            &vec!["u".to_string(), "n".to_string()]
                        );
                        match choix_utiliser.as_str() {
                            "u" => {
                                println!("Utilisation de l'objet {}", obj)
                            }
                            _ => {
                                println!("Vous vous débarassez de l'objet");
                                &mut zones[current_zone_index].objet_zone.ajouter_objet(obj as u8);
                                //ajout dans les objets de la zones
                            }
                        }
                    }
                    None => ()
                }
            }
            "c" => {
                affichage::notifier(&zones[current_zone_index], "Fouillage de la zone en cours...");
                sleep(Duration::from_secs(5));
                zones[current_zone_index].fouiller_zone();
                affichage::afficher_zone(&zones[current_zone_index]);
            }
            "t" => {
                println!("Fouillage de la zone en cours...");
                sleep(Duration::from_secs(5));
                match zones[current_zone_index].objet_zone.afficher(){
                    Some(obj)=> {
                        let choix_recuperer = affichage::faire_choix(
                            "Voulez vous récupérer l'objet ? (o/n)",
                            &vec!["o".to_string(), "n".to_string()]
                        );
                        match choix_recuperer.as_str() {
                            "u" => {
                                perso_joueur.inventaire.ajouter_objet(obj as u8);
                                println!("Vous récupérez l'objet {}", obj)
                            }
                            _ => {
                                println!("Vous laissez l'objet par terre ...");
                                //ajout dans les objets de la zones
                            }
                        }
                    }
                    None => ()
                }
            }
            "d" => {
                let direction = affichage::faire_choix(
                    "🚪 Vers quelle direction voulez-vous aller ?",
                    &vec!["nord".to_string(), "sud".to_string(), "est".to_string(), "ouest".to_string()]
                );
                se_deplacer(&mut zones, &mut current_zone_index, &direction);

                if rng.random_range(0..99) < 10 {
                    affichage::notifier(&zones[current_zone_index], "🎉 L'événement rare s'est produit !");
                }
            }
            "nord" | "sud" | "est" | "ouest" => {
                se_deplacer(&mut zones, &mut current_zone_index, &choix);
                if rng.random_range(0..99) < 10 {
                    affichage::notifier(&zones[current_zone_index], "🎉 L'événement rare s'est produit !");
                }
            }
            "test" => {
                let mut coffre = &mut zones[current_zone_index].coffres[0];
                inventaire.tout_recuperer(&mut coffre.inventaire);
                println!("INVENTAIRE");
                inventaire.afficher();
                println!("COFFRE");
                coffre.ouvrir();
            }
            _ => {
                if let Ok(num) = choix.parse::<usize>() {
                    if (1..=nbr_coffres).contains(&num) {
                        let coffre = &mut zones[current_zone_index].coffres[num-1]; // Récupère le coffre sélectionné
                        match coffre.ouvrir() {
                            Some(objet) => {
                                affichage::notifier(&zones[current_zone_index], &format!("Objet trouvé : {}", objet));
                                inventaire.ajouter_objet(objet as u8);
                            },
                            None => affichage::notifier(&zones[current_zone_index], "Aucun objet à récupérer"),
                        }
                        inventaire.afficher();
                    } else {
                        affichage::notifier(&zones[current_zone_index], "❌ Commande inconnue !");
                    }
                } else {
                    affichage::notifier(&zones[current_zone_index], "❌ Commande inconnue !")
                }
            },
        }
    }
}

