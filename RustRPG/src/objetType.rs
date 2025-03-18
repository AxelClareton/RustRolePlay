#[derive(Debug, Clone)]
pub enum ObjetType {
    Equipement {
        prix: u32
    },
    Arme {
        frequence_degats: u32,
        proba_degats: f32,
        degats: u32,
        prix: u32
    },
    Soin {
        prix: u32
    },
}