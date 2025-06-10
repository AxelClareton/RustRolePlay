use std::fmt;
use std::fs::{File};
use std::io::{self, Read, Write};
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::inventaire::{Inventaire, ObjetInventaire};
use crate::objet::OBJETS_DISPONIBLES;
use crate::Zone;

/// Représente l’état d’une partie du corps.
///
/// Peut être :
/// - `Saine` : la partie est intacte.
/// - `Blessee(u8)` : blessée à un certain pourcentage.
/// - `Morte` : complètement détruite.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EtatPartie {
    Saine,
    Blessee(u8),
    Morte,
}


/// Structure représentant une partie du corps avec sa vie, son état de santé,
/// un temps de guérison et un inventaire d’équipement.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PartieDuCorps {
    nom: String,
    vie_max: u32,
    vie_actuelle: u32,
    etat: EtatPartie,
    guerison: DateTime<Utc>,
    equipement: crate::inventaire::Inventaire,
}

impl fmt::Display for EtatPartie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EtatPartie::Saine => write!(f, "Saine"),
            EtatPartie::Blessee(pourcentage) => write!(f, "Blessée ({}%)", pourcentage),
            EtatPartie::Morte => write!(f, "Morte"),
        }
    }
}

impl fmt::Display for PartieDuCorps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  - Partie : {}", self.nom)?;
        writeln!(f, "    Vie    : {}/{}", self.vie_actuelle, self.vie_max)?;
        writeln!(f, "    État   : {}", self.etat)?;
        writeln!(f, "    Guérison prévue : {}", self.guerison)?;
        writeln!(f, "    Équipement :")?;
        writeln!(f, "{}", self.equipement)
    }
}

impl PartieDuCorps {
    /// Crée une nouvelle partie du corps avec un nom et une vie maximale.
    ///
    /// L’état initial est `Saine`, avec un équipement vide.
    pub fn new(nom: String, vie_max: u32) -> Self {
        Self {
            nom,
            vie_max,
            vie_actuelle: vie_max,
            etat: EtatPartie::Saine,
            guerison: Utc::now(),
            equipement: Inventaire { taille: 1, objets: vec![] },
        }
    }

    /// Vérifie si la partie est saine et a terminé sa guérison.
    pub fn est_saine(&self) -> bool {
        matches!(self.etat, EtatPartie::Saine) && Utc::now() >= self.guerison
    }

    /// Vérifie si la partie est morte.
    pub fn est_morte(&self) -> bool {
        matches!(self.etat, EtatPartie::Morte)
    }

    /// Vérifie si la partie est bléssée.
    pub fn est_blessee(&self) -> bool {
        matches!(self.etat, EtatPartie::Blessee(_))
    }

    /// Renvoie le pourcentage de vie restante (0 à 100).
    pub fn pourcentage_vie(&self) -> f32 {
        if self.vie_max == 0 {
            return 0.0;
        }
        (self.vie_actuelle as f32 / self.vie_max as f32) * 100.0
    }


    /// Inflige des dégâts à la partie du corps.
    ///
    /// Met à jour la vie actuelle, l’état (`Blessee` ou `Morte`),
    /// et calcule un délai de guérison.
    ///
    /// # Retour
    /// `true` si la partie est détruite, `false` sinon.
    pub fn subir_degats(&mut self, degats: u32) -> bool {
        if self.est_morte() {
            return false;
        }

        if degats >= self.vie_actuelle {
            self.vie_actuelle = 0;
            self.etat = EtatPartie::Morte;
            println!("{} est maintenant détruite !", self.nom);
            return true;
        }

        self.vie_actuelle -= degats;
        let pourcentage_vie_restante = self.pourcentage_vie();
        
        let pourcentage_blessure = 100 - pourcentage_vie_restante as u8;
        
        if pourcentage_blessure > 0 {
            self.etat = EtatPartie::Blessee(pourcentage_blessure);
            let temps_guerison = chrono::Duration::minutes(pourcentage_blessure as i64);
            self.guerison = Utc::now() + temps_guerison;
            
            println!("{} est blessée à {}% (vie: {}/{})", 
                self.nom, pourcentage_blessure, self.vie_actuelle, self.vie_max);
        }

        false
    }

    /// Renvoie le nom de la partie.
    pub fn nom(&self) -> &str {
        &self.nom
    }

    /// Renvoie une référence à l'inventaire de la partie.
    pub fn equipement(&self) -> &crate::inventaire::Inventaire {
        &self.equipement
    }

    /// Ajoute un objet à l’équipement de cette partie.
    pub fn ajouter_equipement(&mut self, objet : u8){
        let _ = &self.equipement.ajouter_objet(objet);
    }

    /// Récupère un objet de l’inventaire à un index donné.
    ///
    /// Diminue la quantité ou supprime l’objet si sa quantité tombe à 0.
    pub fn récupérer_objet(&mut self, index: usize) -> ObjetInventaire {
        let objet = self.equipement.objets[index].clone();
        self.equipement.objets[index].nombre -= 1;
        if self.equipement.objets[index].nombre == 0 {
            self.equipement.objets.remove(index);
        }
        self.equipement.trier_quantite();

        objet
    }

    /// Renvoie la vie actuelle.
    pub fn vie_actuelle(&self) -> u32 {
        self.vie_actuelle
    }

    /// Renvoie la vie maximale.
    pub fn vie_max(&self) -> u32 {
        self.vie_max
    }

    /// Renvoie l’état actuel.
    pub fn etat(&self) -> &EtatPartie {
        &self.etat
    }

    /// Renvoie la date de fin de guérison.
    pub fn guerison(&self) -> chrono::DateTime<chrono::Utc> {
        self.guerison
    }
}

/// Représente le résultat d'une blessure infligée à un personnage.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResultatBlessure {
    /// La partie est mort.
    Mort,
    /// Une partie du corps est détruite.
    PartieDetruite,
    /// Une blessure a été infligée, mais pas mortelle.
    Blesse,
    /// Aucun dégât grave.
    RienGrave,
}

/// Représente un personnage jouable ou non-jouable dans le jeu.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Personnage {
    /// Identifiant unique du personnage.
    pub id: u32,
    /// Nom du personnage.
    pub nom: String,
    /// Brève description du personnage.
    pub description: String,
    /// Force physique du personnage.
    pub force: u8,
    /// Inventaire d’objets possédé par le personnage.
    pub inventaire: crate::inventaire::Inventaire,
    /// Liste des parties du corps et leur état.
    pub parties_du_corps: Vec<PartieDuCorps>,
    /// Montant d'argent possédé.
    pub argent: u32,
    /// Indique si le personnage est vivant.
    pub est_vivant: bool,
}

impl fmt::Display for Personnage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Personnage #{} ===", self.id)?;
        writeln!(f, "Nom         : {}", self.nom)?;
        writeln!(f, "Description : {}", self.description)?;
        writeln!(f, "Force       : {}", self.force)?;
        writeln!(f, "Argent      : {}", self.argent)?;
        writeln!(f, "Parties du corps :")?;
        for partie in &self.parties_du_corps {
            writeln!(f, "{}", partie)?;
        }
        Ok(())
    }
}

impl Personnage {
    /// Gère les dégâts reçus par une partie du corps et retourne le résultat.
    ///
    /// # Arguments
    ///
    /// * `nom_partie` - Le nom de la partie ciblée.
    /// * `degats` - Le montant de dégâts infligés.
    pub fn gerer_blessure(&mut self, nom_partie: &str, degats: u32) -> ResultatBlessure {
        if let Some(partie) = self.parties_du_corps.iter_mut()
            .find(|p| p.nom.to_lowercase() == nom_partie.to_lowercase()) {
            
            let partie_detruite = partie.subir_degats(degats);
            
            if partie_detruite && (partie.nom.to_lowercase().contains("tête") || partie.nom.to_lowercase().contains("torse")) {
                self.est_vivant = false;
                println!("{} est mort suite à une blessure mortelle à la {} !", self.nom, partie.nom);
                return ResultatBlessure::Mort;
            }
            
            if partie_detruite {
                return ResultatBlessure::PartieDetruite;
            }
            
            if partie.est_blessee() {
                return ResultatBlessure::Blesse;
            }
        } else {
            println!("Partie du corps '{}' non trouvée !", nom_partie);
        }
        
        ResultatBlessure::RienGrave
    }

    /// Vérifie si le personnage peut encore se battre.
    pub fn peut_se_battre(&self) -> bool {
        if !self.est_vivant {
            return false;
        }
        let tete_vivante = self.parties_du_corps.iter().any(|p| p.nom.to_lowercase().contains("tête") && !p.est_morte());
        let torse_vivant = self.parties_du_corps.iter().any(|p| p.nom.to_lowercase().contains("torse") && !p.est_morte());
        if !tete_vivante || !torse_vivant {
            return false;
        }
        let bras_fonctionnels = self.parties_du_corps.iter().filter(|p| p.nom.to_lowercase().contains("bras") && !p.est_morte()).count();
        let jambes_fonctionnelles = self.parties_du_corps.iter().filter(|p| p.nom.to_lowercase().contains("jambe") && !p.est_morte()).count();
        bras_fonctionnels > 0 && jambes_fonctionnelles > 0
    }

    /// Calcule la force effective du personnage en fonction des blessures.
    pub fn force_effective(&self) -> u8 {
        if !self.est_vivant {
            return 0;
        }

        let mut modificateur = 1.0;
        
        for partie in &self.parties_du_corps {
            match &partie.etat {
                EtatPartie::Morte => {
                    if partie.nom.to_lowercase().contains("bras") {
                        modificateur *= 0.7;
                    } else if partie.nom.to_lowercase().contains("jambe") {
                        modificateur *= 0.8;
                    }
                },
                EtatPartie::Blessee(pourcentage) => {
                    let reduction = (*pourcentage as f32) / 200.0;
                    modificateur *= 1.0 - reduction;
                },
                EtatPartie::Saine => {},
            }
        }

        ((self.force as f32) * modificateur) as u8
    }

    /// Lit un fichier JSON contenant une liste de personnages.
    ///
    /// Retourne `Ok(Some(vec))` si des personnages sont trouvés,
    /// `Ok(None)` si le fichier est vide ou introuvable.
    fn lire_fichier_json(fichier: &str) -> io::Result<Option<Vec<Personnage>>> {
        let mut file = match File::open(fichier) {
            Ok(file) => file,
            Err(_) => return Ok(None),
        };
        
        let mut contenu = String::new();
        file.read_to_string(&mut contenu)?;
        
        if contenu.trim().is_empty() {
            return Ok(None);
        }
        
        match serde_json::from_str(&contenu) {
            Ok(personnages) => Ok(Some(personnages)),
            Err(_) => Ok(None),
        }
    }

    /// Sauvegarde ce personnage dans un fichier JSON existant ou nouveau.
    ///
    /// # Arguments
    ///
    /// * `fichier` - Le chemin du fichier dans lequel sauvegarder.
    pub fn sauvegarder_json(&self, fichier: &str) -> io::Result<()> {
        let mut personnages = match Self::lire_fichier_json(fichier)? {
            Some(personnages) => personnages,
            None => vec![],
        };
        
        personnages.push(self.clone());
        let json = serde_json::to_string_pretty(&personnages)?;
        let mut file = File::create(fichier)?;
        file.write_all(json.as_bytes())
    }

    /// Charge une liste de personnages à partir d’un fichier JSON.
    /// Crée des personnages de test si le fichier est vide ou manquant.
    pub fn charger_depuis_json(fichier: &str) -> io::Result<Vec<Personnage>> {
        match Self::lire_fichier_json(fichier)? {
            Some(personnages) => Ok(personnages),
            None => {
                if fichier.contains("pnj") {
                    crate::personnage::PNJ::creer_pnjs_test_direct()?;
                } else if fichier.contains("mob") {
                    crate::personnage::Mob::creer_mobs_test_direct()?;
                } else if fichier.contains("personnage") {
                    crate::personnage::Joueur::creer_joueur_test_direct()?;
                }
                
                match Self::lire_fichier_json(fichier)? {
                    Some(personnages) => Ok(personnages),
                    None => Ok(vec![]),
                }
            }
        }
    }

    /// Calcule le prochain identifiant disponible en fonction des personnages existants.
    pub fn prochain_id(fichier: &str) -> io::Result<u32> {
        let personnages = match Self::lire_fichier_json(fichier)? {
            Some(personnages) => personnages,
            None => vec![],
        };
        let max_id = personnages.iter().map(|p| p.id).max().unwrap_or(0);
        Ok(max_id + 1)
    }

    /// Ajoute de l’argent au personnage.
    pub fn ajouter_argent(&mut self, montant: u32) {
        self.argent += montant;
    }

    /// Retire de l’argent au personnage.
    pub fn retirer_argent(&mut self, montant: u32) {
        if self.argent < montant {
            println!("Vous n'avez pas assez d'argent !");
        } else {
            self.argent -= montant;
        }
    }

    /// Soigne toutes les parties du corps après un combat.
    pub fn soigner_apres_combat(&mut self) {
        if !self.est_vivant {
            return;
        }
        let maintenant = chrono::Utc::now();
        for partie in &mut self.parties_du_corps {
            if partie.est_morte() {
                partie.vie_actuelle = partie.vie_max;
                partie.etat = EtatPartie::Saine;
                partie.guerison = maintenant + chrono::Duration::hours(1);
            } else if partie.vie_actuelle < partie.vie_max {
                partie.vie_actuelle = partie.vie_max;
                partie.etat = EtatPartie::Saine;
                partie.guerison = maintenant;
            }
        }
    }
}

/// Crée et retourne les parties du corps standards d’un personnage.
fn creer_parties_du_corps() -> Vec<PartieDuCorps> {
    vec![
        PartieDuCorps::new("Tête".to_string(), 50),
        PartieDuCorps::new("Torse".to_string(), 100),
        PartieDuCorps::new("Bras droit".to_string(), 75),
        PartieDuCorps::new("Bras gauche".to_string(), 75),
        PartieDuCorps::new("Jambe droite".to_string(), 80),
        PartieDuCorps::new("Jambe gauche".to_string(), 80),
    ]
}

/// Représente un joueur contrôlé par l'utilisateur.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Joueur {
    /// Le personnage associé au joueur.
    pub personnage: Personnage,
}

/// Représente un personnage non-joueur (PNJ), comme un marchand ou un habitant.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PNJ {
    /// Le personnage associé au PNJ.
    pub personnage: Personnage,
    /// Liste des dialogues que le PNJ peut prononcer.
    pub dialogues: Vec<String>,
    /// Identifiant de la zone où se trouve ce PNJ.
    pub zone_id: u32,
    /// Multiplicateur appliqué aux prix (ex. : pour vendre plus cher).
    pub multiplicateur_prix: f32,
}

impl PNJ {
    /// Calcule le prochain identifiant disponible pour un PNJ à partir du fichier JSON.
    ///
    /// # Arguments
    ///
    /// * `fichier` - Le chemin du fichier JSON contenant les PNJ existants.
    ///
    /// # Retour
    ///
    /// Retourne l'identifiant suivant disponible.
    pub fn prochain_id_pnj(fichier: &str) -> io::Result<u32> {
        let mut file = File::open(fichier)?;
        let mut contenu = String::new();
        file.read_to_string(&mut contenu)?;

        let pnjs: Vec<PNJ> = serde_json::from_str(&contenu)?;

        let max_id = pnjs.iter()
            .map(|pnj| pnj.personnage.id)
            .max()
            .unwrap_or(0);

        Ok(max_id + 1)
    }

    /// Crée un nouveau PNJ et le sauvegarde dans le fichier JSON.
    ///
    /// # Arguments
    ///
    /// * `nom` - Nom du PNJ.
    /// * `description` - Description du PNJ.
    /// * `dialogues` - Liste de dialogues que le PNJ peut utiliser.
    /// * `zone_id` - ID de la zone dans laquelle il se trouve.
    /// * `multiplicateur_prix` - Facteur multiplicatif appliqué aux prix des objets.
    ///
    /// # Retour
    ///
    /// Retourne le PNJ créé.
    pub fn creer_pnj(
        nom: &str, 
        description: &str, 
        dialogues: Vec<String>, 
        zone_id: u32, 
        multiplicateur_prix: f32
    ) -> io::Result<Self> {
        let prochain_id = PNJ::prochain_id_pnj("src/json/pnj.json")?;
        let inventaire = PNJ::choisir_objets_inventaire()?;
        let parties_du_corps = creer_parties_du_corps();

        let mut rng: ThreadRng = rand::rng();
        let valeur = rng.random_range(80..120);
        let valeur2 = rng.random_range(50..200); // Plus d'argent pour les marchands

        let personnage = Personnage {
            id: prochain_id,
            nom: nom.to_string(),
            description: description.to_string(),
            force: valeur,
            inventaire,
            parties_du_corps,
            argent: valeur2,
            est_vivant: true,
        };

        let pnj = PNJ {
            personnage,
            dialogues,
            zone_id,
            multiplicateur_prix,
        };

        pnj.sauvegarder_pnj("src/json/pnj.json")?;
        Ok(pnj)
    }

    /// Permet de sélectionner les objets à ajouter dans l’inventaire du PNJ à partir d’une
    /// liste d’objets disponibles.
    ///
    /// Affiche tous les objets disponibles, et demande à l'utilisateur de saisir
    /// les objets et quantités sous le format `id:quantité`.
    ///
    /// La somme des quantités ne peut pas dépasser la capacité maximale de l'inventaire (10).
    ///
    /// # Exemple d'entrée utilisateur
    ///
    /// `1:3,2:2` — ajoute 3 objets d'ID 1 et 2 objets d'ID 2.
    ///
    /// # Retour
    ///
    /// Retourne un `Inventaire` rempli avec les objets choisis.
    pub fn choisir_objets_inventaire() -> io::Result<Inventaire> {
        let objets_disponibles = OBJETS_DISPONIBLES.read().unwrap();
        let mut inventaire = Inventaire { taille: 10, objets: Vec::new() };

        loop {
            println!("Entrez les objets que le marchand aura parmi ceux-ci :");
            for (id, objet) in objets_disponibles.iter() {
                println!("{}: {}", id, objet.nom);
            }

            println!("Précisez le nombre d'exemplaires pour chaque objet sous cette forme : numéro_de_l'objet:quantité, numéro_de_l'objet:quantité");
            println!("Par exemple : 1:3,2:5");
            print!("Votre choix : ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let choix: Vec<&str> = input.trim().split(',').collect();
            let mut total_quantite = 0;

            for choix_item in &choix {
                let parts: Vec<&str> = choix_item.split(':').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    if let Ok(objet_id) = parts[0].parse::<u8>() {
                        if let Ok(quantite) = parts[1].parse::<u8>() {
                            if objets_disponibles.contains_key(&objet_id) {
                                total_quantite += quantite;
                            } else {
                                println!("Objet avec l'ID {} non trouvé.", objet_id);
                            }
                        } else {
                            println!("Quantité invalide : {}", parts[1]);
                        }
                    } else {
                        println!("ID d'objet invalide : {}", parts[0]);
                    }
                } else {
                    println!("Format invalide : {}", choix_item);
                }
            }

            if total_quantite <= 10 {
                for choix_item in &choix {
                    let parts: Vec<&str> = choix_item.split(':').map(|s| s.trim()).collect();
                    if parts.len() == 2 {
                        if let Ok(objet_id) = parts[0].parse::<u8>() {
                            if let Ok(quantite) = parts[1].parse::<u8>() {
                                if objets_disponibles.contains_key(&objet_id) {
                                    for _ in 0..quantite {
                                        inventaire.ajouter_objet(objet_id);
                                    }
                                }
                            }
                        }
                    }
                }
                break;
            } else {
                println!("La somme des quantités d'objets ne doit pas dépasser 10. Veuillez réessayer.");
                inventaire.objets.clear(); // Vider l'inventaire pour réessayer
            }
        }

        Ok(inventaire)
    }

    /// Sauvegarde ce PNJ dans un fichier JSON.
    ///
    /// Si le fichier existe, ajoute ce PNJ à la liste existante.
    /// Sinon, crée un nouveau fichier avec ce PNJ seul.
    ///
    /// # Arguments
    ///
    /// * `fichier` - Le chemin du fichier JSON où sauvegarder les PNJs.
    ///
    /// # Erreurs
    ///
    /// Retourne une erreur si l'écriture dans le fichier échoue.
    pub fn sauvegarder_pnj(&self, fichier: &str) -> io::Result<()> {
        let mut pnjs = match Self::lire_pnjs_json(fichier)? {
            Some(pnjs) => pnjs,
            None => vec![],
        };
        
        pnjs.push(self.clone());
        let json = serde_json::to_string_pretty(&pnjs)?;
        let mut file = File::create(fichier)?;
        file.write_all(json.as_bytes())
    }

    /// Lit la liste des PNJs depuis un fichier JSON.
    ///
    /// # Arguments
    ///
    /// * `fichier` - Le chemin du fichier JSON contenant la liste des PNJs.
    ///
    /// # Retour
    ///
    /// Retourne `Ok(Some(Vec<PNJ>))` si la lecture et la désérialisation réussissent.
    /// Retourne `Ok(None)` si le fichier est vide ou introuvable.
    ///
    /// # Erreurs
    ///
    /// Retourne une erreur si la lecture du fichier échoue.
    fn lire_pnjs_json(fichier: &str) -> io::Result<Option<Vec<PNJ>>> {
        let mut file = match File::open(fichier) {
            Ok(file) => file,
            Err(_) => return Ok(None),
        };
        
        let mut contenu = String::new();
        file.read_to_string(&mut contenu)?;
        
        if contenu.trim().is_empty() {
            return Ok(None);
        }
        
        match serde_json::from_str(&contenu) {
            Ok(pnjs) => Ok(Some(pnjs)),
            Err(_) => Ok(None),
        }
    }

    /// Charge la liste des PNJs depuis un fichier JSON.
    ///
    /// Si aucun PNJ n'est trouvé, crée des PNJs de test et réessaie la lecture.
    ///
    /// # Arguments
    ///
    /// * `fichier` - Le chemin du fichier JSON.
    ///
    /// # Retour
    ///
    /// Retourne la liste des PNJs ou une liste vide si aucun PNJ n'est disponible.
    ///
    /// # Erreurs
    ///
    /// Retourne une erreur en cas d'échec de lecture ou création des PNJs de test.
    pub fn charger_pnj(fichier: &str) -> io::Result<Vec<PNJ>> {
        match Self::lire_pnjs_json(fichier)? {
            Some(pnjs) => Ok(pnjs),
            None => {
                println!("Aucun PNJ trouvé, création de PNJs de test...");
                Self::creer_pnjs_test_direct()?;
                match Self::lire_pnjs_json(fichier)? {
                    Some(pnjs) => Ok(pnjs),
                    None => Ok(vec![]),
                }
            }
        }
    }

    /// Crée et sauvegarde une liste de PNJs marchands de test dans un fichier JSON.
    ///
    /// Les PNJs de test ont des dialogues, des multiplicateurs de prix, et un inventaire vide.
    ///
    /// # Retour
    ///
    /// Retourne `Ok(())` si la création et la sauvegarde réussissent.
    ///
    /// # Erreurs
    ///
    /// Retourne une erreur si l'écriture dans le fichier JSON échoue.
    pub fn creer_pnjs_test_direct() -> io::Result<()> {
        // Possible de leur mettre l'inventaire içi pour choisir les objets qu'ils vendent
        let pnjs_test = vec![
            (
                "Marcus le Marchand",
                "Un marchand expérimenté qui vend des équipements de qualité",
                vec![
                    "Bienvenue dans ma boutique !".to_string(),
                    "J'ai les meilleurs équipements de la région.".to_string(),
                    "Mes prix sont justes pour la qualité proposée.".to_string(),
                    "Revenez me voir quand vous voulez !".to_string(),
                ],
                1,
                1.2, // Prix 20% plus cher
            ),
            (
                "Elena la Guérisseuse",
                "Une soigneuse qui vend potions et remèdes",
                vec![
                    "Que puis-je faire pour votre santé ?".to_string(),
                    "Mes potions sont préparées avec les meilleures herbes.".to_string(),
                    "La santé n'a pas de prix, mais mes potions si !".to_string(),
                    "Prenez soin de vous, aventurier.".to_string(),
                ],
                2,
                1.5, // Prix 50% plus cher (produits rares)
            ),
            (
                "Gareth le Garde-Marchand",
                "Un ancien garde qui vend des armes et armures",
                vec![
                    "Vous cherchez de l'équipement de combat ?".to_string(),
                    "J'ai servi dans l'armée, je connais le bon matériel.".to_string(),
                    "Ces armes ont fait leurs preuves au combat.".to_string(),
                    "Que vos armes vous portent chance !".to_string(),
                ],
                3,
                1.1, // Prix 10% plus cher
            ),
            (
                "Lydia l'Informatrice",
                "Une espionne qui vend des informations et objets rares",
                vec![
                    "Chut... Approchez-vous, j'ai ce qu'il vous faut.".to_string(),
                    "Mes objets sont... difficiles à trouver ailleurs.".to_string(),
                    "Gardez le secret sur nos transactions.".to_string(),
                    "Les murs ont des oreilles, soyez discret.".to_string(),
                ],
                4,
                2.0, // Prix doublés
            ),
            (
                "Thomas le Forgeron",
                "Un artisan qui fabrique et vend des outils",
                vec![
                    "Mes outils sont forgés avec passion !".to_string(),
                    "Rien ne vaut un bon outil bien fait.".to_string(),
                    "Je garantis la qualité de mes créations.".to_string(),
                    "Que votre travail soit fructueux !".to_string(),
                ],
                5,
                0.9, // Prix 10% moins cher
            ),
        ];

        let mut pnjs = vec![];

        for (id, (nom, description, dialogues, zone_id, multiplicateur_prix)) in pnjs_test.into_iter().enumerate() {
            let inventaire = Inventaire { taille: 10, objets: vec![] };
            let parties_du_corps = creer_parties_du_corps();
            let mut rng: ThreadRng = rand::rng();
            let valeur = rng.random_range(100..140); // plus de force pour les marchands
            let valeur2 = rng.random_range(100..300); // Plus d'argent pour les marchands

            let personnage = Personnage {
                id: (id + 1) as u32,
                nom: nom.to_string(),
                description: description.to_string(),
                force: valeur,
                inventaire,
                parties_du_corps,
                argent: valeur2,
                est_vivant: true,
            };

            let pnj = PNJ {
                personnage,
                dialogues,
                zone_id,
                multiplicateur_prix,
            };

            pnjs.push(pnj);
        }

        let json = serde_json::to_string_pretty(&pnjs)?;
        let mut file = File::create("src/json/pnj.json")?;
        file.write_all(json.as_bytes())?;
        
        println!("5 PNJs marchands de test créés avec succès !");
        Ok(())
    }

    /// Gère l’interaction entre le joueur et ce PNJ.
    ///
    /// Propose au joueur plusieurs choix : combattre, voir l’inventaire, ou quitter.
    ///
    /// # Arguments
    /// * `joueur` - Référence mutable au personnage joueur.
    /// * `zones` - Référence mutable à la liste des zones du jeu.
    /// * `current_zone_index` - Indice de la zone actuelle dans laquelle se trouve le joueur.
    pub fn interagir(&mut self, joueur: &mut Personnage, zones: &mut Vec<Zone>, current_zone_index: usize) {
        println!("Vous rencontrez {}. Que voulez-vous faire ?", self.personnage.nom);
        println!("1. Combattre");
        println!("2. Voir l'inventaire");
        println!("3. Quitter");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture !");

        match choix.trim() {
            "1" => {
                println!("Vous avez choisi de combattre !");
                let resultat = crate::combat::combattre(
                    joueur.clone(),
                    self.personnage.clone(),
                    &zones[current_zone_index],
                    &crate::personnage::PNJ::charger_pnj("src/json/pnj.json").unwrap_or_default()
                );
                if resultat.etat_final_joueur.est_vivant {
                    *joueur = resultat.etat_final_joueur;
                    println!("Vous avez gagné le combat contre le PNJ !");
                    // Drop de l'inventaire du PNJ
                    for objet in &self.personnage.inventaire.objets {
                        for _ in 0..objet.nombre {
                            zones[current_zone_index].objet_zone.ajouter_objet(objet.objet_id);
                        }
                    }
                    self.personnage.inventaire.objets.clear();
                    self.personnage.est_vivant = false;
                    // Récupération de l'argent
                    joueur.ajouter_argent(self.personnage.argent);
                    println!("Vous ramassez {} d'argent sur le PNJ !", self.personnage.argent);
                    self.personnage.argent = 0;
                } else {
                    *joueur = resultat.etat_final_joueur;
                    joueur.est_vivant = false;
                    println!("Vous avez perdu le combat contre le PNJ...");
                }
            }
            "2" => {
                self.afficher_inventaire();
                self.acheter_objet(joueur);
            }
            "3" => {
                println!("Vous quittez l'interaction avec le PNJ.");
            }
            _ => println!("Choix invalide !"),
        }
    }


    /// Permet au joueur d’acheter un objet dans l’inventaire du PNJ.
    ///
    /// Vérifie la quantité disponible, l’argent du joueur, et la place dans son inventaire.
    fn afficher_inventaire(&self) {
        println!("Inventaire de {}:", self.personnage.nom);
        for (index, objet) in self.personnage.inventaire.objets.iter().enumerate() {
            if let Some(o) = OBJETS_DISPONIBLES.read().unwrap().get(&objet.objet_id) {
                println!("{}: {} (x{}) - Prix: {} /unité",
                         index + 1,
                         o.nom,
                         objet.nombre,
                         self.calculer_prix_vente(o.prix));
            }
        }
    }

    /// Calcule le prix de vente d’un objet en appliquant le multiplicateur de prix du PNJ.
    ///
    /// # Arguments
    /// * `prix_base` - Prix de base de l'objet.
    ///
    /// # Retour
    /// Le prix ajusté selon le multiplicateur du PNJ.
    fn acheter_objet(&mut self, joueur: &mut Personnage) {
        println!("Vous avez {} d'argent.", joueur.argent);
        println!("Entrez le numéro de l'objet que vous souhaitez acheter ou 'q' pour quitter :");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture !");

        if choix.trim().eq_ignore_ascii_case("q") {
            println!("Vous quittez l'interaction avec le PNJ.");
            return;
        }

        if let Ok(index) = choix.trim().parse::<usize>() {
            if index > 0 && index <= self.personnage.inventaire.objets.len() {
                let objet_inv = self.personnage.inventaire.objets[index - 1].clone();
                if let Some(objet) = OBJETS_DISPONIBLES.read().unwrap().get(&objet_inv.objet_id) {
                    println!("Combien voulez-vous acheter de {} ?", objet.nom);
                    let mut quantite = String::new();
                    io::stdin().read_line(&mut quantite).expect("Erreur de lecture !");

                    if let Ok(quantite) = quantite.trim().parse::<u8>() {
                        let prix_total = self.calculer_prix_vente(objet.prix) * quantite as u32;
                        let total_objet = joueur.inventaire.objets.iter().map(|o| o.nombre).sum::<u8>() + quantite;
                        if quantite <= objet_inv.nombre && joueur.argent >= prix_total && total_objet <= joueur.inventaire.taille {
                            joueur.retirer_argent(prix_total);
                            self.personnage.argent += prix_total;

                            // Ajouter l'objet à l'inventaire du joueur
                            for _ in 0..quantite {
                                joueur.inventaire.ajouter_objet(objet_inv.objet_id);
                            }

                            // Retirer l'objet de l'inventaire du PNJ
                            self.personnage.inventaire.objets[index - 1].nombre -= quantite;
                            if self.personnage.inventaire.objets[index - 1].nombre == 0 {
                                self.personnage.inventaire.objets.remove(index - 1);
                            }

                            println!("Achat réussi !");
                        } else {
                            println!("Quantité invalide ou pas assez d'argent !");
                        }
                    }
                }
            } else {
                println!("Numéro d'objet invalide !");
            }
        } else {
            println!("Entrée invalide !");
        }
    }

    pub fn calculer_prix_vente(&self, prix_base: u32) -> u32 {
        ((prix_base as f32) * self.multiplicateur_prix) as u32
    }
}

// Affichage personnalisé pour les PNJ
impl fmt::Display for PNJ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== PNJ Marchand #{} ===", self.personnage.id)?;
        writeln!(f, "Nom         : {}", self.personnage.nom)?;
        writeln!(f, "Description : {}", self.personnage.description)?;
        writeln!(f, "Zone        : {}", self.zone_id)?;
        writeln!(f, "Multiplicateur prix : {:.1}", self.multiplicateur_prix)?;
        writeln!(f, "Argent      : {}", self.personnage.argent)?;
        writeln!(f, "Dialogues   :")?;
        for (i, dialogue) in self.dialogues.iter().enumerate() {
            writeln!(f, "  {}: \"{}\"", i + 1, dialogue)?;
        }
        Ok(())
    }
}
/// Représente un Mob (monstre ou créature non-joueur) dans le jeu.
///
/// Contient un personnage avec ses attributs (force, inventaire, etc.).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mob {
    /// Le personnage associé au Mob.
    pub personnage: Personnage,
}

impl Mob {
    /// Crée un nouveau Mob avec un nom et une description.
    ///
    /// Le Mob est initialisé avec des valeurs aléatoires pour la force et l'argent,
    /// un inventaire vide et des parties du corps générées.
    /// Le Mob est sauvegardé dans le fichier JSON spécifié.
    ///
    /// # Arguments
    ///
    /// * `nom` - Nom du Mob.
    /// * `description` - Description du Mob.
    ///
    /// # Erreurs
    ///
    /// Retourne une erreur d'entrée/sortie si la sauvegarde échoue.
    pub fn creer_mob(nom: &str, description: &str) -> io::Result<Self> {
        let prochain_id = Personnage::prochain_id("src/json/mob.json")?;
        let inventaire = Inventaire { taille: 10, objets: vec![] };
        let parties_du_corps = creer_parties_du_corps();

        let mut rng: ThreadRng = rand::rng();
        let valeur = rng.random_range(80..120);
        let valeur2 = rng.random_range(0..20);

        let personnage = Personnage {
            id: prochain_id,
            nom: nom.to_string(),
            description: description.to_string(),
            force: valeur,
            inventaire,
            parties_du_corps,
            argent: valeur2,
            est_vivant: true,
        };

        personnage.sauvegarder_json("src/json/mob.json")?;
        Ok(Mob { personnage })
    }

    /// Charge la liste des Mobs depuis un fichier JSON.
    ///
    /// Si aucun Mob n'est trouvé, crée une série de Mobs de test puis les charge.
    ///
    /// # Arguments
    ///
    /// * `fichier` - Chemin vers le fichier JSON des Mobs.
    ///
    /// # Erreurs
    ///
    /// Retourne une erreur d'entrée/sortie si le fichier ne peut être lu ou parsé.
    pub fn charger_mob(fichier: &str) -> io::Result<Vec<Personnage>> {
        let mobs = Personnage::charger_depuis_json(fichier)?;
        if mobs.is_empty() {
            println!("Aucun Mob trouvé, création de Mobs de test...");
            Self::creer_mobs_test_direct()?;
            return Personnage::charger_depuis_json(fichier);
        }
        Ok(mobs)
    }

    /// Crée directement plusieurs Mobs de test avec des valeurs prédéfinies.
    ///
    /// Les Mobs sont sauvegardés dans `src/json/mob.json`.
    ///
    /// # Erreurs
    ///
    /// Retourne une erreur d'entrée/sortie si la sauvegarde échoue.
    pub fn creer_mobs_test_direct() -> io::Result<()> {
        let mobs_test = vec![
            ("Gobelin Sauvage", "Un petit gobelin agressif aux dents pointues"),
            ("Orc Guerrier", "Un orc massif brandissant une hache"),
            ("Loup des Ténèbres", "Un loup noir aux yeux rougeoyants"),
            ("Squelette Ancien", "Un squelette animé par une magie noire"),
            ("Araignée Géante", "Une araignée venimeuse de la taille d'un cheval"),
            ("Bandit Masqué", "Un voleur de grand chemin sans scrupules"),
            ("Troll des Marais", "Une créature putride qui régénère ses blessures")
        ];

        let mut personnages = vec![];
        let mut current_id = 1;

        for (nom, description) in mobs_test {
            let inventaire = Inventaire { taille: 10, objets: vec![] };
            let parties_du_corps = creer_parties_du_corps();
            let mut rng: ThreadRng = rand::rng();
            let valeur = rng.random_range(80..120);
            let valeur2 = rng.random_range(0..20);

            let personnage = Personnage {
                id: current_id,
                nom: nom.to_string(),
                description: description.to_string(),
                force: valeur,
                inventaire,
                parties_du_corps,
                argent: valeur2,
                est_vivant: true,
            };

            personnages.push(personnage);
            current_id += 1;
        }

        let json = serde_json::to_string_pretty(&personnages)?;
        let mut file = File::create("src/json/mob.json")?;
        file.write_all(json.as_bytes())?;
        
        println!("7 Mobs de test créés avec succès !");
        Ok(())
    }
    
}

impl Joueur {
    /// Crée un nouveau Joueur avec un nom et une description.
    ///
    /// Le Joueur est initialisé avec des valeurs aléatoires pour la force et l'argent,
    /// un inventaire vide et des parties du corps générées.
    /// Le Joueur est sauvegardé dans le fichier JSON spécifié.
    ///
    /// # Arguments
    ///
    /// * `nom` - Nom du Joueur.
    /// * `description` - Description du Joueur.
    ///
    /// # Erreurs
    ///
    /// Retourne une erreur d'entrée/sortie si la sauvegarde échoue.
    pub fn creer_joueur(nom: &str, description: &str) -> io::Result<Self> {
        let prochain_id = Personnage::prochain_id("src/json/personnage.json")?;
        let inventaire = Inventaire { taille: 10, objets: vec![] };
        let parties_du_corps = creer_parties_du_corps();

        let mut rng: ThreadRng = rand::rng();
        let valeur = rng.random_range(80..120);
        let valeur2 = rng.random_range(0..20);

        let personnage = Personnage {
            id: prochain_id,
            nom: nom.to_string(),
            description: description.to_string(),
            force: valeur,
            inventaire,
            parties_du_corps,
            argent: valeur2,
            est_vivant: true,
        };

        personnage.sauvegarder_json("src/json/personnage.json")?;
        Ok(Joueur { personnage })
    }

    /// Charge la liste des Joueurs depuis un fichier JSON.
    ///
    /// # Arguments
    ///
    /// * `fichier` - Chemin vers le fichier JSON des Joueurs.
    ///
    /// # Erreurs
    ///
    /// Retourne une erreur d'entrée/sortie si le fichier ne peut être lu ou parsé.
    pub fn charger_joueur(fichier: &str) -> io::Result<Vec<Personnage>> {
        Personnage::charger_depuis_json(fichier)
    }

    /// Crée directement plusieurs Joueurs de test avec des valeurs prédéfinies.
    ///
    /// Les Joueurs sont sauvegardés dans `src/json/personnage.json`.
    ///
    /// # Erreurs
    ///
    /// Retourne une erreur d'entrée/sortie si la sauvegarde échoue.
    pub fn creer_joueur_test_direct() -> io::Result<()> {
        let joueurs_test = vec![
            ("Marc le fou furieux", "Un aventurier téméraire à la recherche de gloire"),
            ("Alice l'Exploratrice", "Une exploratrice curieuse et rusée"),
            ("Bob le Bâtisseur", "Un constructeur habile avec un grand cœur"),
            ("Clara la Magicienne", "Une magicienne puissante maîtrisant les arcanes"),
            ("David le Guerrier", "Un guerrier robuste prêt à défendre ses alliés")
        ];

        let mut personnages = vec![];
        let mut current_id = 1;

        for (nom, description) in joueurs_test {
            let inventaire = Inventaire { taille: 10, objets: vec![] };
            let parties_du_corps = creer_parties_du_corps();
            let mut rng: ThreadRng = rand::rng();
            let valeur = rng.random_range(80..120);
            let valeur2 = rng.random_range(0..20);

            let personnage = Personnage {
                id: current_id,
                nom: nom.to_string(),
                description: description.to_string(),
                force: valeur,
                inventaire,
                parties_du_corps,
                argent: valeur2,
                est_vivant: true,
            };

            personnages.push(personnage);
            current_id += 1;
        }

        let json = serde_json::to_string_pretty(&personnages)?;
        let mut file = File::create("src/json/personnage.json")?;
        file.write_all(json.as_bytes())?;

        println!("5 joueurs de test créés avec succès !");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partie_du_corps_new_et_etat() {
        let partie = PartieDuCorps::new("Tête".to_string(), 100);
        assert_eq!(partie.nom(), "Tête");
        assert_eq!(partie.vie_actuelle(), 100);
        assert!(partie.est_saine());
        assert!(!partie.est_morte());
    }

    #[test]
    fn test_subir_degats_et_mort() {
        let mut partie = PartieDuCorps::new("Bras".to_string(), 10);
        let detruite = partie.subir_degats(15);
        assert!(detruite);
        assert!(partie.est_morte());
        assert_eq!(partie.vie_actuelle(), 0);
    }

    #[test]
    fn test_personnage_gerer_blessure_et_soigner() {
        let mut p = Personnage {
            id: 1,
            nom: "Test".to_string(),
            description: "desc".to_string(),
            force: 10,
            inventaire: crate::inventaire::Inventaire { taille: 5, objets: vec![] },
            parties_du_corps: vec![PartieDuCorps::new("Tête".to_string(), 10)],
            argent: 0,
            est_vivant: true,
        };
        let res = p.gerer_blessure("Tête", 15);
        assert!(matches!(res, ResultatBlessure::Mort));
        assert!(!p.est_vivant);
        // let soigner = p.soigner_partie("Tête", 5);
        // assert!(!soigner); // ne peut pas soigner un mort
    }
}
