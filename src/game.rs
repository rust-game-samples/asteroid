extern crate sdl2;
extern crate gl;
use sdl2::video::GLProfile;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::render::WindowCanvas;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use std::time::Duration;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;


use crate::actor::Actor;
use crate::asteroid::Asteroid;
use crate::texture::Texture;
use crate::sprite_component::SpriteComponent;

pub struct Game {
    canvas: WindowCanvas,
    context: sdl2::video::GLContext,
    is_running: bool,
    textures: HashMap<String, Rc<RefCell<Texture>>>,
    actors: Vec<Rc<RefCell<Actor>>>,
    pending_actors: Vec<Rc<RefCell<Actor>>>,
    sprites: Vec<Rc<RefCell<SpriteComponent>>>,
    // sprite_shader: Option<Shader>,
    // sprite_verts: Option<VertexArray>,
    asteroids: Vec<Rc<RefCell<Asteroid>>>,
    window: Option<sdl2::video::Window>,
    ticks_count: u32,
    updating_actors: bool,
    // ship: Option<Rc<RefCell<Ship>>>,
}

impl Game {
    pub fn new() -> Self {
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
            textures: HashMap::new(),
            actors: Vec::new(),
            pending_actors: Vec::new(),
            sprites: Vec::new(),
            // sprite_shader: None,
            // sprite_verts: None,
            asteroids: Vec::new(),
            window: None,
            ticks_count: 0,
            updating_actors: false,
            // ship: None,
        }
    }

    pub fn run_loop(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        while self.is_running {
            self.process_input(&mut event_pump);
            self.update_game();
            self.generate_output();
        }
    }

    pub fn get_texture(
        &mut self,
        file_name: &str,
    ) -> Option<Rc<RefCell<Texture>>> {
        if let Some(tex) = self.textures.get(file_name) {
            return Some(Rc::clone(tex));
        } else {
            let mut tex = Rc::new(RefCell::new(Texture::new()));
            let texture_creator: TextureCreator<WindowContext> = self.canvas.texture_creator();

            if tex.borrow_mut().load(&texture_creator, file_name).is_ok() {
                self.textures.insert(file_name.to_string(), Rc::clone(&tex));
                return Some(tex);
            } else {
                return None;
            }
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

    pub fn add_actor(&mut self, actor: Rc<RefCell<Actor>>) {
        if self.updating_actors {
            self.pending_actors.push(actor);
        } else {
            self.actors.push(actor);
        }
    }

    pub fn remove_actor(&mut self, actor: Rc<RefCell<Actor>>) {
        if let Some(index) = self.pending_actors.iter().position(|x| Rc::ptr_eq(x, &actor)) {
            self.pending_actors.swap_remove(index);
        }
        if let Some(index) = self.actors.iter().position(|x| Rc::ptr_eq(x, &actor)) {
            self.actors.swap_remove(index);
        }
    }

    pub fn add_asteroid(&mut self, asteroid: Rc<RefCell<Asteroid>>) {
        self.asteroids.push(asteroid);
    }

    pub fn remove_asteroid(&mut self, asteroid: Rc<RefCell<Asteroid>>) {
        if let Some(pos) = self.asteroids.iter().position(|a| Rc::ptr_eq(a, &asteroid)) {
            self.asteroids.remove(pos);
        }
    }

    pub fn add_sprite(&mut self, sprite: Rc<RefCell<SpriteComponent>>) {
        let my_draw_order = sprite.borrow().get_draw_order();
        let mut insert_pos = self.sprites.len(); // 挿入位置を末尾に設定

        for (i, existing_sprite) in self.sprites.iter().enumerate() {
            if my_draw_order < existing_sprite.borrow().get_draw_order() {
                insert_pos = i;
                break;
            }
        }

        self.sprites.insert(insert_pos, sprite);
    }

    pub fn remove_sprite(&mut self, sprite: Rc<RefCell<SpriteComponent>>) {
        if let Some(pos) = self.sprites.iter().position(|s| Rc::ptr_eq(s, &sprite)) {
            self.sprites.remove(pos);
        }
    }
}
