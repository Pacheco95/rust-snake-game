use std::cell::RefCell;
use std::rc::Rc;

use num_traits::{Euclid, Num};

use crate::game_object::GameObject;

pub const CELL_SIZE: u32 = 10;
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 800;
pub const ROWS: u32 = HEIGHT / CELL_SIZE;
pub const COLUMNS: u32 = WIDTH / CELL_SIZE;
pub const INITIAL_FPS: i32 = 40;
pub const INITIAL_SNAKE_SIZE: u32 = 20;
pub const MAX_FPS: i32 = 60;
pub const MIN_FPS: i32 = 5;
pub const MOUSE_WHEEL_SENSITIVITY: i32 = 5;

pub type GameObjectRefMut = Rc<RefCell<dyn GameObject>>;

#[macro_export]
macro_rules! rect(
    ($x:expr, $y:expr) => (
        Rect::new($x as i32 * CELL_SIZE as i32, $y as i32 * CELL_SIZE as i32, CELL_SIZE, CELL_SIZE)
    )
);

#[macro_export]
macro_rules! vec2(
    ($x:expr, $y:expr) => (Vec2::new($x as f32, $y as f32))
);

pub fn clamp_round<T: Num + Euclid + Copy>(n: T, r: std::ops::Range<T>) -> T {
    (n + r.start).rem_euclid(&(r.end - r.start)) + r.start
}
