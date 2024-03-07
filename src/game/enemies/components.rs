use rand::{
    distributions::{Distribution, Standard},
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
    Slime,
    Snail,
}

impl Distribution<EnemyType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemyType {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=12) {
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
            10 => EnemyType::Slime,
            11 => EnemyType::Snail,
            _ => EnemyType::Trunk,
        }
    }
}
