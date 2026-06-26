pub mod animation;
pub mod bundles;
pub mod components;
pub mod spawn;
pub mod stats;

use crate::prelude::*;
use stats::{EnemyStats, EnemyStatsAssetLoader};

pub struct DragonPlugin;

impl Plugin for DragonPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<EnemyStats>()
            .init_asset_loader::<EnemyStatsAssetLoader>();
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
        app.add_systems(PostUpdate, spawn::assign_enemy_model);
        app.add_systems(
            PostUpdate,
            animation::play_dragon_idle.after(spawn::assign_enemy_model),
        );
    }
}
