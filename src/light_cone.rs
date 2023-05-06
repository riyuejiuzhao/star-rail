use crate::CharacterStats;

pub trait LightCone {
    fn add_light_cone(&self, stats: &CharacterStats) -> CharacterStats;
}

pub struct LightConeStats {
    hp: u64,
    atk: u64,
    def: u64,
}
