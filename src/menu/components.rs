use super::*;

// Tag component used to tag entities added on the main menu screen
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct OnMainMenuScreen;

// All actions that can be triggered from a button click
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub enum MenuButtonAction {
    #[default]
    Play,
    HowToPlay,
    Quit,
}
