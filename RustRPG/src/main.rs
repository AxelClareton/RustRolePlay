mod moteur;
mod coffre;
mod zone;
mod inventaire;
mod objet;
mod personnage;
mod affichage;
mod combat;

use std::io;
use zone::Zone;
use moteur::{charger_zones};
use rand::{rng, Rng};
use crate::moteur::charger_objets;
use std::thread::sleep;
use std::time::Duration;
use rand::seq::IndexedRandom;
use inventaire::Inventaire;
use personnage::Joueur;
use personnage::Personnage;
use personnage::PNJ;
use personnage::Mob;
use crate::combat::{combattre, CombatResultat};
use crate::inventaire::ObjetInventaire;
use crate::objet::{Emplacement, OBJETS_DISPONIBLES};


fn se_deplacer(zones: &mut Vec<Zone>, current_zone_index: &mut usize, direction: &str, perso_joueur: &mut Personnage, pnjs: &Vec<PNJ>) {
    let current_zone = &zones[*current_zone_index];

    // Trouver la connexion
    if let Some(conn) = current_zone.connection.iter().find(|c| c.direction == direction) {
        // Trouver la nouvelle zone via l'ID de la connexion
        if let Some(new_index) = zones.iter().position(|z| z.id == conn.id_dest.parse::<u8>().unwrap()) {
            if(zones[new_index].mob_present){
                let mob_choix = affichage::faire_choix(
                    &format!("Il y a un ennemie dans la zone {}, il se peut qu'il vous attaque ,voulez-vous y aller quand même ? (oui/non)", conn.id_dest),
                    &vec!["oui".to_string(), "non".to_string()]
                );
                match mob_choix.as_str() {
                    "oui" => {
                        //println!("Début du combat");
                    }
                    _ => { 
                        println!("Vous avez peur de l'ennemie, vous restez dans la même zone");
                        return
                    }
                }
            }
            else { 
                println!("Il y a aucun mob")
            }
            if zones[new_index].ouvert {
                *current_zone_index = new_index; // Mise à jour de l'index
                affichage::notifier(&zones[*current_zone_index], "Déplacement...", &pnjs);
                sleep(Duration::from_secs(5));
                affichage::notifier(&zones[*current_zone_index],"Vous êtes arrivés dans la zone", &pnjs);
            }
            else {
                let choix = affichage::faire_choix(
                    &format!("La zone {} n'est pas ouverte, voulez-vous l'acheter ? (oui/non)", conn.id_dest),
                    &vec!["oui".to_string(), "non".to_string()]
                );
                match choix.as_str() {
                    "oui" => {
                        let prix_zone = zones[new_index].prix;
                        if perso_joueur.argent >= prix_zone {
                            perso_joueur.retirer_argent(prix_zone);
                            zones[new_index].ouvert = true;
                            *current_zone_index = new_index;
                            affichage::notifier(&zones[*current_zone_index], "Déplacement...", &pnjs);
                            sleep(Duration::from_secs(5));
                            affichage::notifier(&zones[*current_zone_index],"Vous êtes arrivés dans la zone", &pnjs);
                        } else {
                            affichage::notifier(&zones[*current_zone_index], "❌ Vous n'avez pas assez d'argent pour acheter cette zone !", &pnjs);
                        }
                    }
                    _ => {
                        affichage::notifier(&zones[*current_zone_index], "Zone non achetée, vous restez dans la même zone", &pnjs);
                    }
                }
            }
        } else {
            affichage::notifier(&zones[*current_zone_index], "⚠️ La zone de destination n'a pas été trouvée !", &pnjs);
        }
    } else {
        affichage::notifier(&zones[*current_zone_index], "❌ Vous êtes arrivé au bout du monde, faites demi-tour !", &pnjs);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Chargement des objets
    charger_objets().expect("⚠️ Impossible de charger les objets !");
    // Chargement des zones
    let mut zones = charger_zones().expect("⚠️ Impossible de charger les zones !");
    // Trouver l'index de la zone de départ (id == 1)
    let mut current_zone_index = zones.iter_mut().position(|zone| zone.id == 1)
        .expect("⚠️ La zone avec l'id 1 n'a pas été trouvée !");

    let mut pnjs = PNJ::charger_pnj("src/json/pnj.json")?;

    let _inventaire = &mut Inventaire {
        taille: 5,
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
                let id_choisi = id_choisi.trim();
                if id_choisi.is_empty() {
                    println!("❌ Vous devez entrer un ID !");
                    continue;
                }
                let id_choisi: u32 = match id_choisi.parse() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("❌ L'ID doit être un nombre !");
                        continue;
                    }
                };
    
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
                            // Crée le PNJ (nom, description (&str), plusieurs dialogues (vec<String>), numéro de zone attitré(u32), multiplicateur de prix(f32))
                            // Demander les dialogues
                            println!("Entrez les dialogues du PNJ (séparés par des /) : ");
                            let mut dialogues = String::new();
                            std::io::stdin().read_line(&mut dialogues).expect("❌ Erreur de lecture !");
                            let dialogues: Vec<String> = dialogues.trim().split('/').map(|s| s.trim().to_string()).collect();
                            // demander le nom, la description, les dialogues, le numéro de zone et le multiplicateur de prix
                            println!("Entrez le numéro de la zone attitrée (u32) : ");
                            let mut zone_attribuee = String::new();
                            std::io::stdin().read_line(&mut zone_attribuee).expect("❌ Erreur de lecture !");
                            let zone_attribuee: u32 = zone_attribuee.trim().parse().expect("❌ Erreur de lecture du numéro de zone");
                            println!("Entrez le multiplicateur de prix (f32) : ");
                            let mut multiplicateur_prix = String::new();
                            std::io::stdin().read_line(&mut multiplicateur_prix).expect("❌ Erreur de lecture !");
                            let multiplicateur_prix: f32 = multiplicateur_prix.trim().parse().expect("❌ Erreur de lecture du multiplicateur de prix");
                            // Crée le PNJ
                            match PNJ::creer_pnj(nom, description, dialogues, zone_attribuee, multiplicateur_prix) {
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
    affichage::notifier(&zones[current_zone_index], "✨ Bienvenue dans le RustRPG !", &pnjs);
    
    
    affichage::afficher_zone(&zones[current_zone_index], &pnjs);
    let mut rng = rand::rng();
    // Boucle principale du jeu
    loop {
        let nbr_coffres = zones[current_zone_index].compter_coffre();
        let tableau: Vec<usize>;
        let mut options = vec![
            "d".to_string(), // se déplacer
            "q".to_string(), // quitter
            "c".to_string(), // fouiller la zone
            "i".to_string(), // autre option
            "t".to_string(), // autre option
        ];


        let pnjs_in_zone: Vec<usize> = pnjs.iter()
            .enumerate()
            .filter(|(_, p)| p.zone_id == zones[current_zone_index].id as u32)
            .map(|(i, _)| i)
            .collect();

        if !pnjs_in_zone.is_empty() {
            options.push("p".to_string()); // interagir avec les PNJ
        }

        for i in 1..=nbr_coffres {
            options.push(i.to_string());
        }

        let choix = affichage::faire_choix(
            "Que voulez-vous faire ? ('d' pour vous déplacer, 'i' pour ouvrir l'inventaire, 'q' pour quitter, 'c' pour fouiller la zone, 'p' pour parler aux PNJ si présent dans la zone ou le numéro du coffre) :",&options
        );
        match choix.as_str() {
            "q" => {
                  affichage::notifier(&zones[current_zone_index], "👋 Au revoir !", &pnjs);
                  break Ok(());
              }
            "p" => {
                if !pnjs_in_zone.is_empty() {
                    println!("Choisissez un PNJ pour interagir :");
                    for (index, &pnj_index) in pnjs_in_zone.iter().enumerate() {
                        println!("{}. {}", index + 1, pnjs[pnj_index].personnage.nom);
                    }

                    let mut choix_pnj = String::new();
                    io::stdin().read_line(&mut choix_pnj).expect("Erreur de lecture !");
                    if let Ok(index) = choix_pnj.trim().parse::<usize>() {
                        if index > 0 && index <= pnjs_in_zone.len() {
                            let pnj_index = pnjs_in_zone[index - 1];
                            pnjs[pnj_index].interagir(&mut perso_joueur, &mut zones, current_zone_index);
                        } else {
                            println!("Numéro de PNJ invalide !");
                        }
                    } else {
                        println!("Entrée invalide !");
                    }
                }
            }
            "i" => {
                println!("Votre inventaire : ");
                match perso_joueur.inventaire.afficher(true){
                    Some(obj)=> {
                        let choix_utiliser = affichage::faire_choix(
                            "Voulez vous utiliser l'objet ? (oui ou non)",
                            &vec!["oui".to_string(), "non".to_string()]
                        );

                        match choix_utiliser.as_str() {
                            "oui" => {
                                let id = perso_joueur.inventaire.objets[obj].objet_id;
                                if let Some(o) = OBJETS_DISPONIBLES.read().unwrap().get(&(id as u8)) {
                                    println!("{}", o);
                                    if(o.est_equipement()){
                                        if(o.est_pour_emplacement(Emplacement::Tete)){
                                            tableau = vec![0]
                                        }
                                        else {
                                            tableau = vec![1]
                                        }

                                        for i in tableau{
                                            if(perso_joueur.parties_du_corps[i].equipement().objets.is_empty()){
                                                let objet : ObjetInventaire = perso_joueur.inventaire.récupérer_objet_2(obj);
                                                perso_joueur.parties_du_corps[i].ajouter_equipement(objet.objet_id);
                                                println!("Equipement équipé !");
                                            }
                                            else {
                                                let new_choix = affichage::faire_choix(
                                                    "Equipement plein, voulez vous inverser l'objet ? (oui ou non)",
                                                    &vec!["oui".to_string(), "non".to_string()]
                                                );
                                                match new_choix.as_str() {
                                                    "oui" => {
                                                        let objet : ObjetInventaire = perso_joueur.inventaire.récupérer_objet_2(obj);
                                                        let objet2 : ObjetInventaire = perso_joueur.parties_du_corps[i].récupérer_objet(obj);
                                                        perso_joueur.parties_du_corps[i].ajouter_equipement(objet.objet_id);
                                                        perso_joueur.inventaire.ajouter_objet(objet2.objet_id);
                                                    }

                                                    _ => {

                                                    }
                                                }
                                            }
                                        }
                                    }
                                    else if(o.est_arme()){
                                        let choix = affichage::faire_choix(
                                            "Dans quelle main equipée l'objet ? (g ou d ou q)",
                                            &vec!["g".to_string(), "d".to_string()]
                                        );
                                        match choix.as_str() {
                                            "g" => {
                                                if(perso_joueur.parties_du_corps[3].equipement().objets.is_empty()){
                                                    let objet : ObjetInventaire = perso_joueur.inventaire.récupérer_objet_2(obj);
                                                    perso_joueur.parties_du_corps[3].ajouter_equipement(objet.objet_id);
                                                    println!("Equipement équipé !");
                                                }
                                                else {
                                                    let new_choix = affichage::faire_choix(
                                                        "Equipement plein, voulez vous inverser l'objet ? (oui ou non)",
                                                        &vec!["oui".to_string(), "non".to_string()]
                                                    );
                                                    match new_choix.as_str() {
                                                        "oui" => {
                                                            let objet : ObjetInventaire = perso_joueur.inventaire.récupérer_objet_2(obj);
                                                            let objet2 : ObjetInventaire = perso_joueur.parties_du_corps[3].récupérer_objet(obj);
                                                            perso_joueur.parties_du_corps[3].ajouter_equipement(objet.objet_id);
                                                            perso_joueur.inventaire.ajouter_objet(objet2.objet_id);
                                                        }

                                                        _ => {

                                                        }
                                                    }
                                                }
                                            }
                                            "d" => {
                                                if(perso_joueur.parties_du_corps[2].equipement().objets.is_empty()){
                                                    let objet : ObjetInventaire = perso_joueur.inventaire.récupérer_objet_2(obj);
                                                    perso_joueur.parties_du_corps[2].ajouter_equipement(objet.objet_id);
                                                    println!("Equipement équipé !");
                                                }
                                                else {
                                                    let new_choix = affichage::faire_choix(
                                                        "Equipement plein, voulez vous inverser l'objet ? (oui ou non)",
                                                        &vec!["oui".to_string(), "non".to_string()]
                                                    );
                                                    match new_choix.as_str() {
                                                        "oui" => {
                                                            let objet : ObjetInventaire = perso_joueur.inventaire.récupérer_objet_2(obj);
                                                            let objet2 : ObjetInventaire = perso_joueur.parties_du_corps[2].récupérer_objet(obj);
                                                            perso_joueur.parties_du_corps[2].ajouter_equipement(objet.objet_id);
                                                            perso_joueur.inventaire.ajouter_objet(objet2.objet_id);
                                                        }

                                                        _ => {

                                                        }
                                                    }
                                                }
                                            }
                                            _ => {

                                            }
                                        }
                                    }
                                    else if (o.est_soin()) {
                                        let choix = affichage::faire_choix(
                                            "Sur quelle partie du corps utilisé l'objet ? (0 : tete, 1 : torse, 2 : bras droit, 3 : bras gauche, 4 : jambre droite, 5 : jambe gauche, q : quitter)",
                                            &vec!["0".to_string(), "1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "q".to_string()]
                                        );
                                        match choix.as_str() {
                                            "0" => println!("Soin de la tête"),
                                            "1" => println!("Soin du torse"),
                                            "2" => println!("Soin du bras droit"),
                                            "3" => println!("Soin du bras gauche"),
                                            "4" => println!("Soin de la jambe droite"),
                                            "5" => println!("Soin de la jambe gauche"),
                                            _ => println!("Annulation du soin.")
                                        }
                                }
                                }
                                else{
                                    println!("pas d'objet trouvé");
                                }

                            }
                            _ => {
                                //println!("Vous vous débarassez de l'objet");
                                let choix_jeter = affichage::faire_choix(
                                    "Voulez vous jeter l'objet ? (oui ou non)",
                                    &vec!["oui".to_string(), "non".to_string()]
                                );

                                match choix_jeter.as_str() {
                                    "oui" => {
                                        let objet : ObjetInventaire = perso_joueur.inventaire.récupérer_objet_2(obj);
                                        zones[current_zone_index].objet_zone.ajouter_objet(objet.objet_id);
                                        println!("VOus vous débarassé de l'objet")
                                        //perso_joueur.parties_du_corps[i].ajouter_equipement(objet.objet_id);
                                    }
                                    _ => {
                                        println!("Vous ne faites rien de cette objet.")
                                    }
                                }
                            }
                        }
                    }
                    None => ()
                }
            }
            "c" => {
                affichage::notifier(&zones[current_zone_index], "Fouillage de la zone en cours...", &pnjs);
                sleep(Duration::from_secs(5));
                zones[current_zone_index].fouiller_zone(&pnjs);
                affichage::afficher_zone(&zones[current_zone_index], &pnjs);
            }
            "t" => {
                println!("Fouillage de la zone en cours...");
                sleep(Duration::from_secs(5));
                match zones[current_zone_index].objet_zone.afficher(false){
                    Some(obj)=> {
                        let choix_recuperer = affichage::faire_choix(
                            "Voulez vous récupérer l'objet ? (oui/non)",
                            &vec!["oui".to_string(), "non".to_string()]
                        );
                        match choix_recuperer.as_str() {
                            "oui" => {
                                perso_joueur.inventaire.ajouter_objet(obj as u8);
                                println!("Vous récupérez l'objet {}", obj)
                            }
                            _ => {
                                println!("Vous laissez l'objet par terre ...");
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
                se_deplacer(&mut zones, &mut current_zone_index, &direction, &mut perso_joueur, &pnjs);
                if(zones[current_zone_index].mob_present){
                    let mut rng = rand::rng();
                    let chance: f32 = rng.random();

                    if chance < 0.9 {
                        // Le mob apparaît (60 % de chances)
                        let mobs = Mob::charger_mob("src/json/mob.json")?;

                        let mut rng = rand::rng();
                        if let Some(mob_choisi) = mobs.choose(&mut rng) {
                            println!(
                                "Mob choisi au hasard : ID: {}, Nom: {}, Description: {}",
                                mob_choisi.id, mob_choisi.nom, mob_choisi.description
                            );
                            let resultat = combattre(perso_joueur.clone(), mob_choisi.clone());
                            if(resultat.etat_final_joueur.est_vivant){
                                perso_joueur.parties_du_corps = resultat.etat_final_joueur.parties_du_corps;
                                for p in resultat.etat_final_mob.parties_du_corps{
                                    println!("{}", p);
                                    if p.equipement().objets.len() > 1 {
                                        let objet = &p.equipement().objets[1];
                                        zones[current_zone_index].objet_zone.ajouter_objet(objet.objet_id);
                                    }
                                }
                                
                                println!("Vous avez gagné");
                                perso_joueur.ajouter_argent(mob_choisi.argent);
                                println!("Vous ramassez {} pièces d'or sur le mob !", mob_choisi.argent);
                            }
                            else { 
                                println!("Malheureusement vous venez de perdre la partie s'arrete pour vous ... \n N'hésitez pas a refaire une partie");
                                return Ok(());
                            }
                        }

                    }else{
                        println!("Vous êtes chanceux le mob ne vous attaque pas.")
                    }
                }
                
            }
            "nord" | "sud" | "est" | "ouest" => {
                se_deplacer(&mut zones, &mut current_zone_index, &choix, &mut perso_joueur, &pnjs);
                if rng.random_range(0..99) < 10 {
                    affichage::notifier(&zones[current_zone_index], "🎉 L'événement rare s'est produit !", &pnjs);
                }
            }
            _ => {
                if let Ok(num) = choix.parse::<usize>() {
                    if (1..=nbr_coffres).contains(&num) {
                        let coffre = &mut zones[current_zone_index].coffres[num-1]; // Récupère le coffre sélectionné
                        match coffre.ouvrir() {
                            Some(objet) => {
                                perso_joueur.inventaire.ajouter_objet(objet as u8);
                                if coffre.inventaire.objets.is_empty() {
                                    zones[current_zone_index].supprimer_coffre(num-1);
                                }
                            },
                            None => affichage::notifier(&zones[current_zone_index], "Aucun objet à récupérer", &pnjs),
                        }
                    } else {
                        affichage::notifier(&zones[current_zone_index], "❌ Commande inconnue !", &pnjs);
                    }
                } else {
                    affichage::notifier(&zones[current_zone_index], "❌ Commande inconnue !", &pnjs);
                }
            },
        }
        if !perso_joueur.est_vivant {
            println!("Vous êtes mort... La partie est terminée !");
            break Ok(());
        }
    }
}

