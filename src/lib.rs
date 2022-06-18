mod texture_manager;
mod utils;

pub mod engine {
    use std::borrow::Borrow;
    use std::cell::RefCell;

    use std::collections::{HashMap, LinkedList};
    use std::fmt::Debug;
    use std::hash::{Hash, Hasher};
    use std::ops::Deref;
    use std::rc::Rc;
    use std::time::{Duration, Instant};

    use glm::Vec2;
    use sdl2::event::Event;
    use sdl2::image::InitFlag;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use sdl2::render::WindowCanvas;
    use sdl2::Sdl;
    use uuid::Uuid;

    use crate::texture_manager::TextureManager;
    use crate::{rect, vec2};

    use super::utils::*;

    fn create_canvas(sdl_context: &Sdl) -> WindowCanvas {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("SDL2 Snake Game", WIDTH, HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap()
    }

    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    #[allow(dead_code)]
    enum Entity {
        Player,
        Enemy,
        Obstacle,
    }

    #[derive(Debug, PartialEq, Copy, Clone)]
    struct Coord(f32, f32);

    impl Hash for Coord {
        fn hash<H: Hasher>(&self, state: &mut H) {
            (self.0 as i32).hash(state);
            (self.1 as i32).hash(state);
        }
    }

    impl Eq for Coord {}

    impl From<&Coord> for Rect {
        fn from(&Coord(x, y): &Coord) -> Rect {
            rect!(x, y)
        }
    }

    impl From<&Coord> for Vec2 {
        fn from(&Coord(x, y): &Coord) -> Vec2 {
            Vec2::new(x, y)
        }
    }

    impl From<&Vec2> for Coord {
        fn from(&Vec2 { x, y }: &Vec2) -> Coord {
            Coord(x, y)
        }
    }

    trait GameObject: Debug {
        fn render(&mut self, canvas: &mut WindowCanvas);
        fn move_to(&mut self, direction: Direction);
        fn get_body(&self) -> Box<dyn Iterator<Item = &Coord> + '_>;
        fn get_entity(&self) -> Entity;
        fn get_id(&self) -> Uuid;
    }

    #[derive(Debug)]
    struct Snake {
        id: Uuid,
        body: LinkedList<Coord>,
    }

    impl Snake {
        fn new(origin: Vec2, direction: Direction, initial_size: i32) -> Self {
            let direction = Vec2::from(direction);

            Self {
                id: Uuid::new_v4(),
                body: (0..initial_size)
                    .map(|i| origin + (direction * i as f32))
                    .map(|Vec2 { x, y }| Coord(x, y))
                    .rev()
                    .collect(),
            }
        }
    }

    impl GameObject for Snake {
        fn render(&mut self, canvas: &mut WindowCanvas) {
            let snake_rects: Vec<Rect> = self.body.iter().map(Rect::from).collect();
            let (head, tail) = snake_rects.split_first().unwrap();
            canvas.set_draw_color(Color::WHITE);
            canvas.fill_rect(*head).unwrap();
            canvas.set_draw_color(Color::GREY);
            canvas.fill_rects(tail).unwrap();
        }

        fn move_to(&mut self, direction: Direction) {
            let head = self.body.front().unwrap();
            let head = Vec2::from(head);
            let mut parent_coord = head + direction;

            parent_coord.x = clamp_round(parent_coord.x, 0.0..COLUMNS as f32);
            parent_coord.y = clamp_round(parent_coord.y, 0.0..ROWS as f32);

            let mut parent_coord = Coord::from(&parent_coord);

            self.body
                .iter_mut()
                .for_each(|coord| std::mem::swap(coord, &mut parent_coord));
        }

        fn get_body(&self) -> Box<dyn Iterator<Item = &Coord> + '_> {
            Box::new(self.body.iter())
        }

        fn get_entity(&self) -> Entity {
            Entity::Player
        }

        fn get_id(&self) -> Uuid {
            self.id
        }
    }

    type GameObjectRefMut = Rc<RefCell<dyn GameObject>>;

    struct Grid(HashMap<Uuid, GameObjectRefMut>);

    impl Grid {
        fn new() -> Self {
            Grid(HashMap::with_capacity((ROWS * COLUMNS) as usize))
        }
    }

    pub struct GameEngine<'a> {
        context: Sdl,
        canvas: WindowCanvas,
        fps: u32,
        game_over: bool,
        texture_manager: TextureManager<'a>,
        direction: Direction,
        grid: Grid,
    }

    impl<'a> GameEngine<'a> {
        pub fn new() -> Self {
            let context = sdl2::init().unwrap();
            let canvas = create_canvas(&context);

            sdl2::image::init(InitFlag::WEBP).unwrap();

            let texture_creator = Rc::new(canvas.texture_creator());
            let texture_manager = TextureManager::new(texture_creator);

            let direction = Direction::Down;
            let center = vec2!(ROWS / 2, COLUMNS / 2);
            let snake = RefCell::new(Snake::new(center, direction, INITIAL_SNAKE_SIZE as i32));

            let grid = Grid::new();

            let mut engine = GameEngine {
                context,
                canvas,
                fps: INITIAL_FPS as u32,
                game_over: false,
                texture_manager,
                direction,
                grid,
            };

            engine.add_game_object(Rc::new(snake));

            engine
        }

        fn add_game_object(&mut self, obj_ref: GameObjectRefMut) {
            let obj = obj_ref.deref().borrow();
            let obj_id = obj.get_id();

            for (_, o) in self.grid.0.iter() {
                if obj_id == o.deref().borrow().get_id() {
                    let msg = format!(
                        "Attempt to add duplicated game objects in scene: {}({:?})",
                        obj_id,
                        obj.get_entity()
                    );
                    panic!("{}", msg)
                }
            }

            self.grid.0.insert(obj_id, obj_ref.clone());
        }

        fn remove_game_object_by_id(&mut self, uuid_to_remove: &Uuid) -> Option<GameObjectRefMut> {
            match self.grid.0.remove(uuid_to_remove) {
                None => None,
                Some(game_obj_to_remove) => {
                    self.grid.0.retain(|_, obj| {
                        let obj_id = (*obj).deref().borrow().get_id();
                        *uuid_to_remove != obj_id
                    });
                    Some(game_obj_to_remove)
                }
            }
        }

        fn redraw(&mut self) {
            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();

            if self.game_over {
                let game_over_texture = self.texture_manager.load("res/game-over.webp").unwrap();
                self.canvas.copy(&game_over_texture, None, None).unwrap();
            } else {
                for (_, game_obj) in self.grid.0.borrow().into_iter() {
                    game_obj.deref().borrow_mut().render(&mut self.canvas);
                }
            }

            self.canvas.present();
        }

        fn handle_event(&mut self, event: Event) {
            if let Event::KeyDown { keycode, .. } = event {
                self.handle_key_down_event(keycode)
            }

            if let Event::MouseWheel { y, .. } = event {
                let offset = y * MOUSE_WHEEL_SENSITIVITY;
                self.fps = (self.fps as i32 + offset).clamp(MIN_FPS, MAX_FPS) as u32;
            }
        }

        fn handle_key_down_event(&mut self, keycode: Option<Keycode>) {
            let new_direction = match keycode {
                Some(Keycode::Up) => vec2!(0., -1.),
                Some(Keycode::Down) => vec2!(0., 1.),
                Some(Keycode::Left) => vec2!(-1., 0.),
                Some(Keycode::Right) => vec2!(1., 0.),
                _ => self.direction.into(),
            };

            let is_orthogonal = glm::dot(self.direction.into(), new_direction) == 0.0;

            if is_orthogonal {
                self.direction = new_direction.into();
            }
        }

        pub fn run(&mut self) {
            let mut event_pump = self.context.event_pump().unwrap();
            let mut start = Instant::now();

            'game_loop: loop {
                let should_redraw = start.elapsed().as_millis() >= (1000 / self.fps) as u128;

                if should_redraw {
                    start = Instant::now();
                    self.redraw();

                    if !self.game_over {
                        self.move_player();
                    }
                }

                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => break 'game_loop,
                        _ => {
                            self.handle_event(event);
                        }
                    }
                }

                std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            }
        }

        fn move_player(&mut self) {
            let player_id = {
                let player = self
                    .grid
                    .0
                    .values()
                    .filter(|o| o.deref().deref().borrow().get_entity() == Entity::Player)
                    .next()
                    .unwrap()
                    .deref()
                    .borrow_mut();

                player.get_id().clone()
            };

            let player = self.remove_game_object_by_id(&player_id).unwrap();

            let head = player.borrow_mut().get_body().next().unwrap().clone();

            let next_head = Vec2::new(head.0, head.1) + Vec2::from(self.direction);
            let next_head = Coord(next_head.x, next_head.y);

            let will_collide = player
                .borrow_mut()
                .get_body()
                .any(|body_part| *body_part == next_head);

            if will_collide {
                self.game_over = true;
            }

            player.borrow_mut().move_to(self.direction);
            self.add_game_object(player);
        }
    }
}
