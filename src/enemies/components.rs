use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {}

#[derive(Component, Default)]
pub struct CurrentlyBeingTyped {
    pub index: usize,
}
