use super::*;

// Length of explosion animation - higher is slower
pub const EXPLOSION_ANIMATION_SPEED: f32 = 0.1;

/// Component used to tag explosion Entities
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Explosion {}

/// Component that enables the animation of the explosion
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ExplosionAnimation {
    pub length_of_animation: usize,
    pub animation_timer: Timer,
}
