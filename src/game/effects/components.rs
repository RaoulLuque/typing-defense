use super::*;

// Length of explosion animation - higher is slower
pub const EXPLOSION_ANIMATION_SPEED: f32 = 0.1;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Explosion {}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ExplosionAnimation {
    pub length_of_animation: usize,
    pub animation_timer: Timer,
}
