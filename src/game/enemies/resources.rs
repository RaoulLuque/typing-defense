use super::*;

/// Interval for checking if enemies should spawn - in seconds
const ENEMY_SPAWN_TIMER: f32 = 1.0;

/// Resource for keeping the time between enemy spawns
#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIMER, TimerMode::Repeating),
        }
    }
}

/// Resource for tracking whether enemies are being typed and if so which ones
#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct EnemiesBeingTyped {
    pub indicator: bool,
    pub vec_of_enemies: Vec<Entity>,
}

impl Default for EnemiesBeingTyped {
    fn default() -> EnemiesBeingTyped {
        EnemiesBeingTyped {
            indicator: false,
            vec_of_enemies: Vec::new(),
        }
    }
}

/// Resource for tracking the words that enemies can be
#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct WordsHandle(pub Handle<Words>);

/// Resource for keeping track of where the last enemy was spawned.
/// Default spawn point is left
#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct LastEnemySpawnPoint {
    pub spawn_point: EnemySpawnPoint,
}
