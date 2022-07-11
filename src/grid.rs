use std::collections::HashMap;
use uuid::Uuid;

use super::utils::*;

pub struct Grid(pub HashMap<Uuid, GameObjectRefMut>);

impl Grid {
    pub fn new() -> Self {
        Grid(HashMap::with_capacity((ROWS * COLUMNS) as usize))
    }
}
