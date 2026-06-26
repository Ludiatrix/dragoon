use crate::prelude::*;

use super::components::*;

pub fn update_jump_charge(
    physics_delta: Res<PhysicsDelta>,
    mut players: Query<(
        &PlayerInputState,
        &mut JumpCharge,
        &mut MoveDamper,
    ), With<Player>>,
) {
    let delta = physics_delta.delta_seconds;

    for (input, mut charge, mut damper) in &mut players {
        if input.jump_pressed {
            charge.is_charging = true;
            charge.elapsed = 0.0;
            charge.current = Charge::Low;
            charge.queued_jump = None;
        }

        if charge.is_charging && input.jump_held {
            charge.elapsed += delta;

            charge.current = if charge.elapsed >= 2.0 {
                Charge::Max
            } else if charge.elapsed >= 1.0 {
                Charge::Medium
            } else {
                Charge::Low
            };

            damper.0 = match charge.current {
                Charge::Low => 0.65,
                Charge::Medium => 0.25,
                Charge::Max => 0.0,
            };
        }

        if charge.is_charging && input.jump_released {
            charge.queued_jump = Some(charge.current);
            charge.is_charging = false;
            charge.elapsed = 0.0;
            damper.0 = 1.0;
        }

        if !charge.is_charging && charge.queued_jump.is_none() {
            damper.0 = 1.0;
        }
    }
}