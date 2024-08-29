use crate::actor::Actor;
use std::rc::Rc;
use std::cell::RefCell;
use crate::component::{Component, BaseComponent};

#[derive(Clone)]
pub struct MoveComponent {
    base: BaseComponent,
    angular_speed: f32,
    forward_speed: f32,
}

impl MoveComponent {
    pub fn new(owner: Rc<RefCell<Actor>>, update_order: i32) -> Self {
        MoveComponent {
            base: BaseComponent::new(owner, update_order),
            angular_speed: 0.0,
            forward_speed: 0.0,
        }
    }

    // Getter and Setter
    pub fn get_angular_speed(&self) -> f32 {
        self.angular_speed
    }

    pub fn get_forward_speed(&self) -> f32 {
        self.forward_speed
    }

    pub fn set_angular_speed(&mut self, speed: f32) {
        self.angular_speed = speed;
    }

    pub fn set_forward_speed(&mut self, speed: f32) {
        self.forward_speed = speed;
    }
}

impl Component for MoveComponent {
    fn new(owner: Rc<RefCell<Actor>>, update_order: i32) -> Self {
        MoveComponent::new(owner, update_order)
    }

    fn update(&mut self, delta_time: f32) {
        if self.angular_speed.abs() > f32::EPSILON {
            let mut rot = self.base.owner.borrow().get_rotation();
            rot += self.angular_speed * delta_time;
            self.base.owner.borrow_mut().set_rotation(rot);
        }

        if self.forward_speed.abs() > f32::EPSILON {
            let mut pos = self.base.owner.borrow().get_position();
            let forward = self.base.owner.borrow().get_forward();
            pos += forward * self.forward_speed * delta_time;

            // 画面外に出た場合のスクリーンラップ処理（Asteroid専用）
            if pos.x < 0.0 { pos.x = 1022.0; }
            else if pos.x > 1024.0 { pos.x = 2.0; }

            if pos.y < 0.0 { pos.y = 766.0; }
            else if pos.y > 768.0 { pos.y = 2.0; }

            self.base.owner.borrow_mut().set_position(pos);
        }
    }

    fn get_update_order(&self) -> i32 {
        self.base.get_update_order()
    }
}

impl Drop for MoveComponent {
    fn drop(&mut self) {
        self.base.owner.borrow_mut().remove_component(Rc::new(RefCell::new(self.clone())));
    }
}
