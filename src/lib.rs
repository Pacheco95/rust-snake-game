mod texture_manager;
mod utils;

pub mod engine {
    use std::collections::LinkedList;
    use std::rc::Rc;
    use std::time::{Duration, Instant};

    use glm::Vec2;
    use sdl2::event::Event;
    use sdl2::image::InitFlag;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use sdl2::render::WindowCanvas;
    use sdl2::Sdl;

    use crate::texture_manager::TextureManager;
    use crate::vec2;

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

    trait Render {
        fn render(&mut self, canvas: &mut WindowCanvas);
    }

    trait Move {
        fn move_to(&mut self, direction: Direction);
    }

    trait GameObject: Render + Move {}

    struct Snake {
        body: LinkedList<Vec2>,
    }

    impl Snake {
        fn new(origin: Vec2, direction: Direction, initial_size: i32) -> Self {
            let direction = Vec2::from(direction);
            let body: LinkedList<_> = (0..initial_size)
                .map(|i| origin + (direction * i as f32))
                .rev()
                .collect();
            Self { body }
        }
    }

    impl Render for Snake {
        fn render(&mut self, canvas: &mut WindowCanvas) {
            canvas.set_draw_color(Color::RED);
            let snake_rects: Vec<_> = self.body.iter().map(vec2rect).collect();
            canvas.fill_rects(&snake_rects).unwrap();
        }
    }

    impl Move for Snake {
        fn move_to(&mut self, direction: Direction) {
            let head = self.body.front().unwrap();
            let mut parent_cell = *head + direction;

            parent_cell.x = clamp_round(parent_cell.x, 0.0..COLUMNS as f32);
            parent_cell.y = clamp_round(parent_cell.y, 0.0..ROWS as f32);

            for cell in self.body.iter_mut() {
                std::mem::swap(cell, &mut parent_cell);
            }
        }
    }

    impl GameObject for Snake {}

    pub struct GameEngine<'a> {
        context: Sdl,
        canvas: WindowCanvas,
        game_objects: Vec<Box<dyn GameObject>>,
        fps: u32,
        game_over: bool,
        texture_manager: TextureManager<'a>,
        direction: Direction,
    }

    impl<'a> GameEngine<'a> {
        pub fn new() -> Self {
            let context = sdl2::init().unwrap();
            let canvas = create_canvas(&context);

            sdl2::image::init(InitFlag::WEBP).unwrap();

            let center = vec2!(ROWS / 2, COLUMNS / 2);
            let direction = Direction::Down;
            let snake = Snake::new(center, direction, INITIAL_SNAKE_SIZE as i32);

            let texture_creator = Rc::new(canvas.texture_creator());
            let texture_manager = TextureManager::new(texture_creator);

            let game_objects: Vec<Box<dyn GameObject>> = vec![Box::new(snake)];

            GameEngine {
                context,
                canvas,
                game_objects,
                fps: INITIAL_FPS as u32,
                game_over: false,
                texture_manager,
                direction,
            }
        }

        fn redraw(&mut self) {
            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();

            if self.game_over {
                let game_over_texture = self.texture_manager.load("res/game-over.webp").unwrap();
                self.canvas.copy(&game_over_texture, None, None).unwrap();
            } else {
                self.game_objects
                    .iter_mut()
                    .for_each(|game_obj| game_obj.render(&mut self.canvas))
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
                if start.elapsed().as_millis() >= (1000 / self.fps) as u128 {
                    self.redraw();

                    if !self.game_over {
                        self.game_objects
                            .iter_mut()
                            .for_each(|o| o.move_to(self.direction));
                    }

                    start = Instant::now();
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
    }
}
