use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum Entity {
    Player,
    Enemy,
    Obstacle,
}
