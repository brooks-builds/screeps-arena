use js_sys::{Array, Object, Reflect};
use log::*;
use screeps_arena::{
    constants::{prototypes, ReturnCode},
    game::{self, utils::get_objects_by_prototype},
    prelude::*,
    BodyPart, Creep, Flag, StructureTower,
};
use wasm_bindgen::prelude::*;

mod creep_type;
mod ctf;
mod logging;
mod utilities;

fn setup() {
    logging::setup_logging(logging::Info);
}

fn get_flags() -> (screeps_arena::Flag, screeps_arena::Flag) {
    let flags = get_objects_by_prototype(prototypes::FLAG);
    if flags.first().unwrap().my().unwrap() {
        (flags[0].clone(), flags[1].clone())
    } else {
        (flags[1].clone(), flags[0].clone())
    }
}

fn get_creep_by_id(creeps: &Vec<Creep>, id: JsValue) -> Option<&Creep> {
    for creep in creeps {
        if creep.id().loose_eq(&id) {
            return Some(creep);
        }
    }

    None
}

#[wasm_bindgen(js_name = loop)]
pub fn tick() {
    let tick = game::utils::get_ticks();

    if tick == 1 {
        setup()
    }

    #[cfg(feature = "arena-capture-the-flag")]
    {
        ctf::run();

        // let my_creeps = MyCreeps::new();

        // let enemy_creeps = get_objects_by_prototype(prototypes::CREEP)
        //     .into_iter()
        //     .filter(|creep| !creep.my())
        //     .collect::<Vec<Creep>>();
        // let flags = get_flags();
        // my_creeps.handle_defenders(&flags.0, &enemy_creeps);
        // let enemy_in_range = my_creeps.enemy_within_range(&enemy_creeps, 5);
        // if let Some(enemy) = enemy_in_range {
        //     my_creeps.attack_enemy(&enemy);
        // } else {
        //     my_creeps.collect_closest_body_part();
        // }
        // if tick >= 1500 {
        //     let enemy_array = Array::new();
        //     for enemy in &enemy_creeps {
        //         enemy_array.push(enemy);
        //     }
        //     let closest_enemy_to_my_flag = flags.0.find_closest_by_path(&enemy_array, None);
        //     if let Some(enemy_creep) = closest_enemy_to_my_flag {
        //         let enemy_id = Reflect::get(&enemy_creep, &"id".into()).unwrap();
        //         let enemy_creep = get_creep_by_id(&enemy_creeps, enemy_id).unwrap();
        //         for (index, my_creep) in my_creeps.creeps.iter().enumerate() {
        //             let result = match my_creeps.roles[index] {
        //                 Role::Defender => my_creep.attack(enemy_creep),
        //                 Role::Ranger => my_creep.ranged_attack(enemy_creep),
        //                 Role::Healer => my_creep.move_to(&flags.1, None),
        //             };

        //             if result == ReturnCode::NotInRange {
        //                 my_creep.move_to(&enemy_creep, None);
        //             }
        //         }
        //     }
        // }
    }
}

enum Role {
    Defender,
    Ranger,
    Healer,
}

impl Role {
    pub fn new(creep: &Creep) -> Self {
        for body_part in creep.body() {
            match body_part.part() {
                screeps_arena::Part::Move => continue,
                screeps_arena::Part::Work => continue,
                screeps_arena::Part::Carry => continue,
                screeps_arena::Part::Attack => return Self::Defender,
                screeps_arena::Part::RangedAttack => return Self::Ranger,
                screeps_arena::Part::Tough => continue,
                screeps_arena::Part::Heal => return Self::Healer,
                _ => continue,
            }
        }
        warn!("no idea what kind of Role this is");
        panic!();
    }
}

struct MyCreeps {
    pub creeps: Vec<Creep>,
    pub roles: Vec<Role>,
}

impl MyCreeps {
    pub fn new() -> Self {
        let creeps = get_objects_by_prototype(prototypes::CREEP)
            .into_iter()
            .filter(|creep| creep.my())
            .collect::<Vec<Creep>>();

        let roles = creeps
            .iter()
            .map(|creep| Role::new(creep))
            .collect::<Vec<Role>>();

        Self { creeps, roles }
    }

    pub fn all_move_to(&self, body_part: &BodyPart) {
        for creep in &self.creeps {
            creep.move_to(&body_part, None);
        }
    }

    /// We want the defenders to hang out at the base, and wait until we're attacked. If they see any enemies within 4 of our flag, attack them
    pub fn handle_defenders(&self, our_flag: &Flag, enemies: &Vec<Creep>) {
        let mut closest_enemy_in_range = None;
        let mut closest = 255;

        for enemy in enemies {
            let object = Object::new();
            Reflect::set(
                &object,
                &JsValue::from_str("x"),
                &JsValue::from_str(&enemy.x().to_string()),
            )
            .unwrap();
            Reflect::set(
                &object,
                &JsValue::from_str("y"),
                &JsValue::from_str(&enemy.y().to_string()),
            )
            .unwrap();
            let enemy_range = our_flag.get_range_to(&object);
            if enemy_range < closest {
                closest = enemy_range;
                closest_enemy_in_range = Some(enemy);
            }
        }

        let turrets = get_objects_by_prototype(prototypes::STRUCTURE_TOWER)
            .into_iter()
            .filter(|tower| tower.my().unwrap())
            .collect::<Vec<StructureTower>>();

        if closest < 5 {
            for (index, my_creep) in self.creeps.iter().enumerate() {
                match self.roles[index] {
                    Role::Defender => {
                        my_creep.attack(closest_enemy_in_range.unwrap());
                        for turret in &turrets {
                            turret.attack(closest_enemy_in_range.unwrap());
                        }
                    }
                    _ => (),
                }
            }
        } else {
            for (index, my_creep) in self.creeps.iter().enumerate() {
                match self.roles[index] {
                    Role::Defender => {
                        my_creep.move_to(&our_flag, None);
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn collect_closest_body_part(&self) {
        let body_parts = get_objects_by_prototype(prototypes::BODY_PART);
        let body_part_array = Array::new();
        for body_part in &body_parts {
            body_part_array.push(body_part);
        }
        for (index, creep) in self.creeps.iter().enumerate() {
            let closest_body_part = creep.find_closest_by_path(&body_part_array, None);
            if closest_body_part.is_none() {
                return;
            }

            match self.roles[index] {
                Role::Ranger => {
                    creep.move_to(&closest_body_part.unwrap(), None);
                }
                Role::Healer => {
                    creep.move_to(&closest_body_part.unwrap(), None);
                }
                _ => {}
            }
        }
    }

    pub fn enemy_within_range(&self, enemies: &Vec<Creep>, threat_range: u8) -> Option<Creep> {
        let mut enemy_in_range = None;
        let mut closest_range = 255;

        for enemy in enemies {
            let object = Object::new();
            Reflect::set(
                &object,
                &JsValue::from_str("x"),
                &JsValue::from_str(&enemy.x().to_string()),
            )
            .unwrap();
            Reflect::set(
                &object,
                &JsValue::from_str("y"),
                &JsValue::from_str(&enemy.y().to_string()),
            )
            .unwrap();
            for creep in &self.creeps {
                let enemy_range = creep.get_range_to(&object);
                if enemy_range < closest_range {
                    enemy_in_range = Some(enemy.clone());
                    closest_range = enemy_range;
                }
            }
        }

        if closest_range < threat_range {
            enemy_in_range
        } else {
            None
        }
    }

    pub fn attack_enemy(&self, enemy: &Creep) {
        for (index, creep) in self.creeps.iter().enumerate() {
            match self.roles[index] {
                Role::Defender => {}
                Role::Ranger => {
                    if creep.ranged_attack(enemy) == ReturnCode::NotInRange {
                        creep.move_to(enemy, None);
                    }
                }
                Role::Healer => {
                    if let Some(creep_in_need_of_healing) = self.get_creep_with_lowest_health() {
                        if creep.heal(creep_in_need_of_healing) == ReturnCode::NotInRange {
                            creep.move_to(&creep_in_need_of_healing, None);
                        }
                    }
                }
            }
        }
    }

    fn get_creep_with_lowest_health(&self) -> Option<&Creep> {
        let mut creep_with_lowest_health = None;
        let mut most_hp_lost = 0;

        for creep in &self.creeps {
            let hp_lost = creep.hits_max() - creep.hits();
            if hp_lost > most_hp_lost {
                most_hp_lost = hp_lost;
                creep_with_lowest_health = Some(creep);
            }
        }

        creep_with_lowest_health
    }
}
