use self::{attackers::run_attackers, defenders::run_defenders, state::State};

mod attack_state;
mod attackers;
mod defenders;
mod group;
mod state;

/// Defenders
///
///   - warrior
///   - ranger
///   - healer
///
///   1. Have defenders move to flag and wait until attacked
///   1. Towers attack when enemies near our flag
///   1. Healers heal
///
/// Attackers
///
///   - warrior
///   - rangers
///   - healers
///
///   1. move to staging area (60, 30)
///   1. if there is a body part
///     1. have creep with same body part get it
///     1. return to staging area after getting body part
///   1. at tick 1500 attack closest creep
///   1. get flag
///   1. healers heal
///
/// [creep, creep, creep]
/// [Attacker, Defender, Attacker]
/// [Warrior, Ranger, Healer]
pub fn run() {
    let mut state = State::new();
    state.update_attack_state();

    run_defenders(&state);
    run_attackers(&state);
}
