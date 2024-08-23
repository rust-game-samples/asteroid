extern crate sdl2;
extern crate gl;
use sdl2::video::GLProfile;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::render::WindowCanvas;
use std::time::Duration;

struct Game {
    canvas: WindowCanvas,
    context: sdl2::video::GLContext,
    is_running: bool,
}

impl Game {
    fn new() -> Self {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        // OpenGL attributes setup
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        // Window creation
        let window = video_subsystem
            .window("Asteroid", 1024, 768)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        let canvas = window.into_canvas().build().unwrap();

        Game {
            canvas,
            context,
            is_running: true,
        }
    }

    fn run_loop(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        while self.is_running {
            self.process_input(&mut event_pump);
            self.update_game();
            self.generate_output();
        }
    }

    fn process_input(&mut self, event_pump: &mut sdl2::EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.is_running = false,
                _ => {}
            }
        }

        let keyboard_state = event_pump.keyboard_state();
        if keyboard_state.is_scancode_pressed(Scancode::Escape) {
            self.is_running = false;
        }
    }

    fn update_game(&mut self) {
        // Game update logic goes here
    }

    fn generate_output(&mut self) {
        unsafe {
            gl::ClearColor(0.86, 0.86, 0.86, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        self.canvas.present();
        std::thread::sleep(Duration::from_millis(16)); // Cap the frame rate
    }
}

fn main() {
    let mut game = Game::new();
    game.run_loop();
}

