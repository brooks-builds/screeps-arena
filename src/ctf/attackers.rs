use log::warn;
use screeps_arena::{
    game::utils::get_objects_by_prototype, prototypes, BodyPart, Creep, Part, ReturnCode,
};

use crate::utilities::is_creep_hurt;

use super::{group::Group, state::State};

pub fn run_attackers(state: &State) {
    for (index, creep) in state.creeps.iter().enumerate() {
        if !matches!(state.groups[index], Group::Attacker) {
            continue;
        }

        match state.types[index] {
            crate::creep_type::CreepType::Ranger => handle_ranger(creep, state),
            crate::creep_type::CreepType::Fighter => handle_fighter(creep, state),
            crate::creep_type::CreepType::Healer => handle_healer(creep, state),
        }
    }
}

fn handle_ranger(creep: &Creep, state: &State) {
    match &state.attack_state {
        super::attack_state::AttackState::Gathering => {
            if let Some(body_part) = get_objects_by_prototype(prototypes::BODY_PART)
                .into_iter()
                .filter(|body_part| {
                    body_part.part_type() == Part::Move
                        || body_part.part_type() == Part::RangedAttack
                })
                .collect::<Vec<BodyPart>>()
                .first()
            {
                creep.move_to(body_part, None);
            } else {
                creep.move_to(&state.my_flag, None);
            }
        }
        super::attack_state::AttackState::AttackingCreep(_enemy) => {
            if let Some(enemy) = state.get_closest_enemy(creep) {
                if creep.ranged_attack(enemy) == ReturnCode::NotInRange {
                    creep.move_to(enemy, None);
                }
            }
        }
        super::attack_state::AttackState::Flag => todo!(),
    }
}

fn handle_fighter(creep: &Creep, state: &State) {
    match &state.attack_state {
        super::attack_state::AttackState::Gathering => {
            if let Some(body_part) = get_objects_by_prototype(prototypes::BODY_PART)
                .into_iter()
                .filter(|body_part| body_part.part_type() == Part::Attack)
                .collect::<Vec<BodyPart>>()
                .first()
            {
                creep.move_to(body_part, None);
            } else {
                creep.move_to(&state.my_flag, None);
            }
        }
        super::attack_state::AttackState::AttackingCreep(_enemy) => {
            if let Some(enemy) = state.get_closest_enemy(creep) {
                if creep.attack(enemy) == ReturnCode::NotInRange {
                    creep.move_to(enemy, None);
                }
            }
        }
        super::attack_state::AttackState::Flag => todo!(),
    }
}

fn handle_healer(creep: &Creep, state: &State) {
    for other_creep in state.creeps.iter() {
        if is_creep_hurt(other_creep) {
            if creep.heal(other_creep) == ReturnCode::NotInRange {
                creep.move_to(other_creep, None);
            }
            return;
        }
    }

    match &state.attack_state {
        super::attack_state::AttackState::Gathering => {
            if let Some(body_part) = get_objects_by_prototype(prototypes::BODY_PART)
                .into_iter()
                .filter(|body_part| body_part.part_type() == Part::Heal)
                .collect::<Vec<BodyPart>>()
                .first()
            {
                creep.move_to(body_part, None);
            } else {
                creep.move_to(&state.my_flag, None);
            }
        }
        super::attack_state::AttackState::AttackingCreep(_) => {
            if let Some(hurt_ally) = state
                .creeps
                .iter()
                .filter(|creep| is_creep_hurt(creep))
                .collect::<Vec<&Creep>>()
                .first()
            {
                if creep.heal(hurt_ally) == ReturnCode::NotInRange {
                    creep.move_to(hurt_ally, None);
                }
            }
        }
        super::attack_state::AttackState::Flag => todo!(),
    }
}
