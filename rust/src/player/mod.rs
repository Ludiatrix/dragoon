pub mod components;
pub mod bundles;
pub mod input;
pub mod movement;
use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement::debug_player_spawned);

        app.add_systems(
            PhysicsUpdate,
            (
                input::read_player_input,
                movement::move_player,
            )
                .chain(),
        );
    }
}