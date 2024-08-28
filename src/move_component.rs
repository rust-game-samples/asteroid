use crate::actor::Actor;
// use crate::component::Component;
use crate::math::math::near_zero;
use std::rc::Rc;
use std::cell::RefCell;

pub struct MoveComponent {
    owner: Rc<RefCell<Actor>>,
    angular_speed: f32,
    forward_speed: f32,
}

impl MoveComponent {
    pub fn new(owner: Rc<RefCell<Actor>>, update_order: i32) -> Self {
        // <MoveComponent as Component>::new(owner.clone(), update_order);
        Self {
            owner,
            angular_speed: 0.0,
            forward_speed: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let mut owner = self.owner.borrow_mut();

        // 角速度に基づいて回転を更新
        if !near_zero(self.angular_speed, 0.001) {
            let mut rotation = owner.get_rotation();
            rotation += self.angular_speed * delta_time;
            owner.set_rotation(rotation);
        }

        // 前進速度に基づいて位置を更新
        if !near_zero(self.forward_speed, 0.001) {
            let mut position = owner.get_position();
            position += owner.get_forward() * self.forward_speed * delta_time;

            // スクリーンのラッピング（アステロイドのため）
            if position.x < -512.0 {
                position.x = 510.0;
            } else if position.x > 512.0 {
                position.x = -510.0;
            }
            if position.y < -384.0 {
                position.y = 382.0;
            } else if position.y > 384.0 {
                position.y = -382.0;
            }

            owner.set_position(position);
        }
    }

    // Getter and Setter
    pub fn get_angular_speed(&self) -> f32 {
        self.angular_speed
    }

    pub fn set_angular_speed(&mut self, speed: f32) {
        self.angular_speed = speed;
    }

    pub fn get_forward_speed(&self) -> f32 {
        self.forward_speed
    }

    pub fn set_forward_speed(&mut self, speed: f32) {
        self.forward_speed = speed;
    }
}
