use eyre::{bail, Result};
use js_sys::Reflect;
use screeps_arena::StructureSpawn;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub have_initial_collectors: u8,
    pub want_initial_collectors: u8,
    pub initial_collector_positions: [(u8, u8); 3],
    pub initial_collector_ids: [Option<f64>; 3],
}

impl GameState {
    pub fn new(spawn: &StructureSpawn) -> Result<Self> {
        let have_initial_collectors = 0;
        let want_initial_collectors = 3;
        let initial_collector_positions = Self::calculate_initial_collector_position(spawn);
        let initial_collector_ids = [None, None, None];

        Ok(Self {
            have_initial_collectors,
            want_initial_collectors,
            initial_collector_positions,
            initial_collector_ids,
        })
    }

    pub fn save(&self, spawn: &StructureSpawn) -> Result<()> {
        let serialized_state = serde_json::to_string(self)?;
        if let Err(_error) = Reflect::set(
            spawn,
            &JsValue::from_str("game_state"),
            &JsValue::from_str(&serialized_state),
        ) {
            bail!("Error saving state to spawn");
        }

        Ok(())
    }

    pub fn load(spawn: &StructureSpawn) -> Result<Self> {
        match Reflect::get(spawn, &JsValue::from_str("game_state")) {
            Ok(value) => {
                let serialized_state = value
                    .as_string()
                    .ok_or(eyre::eyre!("Error converting state value to string"))?;
                let state = serde_json::from_str(&serialized_state)?;
                Ok(state)
            }
            Err(_) => {
                bail!("Error loading state");
            }
        }
    }

    pub fn have_all_initial_collectors(&self) -> bool {
        self.initial_collector_ids[0].is_some()
            && self.initial_collector_ids[1].is_some()
            && self.initial_collector_ids[2].is_some()
    }

    fn calculate_initial_collector_position(spawn: &StructureSpawn) -> [(u8, u8); 3] {
        let y = spawn.y();
        let spawn_x = spawn.x();
        if spawn_x < 20 {
            [(spawn_x - 3, y), (spawn_x - 2, y), (spawn_x - 1, y)]
        } else {
            [(spawn_x + 3, y), (spawn_x + 2, y), (spawn_x + 1, y)]
        }
    }
}
