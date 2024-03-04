use super::*;

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct CollidingWith {
    pub entity_colliding_with: Entity,
}

impl Default for CollidingWith {
    fn default() -> CollidingWith {
        CollidingWith {
            entity_colliding_with: Entity::PLACEHOLDER,
        }
    }
}
