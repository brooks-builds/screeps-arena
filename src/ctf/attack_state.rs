use log::warn;
use screeps_arena::Creep;

use super::state::State;

pub enum AttackState {
    Gathering,
    AttackingCreep(Creep),
    Flag,
}

impl AttackState {
    pub fn update(&self, state: &State) -> Option<Self> {
        match self {
            AttackState::Gathering => {
                if let Some(enemy) = state.get_closest_enemy_to_flag_within_radius(75) {
                    return Some(AttackState::AttackingCreep(enemy.clone()));
                }
            }
            AttackState::AttackingCreep(_) => todo!(),
            AttackState::Flag => todo!(),
        }

        None
    }
}
