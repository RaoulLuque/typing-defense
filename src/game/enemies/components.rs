use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};

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
pub struct SpriteSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct WalkingAnimation {
    pub length_of_animation: usize,
    pub animation_timer: Timer,
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

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub enum EnemyType {
    #[default]
    Pig,
    Bat,
    Bee,
    Bunny,
    Chicken,
    Mushroom,
    Trunk,
}

impl Distribution<EnemyType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemyType {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=6) {
            0 => EnemyType::Pig,
            1 => EnemyType::Bat,
            2 => EnemyType::Bee,
            3 => EnemyType::Bunny,
            4 => EnemyType::Chicken,
            5 => EnemyType::Mushroom,
            _ => EnemyType::Trunk,
        }
    }
}
