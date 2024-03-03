use bevy::window::PrimaryWindow;
use rand::Rng;

use super::*;

const TREE_ANIMATION_SPEED: f32 = 0.1;
const TREE_SPAWN_POINTS_SCALES: [(f32, f32); 33] = [
    (-0.477083333, 0.436111111),
    (-0.4375, 0.127777778),
    (-0.416145833, 0.386111111),
    (-0.426041667, 0.293518519),
    (-0.3625, 0.342592593),
    (-0.34375, 0.175925926),
    (-0.226041667, 0.219444444),
    (-0.177083333, 0.157407407),
    (-0.163541667, 0.434259259),
    (-0.057291667, 0.14537037),
    (-0.034375, 0.467592593),
    (0.14375, 0.12962963),
    (0.175, 0.410185185),
    (0.301041667, 0.125),
    (0.363541667, 0.355555556),
    (0.408854167, 0.475925926),
    (0.434895833, 0.161111111),
    (0.463541667, 0.361111111),
    (-0.446354167, -0.140740741),
    (-0.369791667, -0.419444444),
    (-0.317708333, -0.067592593),
    (-0.177083333, -0.388888889),
    (-0.145833333, -0.062037037),
    (-0.129166667, -0.268518519),
    (-0.057291667, -0.347222222),
    (0.038541667, -0.37962963),
    (0.09375, -0.092592593),
    (0.1265625, -0.34537037),
    (0.133333333, -0.166666667),
    (0.192708333, -0.061111111),
    (0.271875, -0.231481481),
    (0.289583333, -0.092592593),
    (0.413020833, -0.12037037),
];

pub fn spawn_trees(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let window = window_query
        .get_single()
        .expect("Primary window should exist");

    for (x_scale, y_scale) in TREE_SPAWN_POINTS_SCALES {
        let tree_spawnpoint = generate_spawn_point_transform_from_x_y(x_scale, y_scale, &window);

        let texture_handle: Handle<Image> = asset_server.load("sprites/decorations/tree.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(192.0, 192.0), 4, 1, None, None);
        let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);

        let mut tree_wiggle_animation: TreeWiggleAnimation = TreeWiggleAnimation {
            length_of_animation: 4,
            animation_timer: Timer::from_seconds(TREE_ANIMATION_SPEED, TimerMode::Repeating),
        };
        tree_wiggle_animation
            .animation_timer
            .set_elapsed(std::time::Duration::from_secs_f32(
                rand::random::<f32>() * TREE_ANIMATION_SPEED,
            ));

        commands.spawn((
            SpriteSheetBundle {
                transform: tree_spawnpoint,
                sprite: TextureAtlasSprite {
                    index: rand::thread_rng().gen_range(0..4),
                    ..default()
                },
                texture_atlas: texture_atlas_handle,
                ..default()
            },
            tree_wiggle_animation,
            Name::new("Tree"),
        ));
    }
}

pub fn animate_trees(
    time: Res<Time>,
    mut tree_query: Query<(&mut TreeWiggleAnimation, &mut TextureAtlasSprite)>,
) {
    for (mut tree_animation, mut atlas_sprite) in &mut tree_query {
        tree_animation.animation_timer.tick(time.delta());
        if tree_animation.animation_timer.just_finished() {
            atlas_sprite.index = if atlas_sprite.index == tree_animation.length_of_animation - 1 {
                0
            } else {
                atlas_sprite.index + 1
            };
        }
    }
}

fn generate_spawn_point_transform_from_x_y(
    x_scale: f32,
    y_scale: f32,
    window: &Window,
) -> Transform {
    Transform::from_xyz(window.width() * x_scale, window.height() * y_scale, 0.5)
}
