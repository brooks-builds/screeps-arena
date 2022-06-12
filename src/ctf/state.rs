use js_sys::{Array, JsString, Object, Reflect};
use screeps_arena::{
    game::{pathfinder::FindPathOptions, utils::get_objects_by_prototype},
    prototypes, Creep, Flag, OwnedStructureProperties, StructureTower,
};
use wasm_bindgen::JsValue;

use crate::{
    creep_type::CreepType,
    utilities::{creep_to_object, get_creep_type},
};

use super::{attack_state::AttackState, group::Group};

pub struct State {
    pub creeps: Vec<Creep>,
    pub groups: Vec<Group>,
    pub types: Vec<CreepType>,
    pub my_flag: Flag,
    pub enemies: Vec<Creep>,
    pub towers: Vec<StructureTower>,
    pub attack_state: AttackState,
}

impl State {
    pub fn new() -> Self {
        let state = Self::default()
            .load_creeps()
            .load_towers()
            .set_groups_and_types();

        state
    }

    fn load_creeps(mut self) -> Self {
        self.creeps = get_objects_by_prototype(prototypes::CREEP)
            .into_iter()
            .filter(|creep| creep.my())
            .collect();

        self.enemies = get_objects_by_prototype(prototypes::CREEP)
            .into_iter()
            .filter(|creep| !creep.my())
            .collect();

        self
    }

    fn load_towers(mut self) -> Self {
        self.towers = get_objects_by_prototype(prototypes::STRUCTURE_TOWER)
            .into_iter()
            .filter(|tower| tower.my().unwrap_or_default())
            .collect();

        self
    }

    fn set_groups_and_types(mut self) -> Self {
        let mut seen_warrior = false;
        let mut seen_ranger = false;
        let mut seen_healer = false;

        for creep in self.creeps.iter() {
            let creep_type = get_creep_type(creep);

            match creep_type {
                CreepType::Ranger => {
                    if !seen_ranger {
                        self.groups.push(Group::Defender);
                        seen_ranger = true
                    } else {
                        self.groups.push(Group::Attacker);
                    }
                }
                CreepType::Fighter => {
                    if !seen_warrior {
                        self.groups.push(Group::Defender);
                        seen_warrior = true;
                    } else {
                        self.groups.push(Group::Attacker);
                    }
                }
                CreepType::Healer => {
                    if !seen_healer {
                        self.groups.push(Group::Defender);
                        seen_healer = true;
                    } else {
                        self.groups.push(Group::Attacker);
                    }
                }
            }

            self.types.push(creep_type);
        }
        self
    }

    pub fn get_enemies_array(&self) -> Array {
        let enemies = Array::new();
        for enemy in self.enemies.iter() {
            enemies.push(enemy);
        }
        enemies
    }

    pub fn get_enemy_by_object(&self, object: Object) -> Option<&Creep> {
        let id = Reflect::get(&object, &JsValue::from_str("id"))
            .expect("Error getting id from enemy 'object'");
        for enemy in self.enemies.iter() {
            if enemy.id() == JsString::from(id.clone()) {
                return Some(enemy);
            }
        }

        None
    }

    pub fn get_creeps_from_group(&self, group: Group) -> Vec<(usize, &Creep)> {
        self.creeps
            .iter()
            .enumerate()
            .filter(|(index, _creep)| self.groups[*index] == group)
            .collect::<Vec<(usize, &Creep)>>()
    }

    pub fn get_closest_enemy_to_flag_within_radius(&self, radius: u8) -> Option<&Creep> {
        if let Some(enemy_object) = self
            .my_flag
            .find_closest_by_range(&self.get_enemies_array())
        {
            let enemy = self.get_enemy_by_object(enemy_object).unwrap();
            if self.my_flag.get_range_to(&creep_to_object(enemy)) < radius {
                Some(enemy)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_closest_enemy_to_creep_withing_radius(
        &self,
        creep: &Creep,
        radius: u8,
    ) -> Option<&Creep> {
        if let Some(enemy_object) = creep.find_closest_by_range(&self.get_enemies_array()) {
            let enemy = self.get_enemy_by_object(enemy_object).unwrap();
            if creep.get_range_to(&creep_to_object(enemy)) < radius {
                Some(enemy)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn update_attack_state(&mut self) {
        if let Some(new_attack_state) = self.attack_state.update(&self) {
            self.attack_state = new_attack_state;
        }
    }

    pub fn get_closest_enemy(&self, creep: &Creep) -> Option<&Creep> {
        if let Some(enemy_object) = creep.find_closest_by_path(&self.get_enemies_array(), None) {
            return self.get_enemy_by_object(enemy_object);
        }

        None
    }
}

impl Default for State {
    fn default() -> Self {
        let my_flags = get_objects_by_prototype(prototypes::FLAG)
            .into_iter()
            .filter(|flag| flag.my().unwrap_or_default())
            .collect::<Vec<Flag>>();

        Self {
            creeps: Default::default(),
            groups: Default::default(),
            types: Default::default(),
            my_flag: my_flags[0].clone(),
            enemies: Default::default(),
            towers: Default::default(),
            attack_state: AttackState::Gathering,
        }
    }
}
