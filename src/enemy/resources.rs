use bevy::prelude::*;

pub const ENEMY_SPAWN_DURATION: f32 = 1.0;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_DURATION, TimerMode::Repeating),
        }
    }
}