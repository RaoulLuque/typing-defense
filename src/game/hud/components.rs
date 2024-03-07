use super::*;

// Tag component used to tag entities added on the main menu screen
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct InGameHudUiElement;

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
