use super::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Enemy {}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct CurrentlyBeingTyped {
    pub index: usize,
}
