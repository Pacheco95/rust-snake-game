use std::collections::HashMap;
use std::ops::Deref;
use uuid::Uuid;

use super::utils::*;

pub struct Grid(pub HashMap<Uuid, GameObjectRefMut>);

impl Grid {
    pub fn new() -> Self {
        Grid(HashMap::with_capacity((ROWS * COLUMNS) as usize))
    }

    pub fn add_game_object(&mut self, obj_ref: GameObjectRefMut) {
        let obj = obj_ref.deref().borrow();
        let obj_id = obj.get_id();

        for (_, o) in self.0.iter() {
            if obj_id == o.deref().borrow().get_id() {
                let msg = format!(
                    "Attempt to add duplicated game objects in scene: {}({:?})",
                    obj_id,
                    obj.get_entity()
                );
                panic!("{}", msg)
            }
        }

        self.0.insert(obj_id, obj_ref.clone());
    }

    pub fn remove_game_object_by_id(&mut self, uuid_to_remove: &Uuid) -> Option<GameObjectRefMut> {
        match self.0.remove(uuid_to_remove) {
            None => None,
            Some(game_obj_to_remove) => {
                self.0.retain(|_, obj| {
                    let obj_id = (*obj).deref().borrow().get_id();
                    *uuid_to_remove != obj_id
                });
                Some(game_obj_to_remove)
            }
        }
    }
}
