use glm::Vec2;
use num_traits::{Euclid, Num};

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

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for Vec2 {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => vec2!(0, -1),
            Direction::Down => vec2!(0, 1),
            Direction::Left => vec2!(-1, 0),
            Direction::Right => vec2!(1, 0),
        }
    }
}

impl From<Vec2> for Direction {
    fn from(v: Vec2) -> Self {
        let (x, y) = (v.x as i32, v.y as i32);

        match (x, y) {
            (0, -1) => Direction::Up,
            (0, 1) => Direction::Down,
            (-1, 0) => Direction::Left,
            (1, 0) => Direction::Right,
            (..) => {
                panic!("Invalid direction vector {:?}", v)
            }
        }
    }
}

impl std::ops::Add<Direction> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Vec2::from(rhs)
    }
}

pub fn clamp_round<T: Num + Euclid + Copy>(n: T, r: std::ops::Range<T>) -> T {
    (n + r.start).rem_euclid(&(r.end - r.start)) + r.start
}
