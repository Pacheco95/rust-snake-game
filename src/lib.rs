pub mod engine {
    use std::collections::LinkedList;
    use std::time::Duration;

    use glm::Vec2;
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use sdl2::render::WindowCanvas;
    use sdl2::Sdl;

    const WIDTH: u32 = 800;
    const HEIGHT: u32 = WIDTH;
    const CELL_SIZE: u32 = 10;
    const ROWS: u32 = HEIGHT / CELL_SIZE;
    const COLUMNS: u32 = WIDTH / CELL_SIZE;
    const INITIAL_FPS: i32 = 60;
    const MIN_FPS: i32 = 1;
    const MAX_FPS: i32 = 60;
    const INITIAL_SNAKE_SIZE: u32 = ROWS;
    const MOUSE_WHEEL_SENSITIVITY: i32 = 5;

    fn vec2rect(vec: &Vec2) -> Rect {
        let [x, y] = [vec.x as i32, vec.y as i32].map(|i| i * CELL_SIZE as i32);
        Rect::new(x, y, CELL_SIZE, CELL_SIZE)
    }

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

    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Direction {
        fn to_vec(&self) -> Vec2 {
            match self {
                Direction::Up => Vec2::new(0., -1.),
                Direction::Down => Vec2::new(0., 1.),
                Direction::Left => Vec2::new(-1., 0.),
                Direction::Right => Vec2::new(1., 0.),
            }
        }
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
    }

    impl GameEngine {
        pub fn new() -> Self {
            let context = sdl2::init().unwrap();
            let canvas = get_canvas(&context);
            let origin = Vec2::new((ROWS / 2) as f32, (COLUMNS / 2) as f32);
            let snake = Snake::new(origin, Direction::Down, INITIAL_SNAKE_SIZE as i32);
            GameEngine {
                context,
                canvas,
                snake,
                fps: INITIAL_FPS as u32,
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

            if parent_cell.x >= COLUMNS as f32 {
                parent_cell.x = 0.;
            } else if parent_cell.x < 0. {
                parent_cell.x = (COLUMNS - 1) as f32
            }

            if parent_cell.y >= ROWS as f32 {
                parent_cell.y = 0.;
            } else if parent_cell.y < 0. {
                parent_cell.y = (ROWS - 1) as f32
            }

            for cell in self.snake.body.iter_mut() {
                [*cell, parent_cell] = [parent_cell, *cell];
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
                Some(Keycode::Up) => Vec2::new(0., -1.),
                Some(Keycode::Down) => Vec2::new(0., 1.),
                Some(Keycode::Left) => Vec2::new(-1., 0.),
                Some(Keycode::Right) => Vec2::new(1., 0.),
                _ => self.snake.direction,
            };

            let is_orthogonal = glm::dot(self.snake.direction, new_direction) == 0.0;

            if is_orthogonal {
                self.snake.direction = new_direction;
            }
        }

        pub fn run(&mut self) {
            let mut event_pump = self.context.event_pump().unwrap();

            'game_loop: loop {
                self.redraw();

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

                self.move_snake();

                std::thread::sleep(Duration::from_millis((1000 / self.fps) as u64));
            }
        }
    }
}
