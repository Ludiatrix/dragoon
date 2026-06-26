use crate::{player::events::*, prelude::*};

use super::components::*;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct PlayerJumpCharge {
    pub entity: Entity,
    pub input: &'static PlayerInputState,
    pub charge: &'static mut JumpCharge,
    pub damper: &'static mut MoveDamper,
}

pub fn update_jump_charge(
    physics_delta: Res<PhysicsDelta>,
    mut started: MessageWriter<JumpChargeStarted>,
    mut changed: MessageWriter<JumpChargeChanged>,
    mut released: MessageWriter<JumpChargeReleased>,
    mut players: Query<PlayerJumpCharge, With<Player>>,
) {
    let delta = physics_delta.delta_seconds;

    for mut player in &mut players {
        if player.input.jump_pressed {
            player.charge.start();
            started.write(JumpChargeStarted {
                entity: player.entity,
            });
        }

        if player.charge.is_charging && player.input.jump_held {
            let previous = player.charge.current;

            player.charge.tick(delta);
            player.damper.apply_charge(player.charge.current);

            if player.charge.current != previous {
                changed.write(JumpChargeChanged {
                    entity: player.entity,
                    charge: player.charge.current,
                });
            }
        }

        if player.charge.is_charging && player.input.jump_released {
            let charge = player.charge.current;

            player.charge.release();
            player.damper.reset();

            released.write(JumpChargeReleased {
                entity: player.entity,
                charge,
            });
        }

        if !player.charge.is_charging && player.charge.queued_jump.is_none() {
            player.damper.reset();
        }
    }
}
