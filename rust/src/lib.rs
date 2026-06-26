mod prelude;
mod player;

use crate::prelude::*;
use godot::prelude::*;

#[bevy_app]
fn build_app(app: &mut App) {
    app.add_plugins(GodotDefaultPlugins)
        .add_plugins(player::PlayerPlugin);
}