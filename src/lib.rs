use screeps_arena::game;
use wasm_bindgen::prelude::*;

mod creep_type;
mod ctf;
mod logging;
mod utilities;

fn setup() {
    logging::setup_logging(logging::Info);
}

#[wasm_bindgen(js_name = loop)]
pub fn tick() {
    let tick = game::utils::get_ticks();

    if tick == 1 {
        setup()
    }

    #[cfg(feature = "arena-capture-the-flag")]
    {
        ctf::run();

        // let my_creeps = MyCreeps::new();

        // let enemy_creeps = get_objects_by_prototype(prototypes::CREEP)
        //     .into_iter()
        //     .filter(|creep| !creep.my())
        //     .collect::<Vec<Creep>>();
        // let flags = get_flags();
        // my_creeps.handle_defenders(&flags.0, &enemy_creeps);
        // let enemy_in_range = my_creeps.enemy_within_range(&enemy_creeps, 5);
        // if let Some(enemy) = enemy_in_range {
        //     my_creeps.attack_enemy(&enemy);
        // } else {
        //     my_creeps.collect_closest_body_part();
        // }
        // if tick >= 1500 {
        //     let enemy_array = Array::new();
        //     for enemy in &enemy_creeps {
        //         enemy_array.push(enemy);
        //     }
        //     let closest_enemy_to_my_flag = flags.0.find_closest_by_path(&enemy_array, None);
        //     if let Some(enemy_creep) = closest_enemy_to_my_flag {
        //         let enemy_id = Reflect::get(&enemy_creep, &"id".into()).unwrap();
        //         let enemy_creep = get_creep_by_id(&enemy_creeps, enemy_id).unwrap();
        //         for (index, my_creep) in my_creeps.creeps.iter().enumerate() {
        //             let result = match my_creeps.roles[index] {
        //                 Role::Defender => my_creep.attack(enemy_creep),
        //                 Role::Ranger => my_creep.ranged_attack(enemy_creep),
        //                 Role::Healer => my_creep.move_to(&flags.1, None),
        //             };

        //             if result == ReturnCode::NotInRange {
        //                 my_creep.move_to(&enemy_creep, None);
        //             }
        //         }
        //     }
        // }
    }
}
