use super::*;

/// Component used to tag the castle
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Castle {}

/// Component used to tag the destroyed castle
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct DestroyedCastle {}
