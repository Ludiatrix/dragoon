pub mod bundles;
pub mod components;
pub mod input;
pub mod jump_charge;
pub mod movement;
pub mod events;
pub mod visuals;

use crate::{player::events::*, prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement::debug_player_spawned);

        app.add_message::<JumpChargeStarted>()
            .add_message::<JumpChargeChanged>()
            .add_message::<JumpChargeReleased>()
            .add_message::<JumpLaunched>();

        app.add_systems(
            PhysicsUpdate,
            (
                input::read_player_input,
                jump_charge::update_jump_charge,
                movement::move_player,
                visuals::debug_jump_charge_events,
            )
                .chain(),
        );
    }
}
