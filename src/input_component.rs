use std::rc::Rc;
use std::cell::RefCell;
use crate::actor::Actor;
use crate::component::{Component};
use crate::move_component::MoveComponent;

pub struct InputComponent {
    move_component: MoveComponent,
    max_forward_speed: f32,
    max_angular_speed: f32,
    forward_key: u8,
    back_key: u8,
    clockwise_key: u8,
    counter_clockwise_key: u8,
}

impl InputComponent {
    pub fn new(owner: Rc<RefCell<Actor>>, update_order: i32) -> Self {
        InputComponent {
            move_component: MoveComponent::new(owner, update_order),
            max_forward_speed: 0.0,
            max_angular_speed: 0.0,
            forward_key: 0,
            back_key: 0,
            clockwise_key: 0,
            counter_clockwise_key: 0,
        }
    }

    pub fn get_max_forward_speed(&self) -> f32 {
        self.max_forward_speed
    }

    pub fn get_max_angular_speed(&self) -> f32 {
        self.max_angular_speed
    }

    pub fn get_forward_key(&self) -> u8 {
        self.forward_key
    }

    pub fn get_back_key(&self) -> u8 {
        self.back_key
    }

    pub fn get_clockwise_key(&self) -> u8 {
        self.clockwise_key
    }

    pub fn get_counter_clockwise_key(&self) -> u8 {
        self.counter_clockwise_key
    }

    pub fn set_max_forward_speed(&mut self, speed: f32) {
        self.max_forward_speed = speed;
    }

    pub fn set_max_angular_speed(&mut self, speed: f32) {
        self.max_angular_speed = speed;
    }

    pub fn set_forward_key(&mut self, key: u8) {
        self.forward_key = key;
    }

    pub fn set_back_key(&mut self, key: u8) {
        self.back_key = key;
    }

    pub fn set_clockwise_key(&mut self, key: u8) {
        self.clockwise_key = key;
    }

    pub fn set_counter_clockwise_key(&mut self, key: u8) {
        self.counter_clockwise_key = key;
    }
}

impl Component for InputComponent {
    fn new(owner: Rc<RefCell<Actor>>, update_order: i32) -> Self {
        InputComponent::new(owner, update_order)
    }

    fn update(&mut self, delta_time: f32) {
        self.move_component.update(delta_time);
    }

    fn process_input(&mut self, key_state: &[u8]) {
        // Calculate forward speed for MoveComponent
        let mut forward_speed = 0.0;
        if key_state[self.forward_key as usize] != 0 {
            forward_speed += self.max_forward_speed;
        }
        if key_state[self.back_key as usize] != 0 {
            forward_speed -= self.max_forward_speed;
        }
        self.move_component.set_forward_speed(forward_speed);

        // Calculate angular speed for MoveComponent
        let mut angular_speed = 0.0;
        if key_state[self.clockwise_key as usize] != 0 {
            angular_speed += self.max_angular_speed;
        }
        if key_state[self.counter_clockwise_key as usize] != 0 {
            angular_speed -= self.max_angular_speed;
        }
        self.move_component.set_angular_speed(angular_speed);
    }

    fn get_update_order(&self) -> i32 {
        self.move_component.get_update_order()
    }
}
