use eyre::{bail, Result};
use log::warn;
use screeps_arena::{Part, StructureSpawn};

use crate::global::utilities::get_creep_id;

use super::{game_state::GameState, role::Role};

const INITIAL_COLLECTOR_BODY: [Part; 2] = [Part::Move, Part::Carry];

pub fn run_spawner(spawn: &StructureSpawn, game_state: &mut GameState) -> Result<()> {
    if game_state.have_initial_collectors < game_state.want_initial_collectors {
        spawn_creep(
            &INITIAL_COLLECTOR_BODY,
            spawn,
            Role::InitialCollector,
            game_state,
        )
    } else {
        Ok(())
    }
}

fn spawn_creep(
    body: &[Part],
    spawn: &StructureSpawn,
    role: Role,
    game_state: &mut GameState,
) -> Result<()> {
    match spawn.spawn_creep(body) {
        Ok(creep) => {
            game_state.have_initial_collectors += 1;
            role.add_to_creep(&creep)?;
            Ok(())
        }
        Err(return_code) => match return_code {
            screeps_arena::ReturnCode::NotEnough => bail!("Not enough energy"),
            _ => Ok(()),
        },
    }
}
