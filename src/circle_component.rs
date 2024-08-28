use std::cell::RefCell;
use std::rc::Rc;
use crate::actor::Actor;
use crate::math::Vector2;

pub struct CircleComponent {
    owner: Rc<RefCell<Actor>>,
    radius: f32,
}

impl CircleComponent {
    pub fn new(owner: Rc<RefCell<Actor>>) -> Self {
        Self {
            owner,
            radius: 0.0,
        }
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    pub fn get_radius(&self) -> f32 {
        self.owner.borrow_mut().get_scale() * self.radius
    }

    pub fn get_center(&self) -> Vector2 {
        self.owner.borrow_mut().get_position()
    }
}

// 円の衝突判定
pub fn intersect(a: &CircleComponent, b: &CircleComponent) -> bool {
    // Calculate distance squared
    let diff = a.get_center() - b.get_center();
    let dist_sq = diff.length_sq();

    // Calculate sum of radii squared
    let radii_sum = a.get_radius() + b.get_radius();
    let radii_sq = radii_sum * radii_sum;

    dist_sq <= radii_sq
}
