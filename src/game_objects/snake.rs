use std::collections::LinkedList;
use std::fmt::Debug;

use glm::Vec2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use uuid::Uuid;

use crate::direction::Direction;
use crate::entity::Entity;
use crate::game_object::GameObject;
use crate::rect;
use crate::utils::*;

#[derive(Debug)]
pub struct Snake {
    id: Uuid,
    body: LinkedList<Vec2>,
}

impl Snake {
    pub fn new(origin: Vec2, direction: Direction, initial_size: i32) -> Self {
        let direction = Vec2::from(direction);

        Self {
            id: Uuid::new_v4(),
            body: (0..initial_size)
                .map(|i| origin + (direction * i as f32))
                .rev()
                .collect(),
        }
    }
}

impl GameObject for Snake {
    fn render(&mut self, canvas: &mut WindowCanvas) {
        let snake_rects: Vec<Rect> = self.body.iter().map(|vec2| rect!(vec2.x, vec2.y)).collect();
        let (head, tail) = snake_rects.split_first().unwrap();
        canvas.set_draw_color(Color::WHITE);
        canvas.fill_rect(*head).unwrap();
        canvas.set_draw_color(Color::GREY);
        canvas.fill_rects(tail).unwrap();
    }

    fn move_to(&mut self, direction: Direction) {
        let head = self.body.front().unwrap();
        let head = Vec2::new(head.x, head.y);
        let mut parent_coord = head + direction;

        parent_coord.x = clamp_round(parent_coord.x, 0.0..COLUMNS as f32);
        parent_coord.y = clamp_round(parent_coord.y, 0.0..ROWS as f32);

        self.body
            .iter_mut()
            .for_each(|coord| std::mem::swap(coord, &mut parent_coord));
    }

    fn get_body(&self) -> Box<dyn Iterator<Item = &Vec2> + '_> {
        Box::new(self.body.iter())
    }

    fn get_entity(&self) -> Entity {
        Entity::Player
    }

    fn get_id(&self) -> Uuid {
        self.id
    }
}
