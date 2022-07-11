use crate::direction::Direction;
use crate::entity::Entity;
use glm::Vec2;
use sdl2::render::WindowCanvas;
use std::fmt::Debug;
use uuid::Uuid;

pub trait GameObject: Debug {
    fn render(&mut self, canvas: &mut WindowCanvas);
    fn move_to(&mut self, direction: Direction);
    fn get_body(&self) -> Box<dyn Iterator<Item = &Vec2> + '_>;
    fn get_entity(&self) -> Entity;
    fn get_id(&self) -> Uuid;
}
