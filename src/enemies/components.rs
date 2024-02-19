use rand::{seq::SliceRandom, Rng};

use super::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Enemy {}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Speed {
    pub speed: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct CurrentlyBeingTyped {
    pub index: usize,
}

#[derive(Reflect, Component, Default, Debug, PartialEq, Copy, Clone)]
#[reflect(Component)]
pub enum EnemySpawnPoint {
    Top,
    Bottom,
    #[default]
    Left,
    Right,
}

impl EnemySpawnPoint {
    pub fn next_spawn_point_excluding_self(self, rng: &mut impl Rng) -> EnemySpawnPoint {
        use EnemySpawnPoint::*;
        match self {
            Top => [Bottom, Left, Right],
            Bottom => [Top, Left, Right],
            Left => [Top, Bottom, Right],
            Right => [Top, Bottom, Left],
        }
        .choose(rng)
        .copied()
        .unwrap()
    }
}
