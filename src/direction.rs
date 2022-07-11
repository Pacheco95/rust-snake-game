use crate::vec2;
use glm::Vec2;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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

impl std::ops::Add<Direction> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Vec2::from(rhs)
    }
}
