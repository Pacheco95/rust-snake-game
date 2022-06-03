pub mod engine {
    use std::time::Duration;

    use sdl2::{Sdl};
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use sdl2::render::WindowCanvas;
    use vecmath::Vector2;

    fn get_canvas(sdl_context: &Sdl) -> WindowCanvas {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("SDL2 Snake Game", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string()).unwrap();

        canvas.set_draw_color(Color::BLACK);

        canvas
    }

    pub struct GameEngine {
        context: Sdl,
        canvas: WindowCanvas,
    }

    impl GameEngine {
        pub fn new() -> Self {
            let context = sdl2::init().unwrap();
            let canvas = get_canvas(&context);
            GameEngine { context, canvas }
        }

        fn redraw(&mut self) {
            self.canvas.clear();
            self.canvas.present();
        }

        fn handle_event(&mut self, event: Event) {
            if let Event::KeyDown {keycode, ..} = event {
                let _direction: Vector2<i32> = match keycode {
                    Some(Keycode::Up) => [0, 1],
                    Some(Keycode::Down) => [0, -1],
                    Some(Keycode::Left) => [-1, 0],
                    Some(Keycode::Right) => [1, 0],
                    _ => [0, 0]
                };
            }
        }

        pub fn run(&mut self) {
            let mut event_pump = self.context.event_pump().unwrap();

            'game_loop: loop {
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

                self.redraw();
                std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
            }
        }
    }
}