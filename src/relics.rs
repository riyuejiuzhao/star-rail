use crate::CharacterStats;

pub trait Relics {
    fn add_light_cone(&self, stats: &CharacterStats) -> CharacterStats;
}

pub trait Head : Relics {
}

pub trait Hands : Relics {
}

pub trait Body : Relics {
}
 
pub trait Feet : Relics {
}

pub trait PlanarSphere : Relics {
}

pub trait LinkRope : Relics {
}