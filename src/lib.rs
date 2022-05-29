use js_sys::{Array, Object, Reflect};
use log::*;
use screeps_arena::{
    constants::{prototypes, ReturnCode},
    game::{
        self,
        utils::{get_objects, get_objects_by_prototype},
    },
    prelude::*,
    Creep, StructureTower,
};
use wasm_bindgen::prelude::*;

const TOP_LEFT_FLAG_POSITION: u8 = 3;
const BOTTOM_RIGHT_FLAG_POSITION: u8 = 96;

mod logging;

fn setup() {
    logging::setup_logging(logging::Info);
}

#[cfg(feature = "arena-capture-the-flag")]
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

// add wasm_bindgen to any function you would like to expose for call from js
// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn tick() {
    let tick = game::utils::get_ticks();

    if tick == 1 {
        setup()
    }

    #[cfg(feature = "arena-capture-the-flag")]
    {
        let my_creeps = MyCreeps::new();

        let enemy_creeps = get_objects_by_prototype(prototypes::CREEP)
            .into_iter()
            .filter(|creep| !creep.my())
            .collect::<Vec<Creep>>();
        let flags = get_flags();
        let enemy_array = Array::new();
        for enemy in &enemy_creeps {
            enemy_array.push(enemy);
        }
        let closest_enemy_to_my_flag = flags.0.find_closest_by_path(&enemy_array, None);
        if let Some(enemy_creep) = closest_enemy_to_my_flag {
            let enemy_id = Reflect::get(&enemy_creep, &"id".into()).unwrap();
            let enemy_creep = get_creep_by_id(&enemy_creeps, enemy_id).unwrap();
            for (index, my_creep) in my_creeps.creeps.iter().enumerate() {
                let result = match my_creeps.roles[index] {
                    Role::Fighter => my_creep.attack(enemy_creep),
                    Role::Ranger => my_creep.ranged_attack(enemy_creep),
                    Role::Healer => my_creep.move_to(&flags.1, None),
                };

                if result == ReturnCode::NotInRange {
                    my_creep.move_to(&enemy_creep, None);
                }
            }
        }
    }
}

enum Role {
    Fighter,
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
                screeps_arena::Part::Attack => return Self::Fighter,
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
}
