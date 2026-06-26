use crate::prelude::*;

use super::components::*;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct PlayerJumpCharge {
    pub input: &'static PlayerInputState,
    pub charge: &'static mut JumpCharge,
    pub damper: &'static mut MoveDamper,
}

pub fn update_jump_charge(
    physics_delta: Res<PhysicsDelta>,
    mut players: Query<PlayerJumpCharge, With<Player>>,
) {
    let delta = physics_delta.delta_seconds;

    for mut player in &mut players {
        if player.input.jump_pressed {
            player.charge.start();
        }

        if player.charge.is_charging && player.input.jump_held {
            player.charge.tick(delta);
            player.damper.apply_charge(player.charge.current);
        }

        if player.charge.is_charging && player.input.jump_released {
            player.charge.release();
            player.damper.reset();
        }

        if !player.charge.is_charging && player.charge.queued_jump.is_none() {
            player.damper.reset();
        }
    }
}
