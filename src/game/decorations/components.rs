use super::*;

/// Component used to tag trees
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tree {}

/// Component used to enable the tree wiggle animation
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct TreeWiggleAnimation {
    pub length_of_animation: usize,
    pub animation_timer: Timer,
}