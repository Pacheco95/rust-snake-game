use glm::Vec2;
use konst::primitive::{parse_i32, parse_u32};
use konst::unwrap_ctx;

use sdl2::rect::Rect;

pub const WIDTH: u32 = unwrap_ctx!(parse_u32(env!("WIDTH")));
pub const HEIGHT: u32 = WIDTH;
pub const CELL_SIZE: u32 = unwrap_ctx!(parse_u32(env!("CELL_SIZE")));
pub const ROWS: u32 = HEIGHT / CELL_SIZE;
pub const COLUMNS: u32 = WIDTH / CELL_SIZE;
pub const INITIAL_FPS: i32 = unwrap_ctx!(parse_i32(env!("INITIAL_FPS")));
pub const MIN_FPS: i32 = unwrap_ctx!(parse_i32(env!("MIN_FPS")));
pub const MAX_FPS: i32 = unwrap_ctx!(parse_i32(env!("MAX_FPS")));
pub const INITIAL_SNAKE_SIZE: u32 = unwrap_ctx!(parse_u32(env!("INITIAL_SNAKE_SIZE")));
pub const MOUSE_WHEEL_SENSITIVITY: i32 = unwrap_ctx!(parse_i32(env!("MOUSE_WHEEL_SENSITIVITY")));

#[macro_export]
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

#[macro_export]
macro_rules! vec2(
    ($x:expr, $y:expr) => (Vec2::new($x as f32, $y as f32))
);

pub fn vec2rect(vec: &Vec2) -> Rect {
    let size = CELL_SIZE as f32;
    rect!(vec.x * size, vec.y * size, CELL_SIZE, CELL_SIZE)
}

#[allow(dead_code)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_vec(&self) -> Vec2 {
        match self {
            Direction::Up => vec2!(0, -1),
            Direction::Down => vec2!(0, 1),
            Direction::Left => vec2!(-1, 0),
            Direction::Right => vec2!(1, 0),
        }
    }
}

use num_traits::{Euclid, Num};

pub fn clamp_round<T: Num + Euclid + Copy>(n: T, r: std::ops::Range<T>) -> T {
    (n + r.start).rem_euclid(&(r.end - r.start)) + r.start
}
