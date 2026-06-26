pub mod ai;
pub mod ai_config;
pub mod animation;
pub mod bundles;
pub mod components;
pub mod spawn;
pub mod stats;

use crate::prelude::*;
use ai_config::{EnemyAiConfig, EnemyAiConfigAssetLoader};
use stats::{EnemyStats, EnemyStatsAssetLoader};

pub struct DragonPlugin;

impl Plugin for DragonPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<EnemyStats>()
            .init_asset_loader::<EnemyStatsAssetLoader>()
            .init_asset::<EnemyAiConfig>()
            .init_asset_loader::<EnemyAiConfigAssetLoader>();
        app.add_systems(Update, spawn::spawn_dragons);
        app.add_systems(
            Update,
            (
                spawn::assign_enemy_stats,
                spawn::assign_enemy_body,
                spawn::spawn_enemy_models,
            )
                .chain(),
        );
        app.add_systems(Update, ai::pick_dragon_targets);
        app.add_systems(PostUpdate, spawn::assign_enemy_model);
        app.add_systems(
            PostUpdate,
            animation::update_dragon_animations.after(spawn::assign_enemy_model),
        );
        app.add_systems(PhysicsUpdate, ai::move_dragons);
    }
}
