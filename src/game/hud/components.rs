use super::*;

// Tag component used to tag entities in the in game hud
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct InGameHudUiElement;

// Tag component used to tag entities in the in game hud
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct InBetweenRoundsHudUiElement;

// Tag component used to tag the text that displays the current wpm counter
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct WpmText;

// Tag component used to tag the text that displays the current score
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ScoreText;

// Tag component used to tag the text that displays the current streak
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct StreakText;

// Tag component used to tag the parent entity of all the nodes in the hud banner at the top of the screen
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct InGameHudParent;

#[derive(Reflect, Component, Default)]
pub(crate) struct UiFixedZ {
    pub z: f32,
}
