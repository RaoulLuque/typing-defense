use super::*;

// Component used to tag entities added on the main menu screen
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct MainMenuScreenUiElement;

// Component used to tag the github icon button
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct GitHubButton;

// Component used to tag entities added on the how to play screen
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct HowToPlayScreenUiElement;

// All actions that can be triggered from a button click
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub enum MenuButtonAction {
    #[default]
    Play,
    HowToPlay,
    Resume,
    Main,
    OpenLink(String),
}

// Component used to tag text entities on the main menu screen
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct MainMenuText;

// The different settings buttons for opening or closing the settings
#[derive(Reflect, Component, Default, PartialEq, Eq, Debug)]
#[reflect(Component)]
pub enum SettingsButton {
    #[default]
    OpenSettings,
    CloseSettings,
    Plus,
    Minus,
}

// Component used to tag the closed settings ui elements
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct DifficultySettingsText;

// Component used to tag the closed settings ui elements
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct SettingsMenuClosed;

// Component used to tag the opened settings ui elements
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct SettingsMenuOpened;
