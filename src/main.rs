mod light_cone;
mod relics;

use std::{error::Error, fmt::Display};

use light_cone::LightCone;
use relics::{Body, Feet, Hands, Head, LinkRope, PlanarSphere};

//基本常量
const MaxStars: i32 = 6;

//角色错误
#[derive(Debug)]
enum CharacterErr {
    CharacterStatsErr,
}

impl Display for CharacterErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CharacterErr::CharacterStatsErr => write!(f, "CharacterStatsErr"),
        }
    }
}

impl Error for CharacterErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

//角色状态
//与星穹铁道中相对应
#[derive(Clone, Copy)]
struct CharacterStats {
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spd: i32,
}

#[derive(Clone, Copy)]
struct CharacterLevelStats {
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
}

//角色成长的模板
struct CharacterStatsTemplate {
    basic: CharacterStats,
    templates: [CharacterLevelStats; (2 * MaxStars + 1) as usize],
}

impl CharacterStatsTemplate {
    fn stars_begin_level(stars: i32) -> i32 {
        if stars == 0 {
            0
        } else {
            10 + 10 * stars
        }
    }
    fn stars_end_level(stars: i32) -> i32 {
        if stars == 0 {
            20
        } else {
            20 + 10 * stars
        }
    }
    fn stars_begin_stats(&self, stars: i32) -> CharacterLevelStats {
        if stars == 0 {
            CharacterLevelStats {
                hp: self.basic.hp,
                atk: self.basic.atk,
                def: self.basic.def,
            }
        } else {
            self.templates[(2 * stars - 1) as usize]
        }
    }
    fn stars_end_stats(&self, stars: i32) -> CharacterLevelStats {
        self.templates[(2 * stars) as usize]
    }
    pub fn level2stats(&self, stars: i32, level: i32) -> Result<CharacterStats, CharacterErr> {
        let begin_level = Self::stars_begin_level(stars);
        let end_level = Self::stars_end_level(stars);
        if level > end_level || level < begin_level {
            return Err(CharacterErr::CharacterStatsErr);
        }
        //let beginIndex = (2 * stars) as usize;
        let begin_stats = self.stars_begin_stats(stars);
        let end_stats = self.stars_end_stats(stars);
        Ok(CharacterStats {
            hp: begin_stats.hp
                + (end_stats.hp - begin_stats.hp) * (level - begin_level)
                    / (end_level - begin_level),
            atk: begin_stats.atk
                + (end_stats.atk - begin_stats.atk) * (level - begin_level)
                    / (end_level - begin_level),
            def: begin_stats.def
                + (end_stats.def - begin_stats.def) * (level - begin_level)
                    / (end_level - begin_level),
            spd: self.basic.spd,
        })
    }
}

//角色
struct Character {
    //晋升等级
    stars: i32,
    //等级
    level: i32,
    //basic_status: CharacterStats,
    //角色成长模板
    template: CharacterStatsTemplate,
    //光锥
    light_cone: Option<Box<dyn LightCone>>,
    //遗器
    head: Option<Box<dyn Head>>,
    hands: Option<Box<dyn Hands>>,
    body: Option<Box<dyn Body>>,
    feet: Option<Box<dyn Feet>>,
    planar_sphere: Option<Box<dyn PlanarSphere>>,
    link_rope: Option<Box<dyn LinkRope>>,
}

impl Character {
    fn get_basic_status(&self) -> Result<CharacterStats, CharacterErr> {
        self.template.level2stats(self.stars, self.level)
    }
    //考虑当前所有状态后的属性
    fn get_real_stats(&self) -> Result<CharacterStats, CharacterErr> {
        let mut now = self.get_basic_status()?;
        now = match &self.light_cone {
            Some(light_cone) => light_cone.add_light_cone(&now),
            None => now,
        };
        Ok(now)
    }
}

fn main() {
    println!("Hello, world!");
}
