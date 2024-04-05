use super::*;

pub fn animate_explosions(
    time: Res<Time>,
    mut commands: Commands,
    mut explosion_query: Query<(Entity, &mut ExplosionAnimation, &mut TextureAtlas)>,
) {
    for (entity_id, mut explosion_animation, mut atlas_sprite) in &mut explosion_query {
        explosion_animation.animation_timer.tick(time.delta());
        if explosion_animation.animation_timer.just_finished() {
            if atlas_sprite.index == explosion_animation.length_of_animation - 1 {
                commands.entity(entity_id).despawn_recursive();
            } else {
                atlas_sprite.index += 1
            };
        }
    }
}
