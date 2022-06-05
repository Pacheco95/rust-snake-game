use glm::Vec2;

use sdl2::rect::Rect;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = WIDTH;
pub const CELL_SIZE: u32 = 10;
pub const ROWS: u32 = HEIGHT / CELL_SIZE;
pub const COLUMNS: u32 = WIDTH / CELL_SIZE;
pub const INITIAL_FPS: i32 = 60;
pub const MIN_FPS: i32 = 1;
pub const MAX_FPS: i32 = 60;
pub const INITIAL_SNAKE_SIZE: u32 = ROWS;
pub const MOUSE_WHEEL_SENSITIVITY: i32 = 5;

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
