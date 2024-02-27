use super::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tree {}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct TreeWiggleAnimation {
    pub length_of_animation: usize,
    pub animation_timer: Timer,
}

#[derive(Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub enum StaticDecorationType {
    MushroomSmall,
    #[default]
    MushroomMedium,
    MushroomBig,
    BushSmall,
    BushMedium,
    BushBig,
    GrassSmall,
    GrassMedium,
    RockSmall,
    RockMedium,
    RockBig,
    TreeStump,
}
