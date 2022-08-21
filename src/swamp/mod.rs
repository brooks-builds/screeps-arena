// next will be to run the attackers. Each attacker is really made of 2 units, one is a giant creep with nothing but ranged attack. The other is a fast creep with nothing but move. The mover tows the shooter around
// attack all enemy creeps, then the enemy spawn
#![allow(dead_code)]

mod game_state;
mod role;
mod run_initial_collector;
mod spawner;

use eyre::Result;
use screeps_arena::{game::utils::get_objects_by_prototype, prototypes, StructureContainer};

use crate::global::utilities::{get_creeps, get_spawn};

use self::game_state::GameState;

pub fn run(ticks: u32) -> Result<()> {
    let my_spawn = get_spawn(true).ok_or(eyre::eyre!("Error getting my spawn"))?;
    let my_creeps = get_creeps(true);
    let containers = get_objects_by_prototype(prototypes::STRUCTURE_CONTAINER)
        .into_iter()
        .filter(|container| container.store().get_used_capacity(None) > 0)
        .collect::<Vec<StructureContainer>>();

    if ticks == 1 {
        let empty_gamestate = GameState::new(&my_spawn)?;
        empty_gamestate.save(&my_spawn)?;
    }

    let mut game_state = GameState::load(&my_spawn)?;
    spawner::run_spawner(&my_spawn, &mut game_state)?;

    for my_creep in &my_creeps {
        run_initial_collector::run_initial_collector(
            my_creep,
            &my_spawn,
            &mut game_state,
            &my_creeps,
            &containers[0],
        )?;
    }

    game_state.save(&my_spawn)
}
