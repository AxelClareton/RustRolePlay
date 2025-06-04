use std::fmt;
use std::fs::{File};
use std::io::{self, Read, Write};
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::inventaire::{Inventaire, ObjetInventaire};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EtatPartie {
    Saine,
    Blessee(u8),
    Morte,
}

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

    pub fn est_saine(&self) -> bool {
        matches!(self.etat, EtatPartie::Saine) && Utc::now() >= self.guerison
    }

    pub fn est_morte(&self) -> bool {
        matches!(self.etat, EtatPartie::Morte)
    }

    pub fn est_blessee(&self) -> bool {
        matches!(self.etat, EtatPartie::Blessee(_))
    }

    pub fn pourcentage_blessure(&self) -> u8 {
        match self.etat {
            EtatPartie::Blessee(pourcentage) => pourcentage,
            EtatPartie::Morte => 100,
            EtatPartie::Saine => 0,
        }
    }

    pub fn pourcentage_vie(&self) -> f32 {
        if self.vie_max == 0 {
            return 0.0;
        }
        (self.vie_actuelle as f32 / self.vie_max as f32) * 100.0
    }

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

    pub fn soigner(&mut self, soin: u32) {
        if self.est_morte() {
            println!("{} est détruite et ne peut pas être soignée normalement.", self.nom);
            return;
        }

        let ancienne_vie = self.vie_actuelle;
        self.vie_actuelle = (self.vie_actuelle + soin).min(self.vie_max);
        
        let vie_recuperee = self.vie_actuelle - ancienne_vie;
        if vie_recuperee > 0 {
            println!("{} récupère {} points de vie ({}/{})", 
                self.nom, vie_recuperee, self.vie_actuelle, self.vie_max);
        }

        self.mettre_a_jour_etat();
    }

    fn mettre_a_jour_etat(&mut self) {
        if self.vie_actuelle == 0 {
            self.etat = EtatPartie::Morte;
        } else if self.vie_actuelle == self.vie_max {
            self.etat = EtatPartie::Saine;
            self.guerison = Utc::now();
        } else {
            let pourcentage_blessure = 100 - self.pourcentage_vie() as u8;
            self.etat = EtatPartie::Blessee(pourcentage_blessure);
        }
    }

    pub fn regeneration_naturelle(&mut self) {
        if self.est_morte() || self.est_saine() {
            return;
        }

        if Utc::now() >= self.guerison {
            let temps_ecoule = Utc::now().signed_duration_since(self.guerison);
            let regeneration = (temps_ecoule.num_minutes() / 10).max(1) as u32;
            
            if regeneration > 0 {
                self.soigner(regeneration);
            }
        }
    }

    pub fn nom(&self) -> &str {
        &self.nom
    }
    
    pub fn equipement(&self) -> &crate::inventaire::Inventaire {
        &self.equipement
    }
    
    pub fn ajouter_equipement(&mut self, objet : u8){
        let _ = &self.equipement.ajouter_objet(objet);
    }

    pub fn récupérer_objet(&mut self, index: usize) -> ObjetInventaire {
        let objet = self.equipement.objets[index].clone();
        self.equipement.objets[index].nombre -= 1;
        if self.equipement.objets[index].nombre == 0 {
            self.equipement.objets.remove(index);
        }
        self.equipement.trier_quantite();

        objet
    }

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResultatBlessure {
    Mort,
    PartieDetruite,
    Blesse,
    RienGrave,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Personnage {
    pub id: u32,
    pub nom: String,
    pub description: String,
    pub force: u8,
    pub inventaire: crate::inventaire::Inventaire,
    pub parties_du_corps: Vec<PartieDuCorps>,
    pub argent: u32,
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
    pub fn gerer_blessure(&mut self, nom_partie: &str, degats: u32) -> ResultatBlessure {
        if let Some(partie) = self.parties_du_corps.iter_mut()
            .find(|p| p.nom.to_lowercase() == nom_partie.to_lowercase()) {
            
            let partie_detruite = partie.subir_degats(degats);
            
            if partie_detruite && partie.nom.to_lowercase().contains("Tête") {
                self.est_vivant = false;
                println!("{} est mort suite à une blessure mortelle à la tête !", self.nom);
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

    pub fn regeneration_naturelle(&mut self) {
        if !self.est_vivant {
            return;
        }

        for partie in &mut self.parties_du_corps {
            partie.regeneration_naturelle();
        }
    }

    pub fn soigner_partie(&mut self, nom_partie: &str, soin: u32) -> bool {
        if !self.est_vivant {
            println!("{} est mort et ne peut pas être soigné.", self.nom);
            return false;
        }

        if let Some(partie) = self.parties_du_corps.iter_mut()
            .find(|p| p.nom.to_lowercase() == nom_partie.to_lowercase()) {
            partie.soigner(soin);
            return true;
        }
        
        println!("Partie du corps '{}' non trouvée !", nom_partie);
        false
    }

    pub fn afficher_etat_sante(&self) {
        println!("\n=== État de santé de {} ===", self.nom);
        println!("Statut: {}", if self.est_vivant { "Vivant" } else { "Mort" });
        
        for partie in &self.parties_du_corps {
            let statut = match &partie.etat {
                EtatPartie::Saine => "Saine".to_string(),
                EtatPartie::Blessee(p) => format!("Blessée ({}%)", p),
                EtatPartie::Morte => "Détruite".to_string(),
            };
            
            println!("  {} - Vie: {}/{} - État: {}", 
                partie.nom, partie.vie_actuelle, partie.vie_max, statut);
        }
        println!("======================\n");
    }

    pub fn peut_se_battre(&self) -> bool {
        if !self.est_vivant {
            return false;
        }

        let bras_fonctionnels = self.parties_du_corps.iter()
            .filter(|p| p.nom.to_lowercase().contains("bras") && !p.est_morte())
            .count();
            
        let jambes_fonctionnelles = self.parties_du_corps.iter()
            .filter(|p| p.nom.to_lowercase().contains("jambe") && !p.est_morte())
            .count();

        bras_fonctionnels > 0 && jambes_fonctionnelles > 0
    }

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

    pub fn prochain_id(fichier: &str) -> io::Result<u32> {
        let personnages = match Self::lire_fichier_json(fichier)? {
            Some(personnages) => personnages,
            None => vec![],
        };
        let max_id = personnages.iter().map(|p| p.id).max().unwrap_or(0);
        Ok(max_id + 1)
    }

    pub fn ajouter_argent(&mut self, montant: u32) {
        self.argent += montant;
    }

    pub fn retirer_argent(&mut self, montant: u32) {
        if self.argent < montant {
            println!("Vous n'avez pas assez d'argent !");
        } else {
            self.argent -= montant;
        }
    }
}

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Joueur {
    pub personnage: Personnage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PNJ {
    pub personnage: Personnage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mob {
    pub personnage: Personnage,
}

impl PNJ {
    pub fn creer_pnj(nom: &str, description: &str) -> io::Result<Self> {
        let prochain_id = Personnage::prochain_id("src/json/pnj.json")?;
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

        personnage.sauvegarder_json("src/json/pnj.json")?;
        Ok(PNJ { personnage })
    }

    pub fn charger_pnj(fichier: &str) -> io::Result<Vec<Personnage>> {
        let pnjs = Personnage::charger_depuis_json(fichier)?;
        if pnjs.is_empty() {
            println!("Aucun PNJ trouvé, création de PNJs de test...");
            Self::creer_pnjs_test_direct()?;
            return Personnage::charger_depuis_json(fichier);
        }
        Ok(pnjs)
    }

    pub fn creer_pnjs_test_direct() -> io::Result<()> {
        let pnjs_test = vec![
            ("Marcus le Marchand", "Un marchand expérimenté qui vend des équipements"),
            ("Elena la Guérisseuse", "Une soigneuse capable de guérir les blessures"),
            ("Gareth le Garde", "Un garde robuste qui protège la ville"),
            ("Lydia l'Informatrice", "Une espionne qui connaît tous les secrets"),
            ("Thomas le Forgeron", "Un artisan qui fabrique des armes et armures")
        ];

        let mut personnages = vec![];
        let mut current_id = 1;

        for (nom, description) in pnjs_test {
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
        let mut file = File::create("src/json/pnj.json")?;
        file.write_all(json.as_bytes())?;
        
        println!("5 PNJs de test créés avec succès !");
        Ok(())
    }

    pub fn creer_pnjs_test() -> io::Result<()> {
        Self::creer_pnjs_test_direct()
    }
}

impl Mob {
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

    pub fn charger_mob(fichier: &str) -> io::Result<Vec<Personnage>> {
        let mobs = Personnage::charger_depuis_json(fichier)?;
        if mobs.is_empty() {
            println!("Aucun Mob trouvé, création de Mobs de test...");
            Self::creer_mobs_test_direct()?;
            return Personnage::charger_depuis_json(fichier);
        }
        Ok(mobs)
    }

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

    pub fn creer_mobs_test() -> io::Result<()> {
        Self::creer_mobs_test_direct()
    }
}

impl Joueur {
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

    pub fn charger_joueur(fichier: &str) -> io::Result<Vec<Personnage>> {
        Personnage::charger_depuis_json(fichier)
    }

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

    pub fn creer_joueur_test() -> io::Result<()> {
        Self::creer_joueur_test_direct()
    }
}
