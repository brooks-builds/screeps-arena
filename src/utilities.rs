use js_sys::{Object, Reflect};
use log::warn;
use screeps_arena::Creep;
use wasm_bindgen::JsValue;

use crate::creep_type::CreepType;

pub fn get_creep_type(creep: &Creep) -> CreepType {
    for part in creep.body() {
        return match part.part() {
            screeps_arena::Part::Move => continue,
            screeps_arena::Part::Work => unimplemented!(),
            screeps_arena::Part::Carry => unimplemented!(),
            screeps_arena::Part::Attack => CreepType::Fighter,
            screeps_arena::Part::RangedAttack => CreepType::Ranger,
            screeps_arena::Part::Tough => continue,
            screeps_arena::Part::Heal => CreepType::Healer,
            _ => {
                warn!("matching _ for body part");
                unreachable!("unknown body part")
            }
        };
    }

    warn!("could not find any body parts");
    unreachable!("could not find body parts")
}

pub fn is_creep_hurt(creep: &Creep) -> bool {
    creep.hits_max() - creep.hits() > 0
}

pub fn creep_to_object(creep: &Creep) -> Object {
    let object = Object::new();
    Reflect::set(
        &object,
        &JsValue::from_str("x"),
        &JsValue::from_str(&creep.x().to_string()),
    );
    Reflect::set(
        &object,
        &JsValue::from_str("y"),
        &JsValue::from_str(&creep.y().to_string()),
    );
    object
}

pub fn create_vector_object(x: u8, y: u8) -> Object {
    let object = Object::new();
    Reflect::set(
        &object,
        &JsValue::from_str("x"),
        &JsValue::from_str(&x.to_string()),
    )
    .unwrap();
    Reflect::set(
        &object,
        &JsValue::from_str("y"),
        &JsValue::from_str(&y.to_string()),
    )
    .unwrap();
    object
}
