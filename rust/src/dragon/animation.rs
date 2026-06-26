use super::ai_config::{config_for, EnemyAiConfig};
use super::components::{DragonAi, DragonAiState, DragonLocomotion, Enemy};
use super::stats::EnemyStats;
use crate::prelude::*;
use godot::classes::{AnimationPlayer, Node};
use godot::obj::InstanceId;

fn desired_animation(state: DragonAiState, locomotion: DragonLocomotion) -> &'static str {
    match (state, locomotion) {
        (DragonAiState::Idle, DragonLocomotion::Walk) => "Idle_Ground",
        (DragonAiState::Idle, DragonLocomotion::Fly) => "Fly_Idle",
        (DragonAiState::Moving, DragonLocomotion::Walk) => "Walk_Ground_Forward",
        (DragonAiState::Moving, DragonLocomotion::Fly) => "Walk_Fly_Forward",
    }
}

pub fn update_dragon_animations(
    mut godot: GodotAccess,
    stats: Res<Assets<EnemyStats>>,
    configs: Res<Assets<EnemyAiConfig>>,
    dragons: Query<(&Enemy, &DragonAi)>,
) {
    let invalid = GodotNodeHandle::from_instance_id(InstanceId::from_i64(1));

    for (enemy, ai) in &dragons {
        if enemy.model == invalid {
            continue;
        }

        let Some(config) = config_for(enemy, &stats, &configs) else {
            continue;
        };

        let clip = desired_animation(ai.state, ai.locomotion);

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

        player
            .play_ex()
            .name(clip)
            .custom_blend(config.anim_crossfade_secs as f64)
            .done();
    }
}
