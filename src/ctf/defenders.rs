use screeps_arena::{Creep, ReturnCode};

use crate::{
    creep_type::CreepType,
    utilities::{create_vector_object, creep_to_object, is_creep_hurt},
};

use super::{group::Group, state::State};

pub fn run_defenders(state: &State) {
    for (index, creep) in state.creeps.iter().enumerate() {
        if !matches!(state.groups[index], Group::Defender) {
            continue;
        }

        match state.types[index] {
            crate::creep_type::CreepType::Ranger => handle_ranger(creep, state),
            crate::creep_type::CreepType::Fighter => handle_fighter(creep, state),
            crate::creep_type::CreepType::Healer => handle_healer(creep, state),
        }
    }

    handle_towers(state);
}

fn handle_ranger(creep: &Creep, state: &State) {
    creep.move_to(&state.my_flag, None);
    if let Some(closest_enemy_object) = creep.find_closest_by_range(&state.get_enemies_array()) {
        if let Some(closest_enemy) = state.get_enemy_by_object(closest_enemy_object) {
            creep.ranged_attack(closest_enemy);
        }
    }
}

fn handle_fighter(creep: &Creep, state: &State) {
    if let Some(closest_enemy_object) = state
        .my_flag
        .find_closest_by_range(&state.get_enemies_array())
    {
        if let Some(closest_enemy) = state.get_enemy_by_object(closest_enemy_object) {
            if creep.get_range_to(&creep_to_object(closest_enemy)) < 3 {
                if creep.attack(closest_enemy) == ReturnCode::NotInRange {
                    creep.move_to(closest_enemy, None);
                }
            } else {
                creep.move_to(&state.my_flag, None);
            }
        }
    }
}

fn handle_healer(healer: &Creep, state: &State) {
    let position = if state.my_flag.x() > 50 {
        create_vector_object(state.my_flag.x(), state.my_flag.y() + 1)
    } else {
        create_vector_object(state.my_flag.x(), state.my_flag.y() - 1)
    };

    healer.move_to(&position, None);
    if let Some(ranger) = get_defending_ranger(state) {
        if is_creep_hurt(ranger) {
            healer.heal(ranger);
            return;
        }
    }

    if let Some(fighter) = get_defending_fighter(state) {
        if is_creep_hurt(fighter) {
            healer.ranged_heal(fighter);
            return;
        }
    }

    if is_creep_hurt(healer) {
        healer.heal(healer);
    }
}

fn get_defending_ranger(state: &State) -> Option<&Creep> {
    for (index, defender) in state.get_creeps_from_group(Group::Defender) {
        if matches!(state.types[index], CreepType::Ranger) {
            return Some(defender);
        }
    }
    None
}

fn get_defending_fighter(state: &State) -> Option<&Creep> {
    for (index, defender) in state.get_creeps_from_group(Group::Defender) {
        if matches!(state.types[index], CreepType::Fighter) {
            return Some(defender);
        }
    }
    None
}

fn handle_towers(state: &State) {
    if let Some(enemy) = state.get_closest_enemy_to_flag_within_radius(2) {
        state.towers.iter().for_each(|tower| {
            tower.attack(enemy);
        });
    }
}
