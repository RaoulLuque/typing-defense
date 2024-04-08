use super::*;

use rand::{seq::SliceRandom, Rng};

/// Component used to track where an enemy entity was spawned
#[derive(Reflect, Component, Debug, Default, PartialEq, Copy, Clone)]
#[reflect(Component)]
pub enum EnemySpawnPoint {
    #[default]
    TopLeft,
    TopRight,
    Left,
    Right,
    BottomLeft,
    BottomRight,
}

/// Component used to tag the number of turns an enemy entity made so far (for pathfinding)
#[derive(Reflect, Component, Default, Debug, PartialEq, Copy, Clone)]
#[reflect(Component)]
pub struct PathCheckpointNumber {
    pub number: usize,
}

impl EnemySpawnPoint {
    pub fn next_spawn_point_excluding_self(self, rng: &mut impl Rng) -> EnemySpawnPoint {
        use EnemySpawnPoint::*;
        match self {
            TopLeft => [BottomLeft, BottomRight, TopRight, Right, Left],
            TopRight => [BottomLeft, BottomRight, TopLeft, Right, Left],
            Left => [BottomLeft, BottomRight, TopRight, TopLeft, Right],
            Right => [BottomLeft, BottomRight, TopRight, TopLeft, Left],
            BottomLeft => [Right, BottomRight, TopRight, TopLeft, Left],
            BottomRight => [BottomLeft, Right, TopRight, TopLeft, Left],
        }
        .choose(rng)
        .copied()
        .unwrap()
    }
}
