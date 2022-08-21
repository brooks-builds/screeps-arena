use std::default;

use eyre::{bail, Result};
use js_sys::Reflect;
use screeps_arena::Creep;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize)]
pub enum Role {
    None,
    InitialCollector,
}

impl Role {
    pub fn add_to_creep(&self, creep: &Creep) -> Result<()> {
        let role_string = self.to_string();

        if let Err(_) = Reflect::set(
            creep,
            &JsValue::from_str("role"),
            &JsValue::from_str(&role_string),
        ) {
            bail!("Error setting role on creep");
        }

        Ok(())
    }
}

impl Default for Role {
    fn default() -> Self {
        Self::None
    }
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::None => "None",
            Role::InitialCollector => "InitialCollector",
        }
        .to_owned()
    }
}
