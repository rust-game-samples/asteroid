use std::cell::RefCell;
use std::rc::Rc;

use crate::component::Component;
use crate::game::Game;
use crate::math::{Matrix4, Vector2, Vector3};

#[derive(Clone)]
pub enum State {
    Active,
    Paused,
    Dead,
}

#[derive(Clone)]
pub struct Actor {
    state: State,
    position: Vector2,
    scale: f32,
    rotation: f32,
    game: Rc<RefCell<Game>>,
    recompute_world_transform: bool,
    components: Vec<Rc<RefCell<dyn Component>>>,
    world_transform: Matrix4,
}

impl Actor {
    pub fn new(game: Rc<RefCell<Game>>) -> Self {
        let actor = Actor {
            state: State::Active,
            position: Vector2::zero(),
            scale: 1.0,
            rotation: 0.0,
            game,
            recompute_world_transform: true,
            components: Vec::new(),
            world_transform: Matrix4::identity(),
        };
        actor.game.borrow_mut().add_actor(Rc::new(RefCell::new(actor.clone())));
        actor
    }

    pub fn update(&mut self, delta_time: f32) {
        if let State::Active = self.state {
            self.compute_world_transform();
            self.update_components(delta_time);
            self.update_actor(delta_time);
            self.compute_world_transform();
        }
    }

    fn update_components(&mut self, delta_time: f32) {
        for comp in &self.components {
            comp.borrow_mut().update(delta_time);
        }
    }

    fn update_actor(&mut self, _delta_time: f32) {
        // Actor-specific logic to be implemented by subclasses
    }

    pub fn process_input(&mut self, key_state: &[u8]) {
        if let State::Active = self.state {
            for comp in &self.components {
                comp.borrow_mut().process_input(key_state);
            }
            self.actor_input(key_state);
        }
    }

    fn actor_input(&mut self, _key_state: &[u8]) {
        // Actor-specific input logic to be implemented by subclasses
    }

    fn compute_world_transform(&mut self) {
        if self.recompute_world_transform {
            self.recompute_world_transform = false;
            self.world_transform = Matrix4::create_scale(self.scale, self.scale, self.scale)
                * Matrix4::create_rotation_z(self.rotation)
                * Matrix4::create_translation(Vector3::new(self.position.x, self.position.y, 0.0));

            for comp in &self.components {
                comp.borrow_mut().on_update_world_transform();
            }
        }
    }

    pub fn add_component(&mut self, component: Rc<RefCell<dyn Component>>) {
        let my_order = component.borrow().get_update_order();
        let index = self
            .components
            .iter()
            .position(|comp| comp.borrow().get_update_order() > my_order)
            .unwrap_or(self.components.len());
        self.components.insert(index, component);
    }

    pub fn remove_component(&mut self, component: Rc<RefCell<dyn Component>>) {
        if let Some(pos) = self.components.iter().position(|c| Rc::ptr_eq(c, &component)) {
            self.components.remove(pos);
        }
    }

    pub fn set_position(&mut self, pos: Vector2) {
        self.position = pos;
        self.recompute_world_transform = true;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.recompute_world_transform = true;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        self.recompute_world_transform = true;
    }

    pub fn get_position(&self) -> Vector2 {
        self.position
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn get_world_transform(&self) -> Matrix4 {
        self.world_transform
    }

    pub fn get_forward(&self) -> Vector2 {
        Vector2::new(self.rotation.cos(), self.rotation.sin())
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn get_game(&self) -> Rc<RefCell<Game>> {
        Rc::clone(&self.game)
    }
}

impl Drop for Actor {
    fn drop(&mut self) {
        let self_as_rc = Rc::new(RefCell::new(self.clone()));
        self.game.borrow_mut().remove_actor(Rc::clone(&self_as_rc));
        while let Some(component) = self.components.pop() {
            drop(component);
        }
    }
}
