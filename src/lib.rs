mod utils;

pub mod engine {
    use std::collections::LinkedList;
    use std::time::{Duration, Instant};

    use glm::Vec2;
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use sdl2::render::WindowCanvas;
    use sdl2::Sdl;

    use crate::vec2;

    use super::utils::*;

    fn get_canvas(sdl_context: &Sdl) -> WindowCanvas {
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

    struct Snake {
        body: LinkedList<Vec2>,
        direction: Vec2,
    }

    impl Snake {
        fn new(origin: Vec2, direction: Direction, initial_size: i32) -> Self {
            let direction = direction.to_vec();
            let body: LinkedList<_> = (0..initial_size)
                .map(|i| origin + (direction * i as f32))
                .rev()
                .collect();
            Self { body, direction }
        }
    }

    pub struct GameEngine {
        context: Sdl,
        canvas: WindowCanvas,
        snake: Snake,
        fps: u32,
        game_over: bool,
    }

    impl GameEngine {
        pub fn new() -> Self {
            let context = sdl2::init().unwrap();
            let canvas = get_canvas(&context);
            let origin = vec2!(ROWS / 2, COLUMNS / 2);
            let snake = Snake::new(origin, Direction::Down, INITIAL_SNAKE_SIZE as i32);

            GameEngine {
                context,
                canvas,
                snake,
                fps: INITIAL_FPS as u32,
                game_over: false,
            }
        }

        fn redraw(&mut self) {
            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();
            self.render_snake();
            self.canvas.present();
        }

        fn render_snake(&mut self) {
            self.canvas.set_draw_color(Color::RED);
            let snake_rects: Vec<_> = self.snake.body.iter().map(vec2rect).collect();
            self.canvas.fill_rects(&snake_rects).unwrap();
        }

        fn move_snake(&mut self) {
            let head = self.snake.body.front().unwrap();
            let mut parent_cell = *head + self.snake.direction;

            let collided = self.snake.body.contains(&parent_cell);

            if collided {
                self.game_over = true;
                return;
            }

            parent_cell.x = clamp_round(parent_cell.x, 0.0..COLUMNS as f32);
            parent_cell.y = clamp_round(parent_cell.y, 0.0..ROWS as f32);

            for cell in self.snake.body.iter_mut() {
                std::mem::swap(cell, &mut parent_cell);
            }
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
                _ => self.snake.direction,
            };

            let is_orthogonal = glm::dot(self.snake.direction, new_direction) == 0.0;

            if is_orthogonal {
                self.snake.direction = new_direction;
            }
        }

        pub fn run(&mut self) {
            let mut event_pump = self.context.event_pump().unwrap();

            let mut start = Instant::now();

            'game_loop: loop {
                if start.elapsed().as_millis() >= (1000 / self.fps) as u128 {
                    self.redraw();

                    if !self.game_over {
                        self.move_snake();
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
