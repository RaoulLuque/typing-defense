use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use super::*;

/// Component used to tag enemies
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Enemy {}

/// Component used to track the speed of an enemy.
/// Is set as 0.625 to 1.375 times the enemy base speed this round
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Speed {
    pub speed: f32,
}

/// Component used to track the size of a sprite
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct SpriteSize {
    pub width: f32,
    pub height: f32,
}

/// Component used to enable animations of enemies
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct WalkingAnimation {
    pub length_of_animation: usize,
    pub animation_timer: Timer,
}

/// Component used to tag entities that are currently being typed
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct CurrentlyBeingTyped {
    pub index: usize,
}

/// Component used to track the type of an enemy
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
    BlueBird,
    Radish,
    Rino,
    RockOne,
    RockTwo,
    RockThree,
    Snail,
}

impl Distribution<EnemyType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemyType {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=11) {
            0 => EnemyType::Pig,
            1 => EnemyType::Bat,
            2 => EnemyType::Bee,
            3 => EnemyType::Bunny,
            4 => EnemyType::Chicken,
            5 => EnemyType::Mushroom,
            6 => EnemyType::BlueBird,
            7 => EnemyType::Radish,
            8 => EnemyType::Rino,
            9 => match rng.gen_range(0..=2) {
                0 => EnemyType::RockOne,
                1 => EnemyType::RockTwo,
                _ => EnemyType::RockThree,
            },
            10 => EnemyType::Snail,
            _ => EnemyType::Trunk,
        }
    }
}
