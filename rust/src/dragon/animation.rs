use super::components::Enemy;
use crate::prelude::*;
use godot::classes::{AnimationPlayer, Node};
use godot::obj::InstanceId;

const IDLE_ANIMATION: &str = "Idle_Ground";

pub fn play_dragon_idle(mut godot: GodotAccess, enemies: Query<&Enemy>) {
    let invalid = GodotNodeHandle::from_instance_id(InstanceId::from_i64(1));

    for enemy in &enemies {
        if enemy.model == invalid {
            continue;
        }

        let Some(model) = godot.try_get::<Node>(enemy.model) else {
            continue;
        };

        let Some(player_node) = model
            .find_child_ex("AnimationPlayer")
            .recursive(true)
            .owned(false)
            .done()
        else {
            continue;
        };

        let Ok(mut player) = player_node.try_cast::<AnimationPlayer>() else {
            continue;
        };

        player.play_ex().name(IDLE_ANIMATION).done();
    }
}
