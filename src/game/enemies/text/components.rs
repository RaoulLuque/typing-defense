use super::*;

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct TextCollidingWith {
    pub entity_colliding_with: Entity,
}

impl Default for TextCollidingWith {
    fn default() -> TextCollidingWith {
        TextCollidingWith {
            entity_colliding_with: Entity::PLACEHOLDER,
        }
    }
}
