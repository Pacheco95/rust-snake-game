pub mod engine {
    use std::collections::LinkedList;
    use std::time::Duration;

    use glm::{normalize, Vec2};
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use sdl2::render::WindowCanvas;
    use sdl2::Sdl;

    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 800;
    const CELL_SIZE: u32 = 10;

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

    struct Snake {
        body: LinkedList<Vec2>,
    }

    impl Snake {
        fn new(head: Vec2, direction: Vec2, initial_size: i32) -> Self {
            let direction = normalize(direction);
            let body: LinkedList<_> = (0..initial_size)
                .map(|i| head + (direction * i as f32))
                .collect();
            Self { body }
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
            let origin = Vec2::new(0., 0.);
            let up = Vec2::new(0., 1.);
            let snake = Snake::new(origin, up, 5);
            GameEngine {
                context,
                canvas,
                snake,
                fps: 24,
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

        fn handle_event(&mut self, event: Event) {
            if let Event::KeyDown { keycode, .. } = event {
                let _direction = match keycode {
                    Some(Keycode::Up) => Vec2::new(0., 1.),
                    Some(Keycode::Down) => Vec2::new(0., -1.),
                    Some(Keycode::Left) => Vec2::new(-1., 0.),
                    Some(Keycode::Right) => Vec2::new(1., 0.),
                    _ => Vec2::new(0., 0.),
                };
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

                std::thread::sleep(Duration::from_millis((1000 / self.fps) as u64));
            }
        }
    }
}
