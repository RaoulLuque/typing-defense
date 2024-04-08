use super::*;

/// Component used to track what entity the enemy is currently colliding with. Defaults to entity placeholder from bevy
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
