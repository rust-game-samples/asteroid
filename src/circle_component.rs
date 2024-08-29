use std::cell::RefCell;
use std::rc::Rc;
use crate::actor::Actor;
use crate::math::Vector2;
use crate::component::{Component, BaseComponent};

#[derive(Clone)]
pub struct CircleComponent {
    base: BaseComponent,
    radius: f32,
}

impl CircleComponent {
    pub fn new(owner: Rc<RefCell<Actor>>) -> Self {
        let base = BaseComponent::new(owner, 100);
        CircleComponent { base, radius: 0.0 }
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    pub fn get_radius(&self) -> f32 {
        self.base.owner.borrow().get_scale() * self.radius
    }

    pub fn get_center(&self) -> Vector2 {
        self.base.owner.borrow().get_position()
    }
}

impl Component for CircleComponent {
    fn new(owner: Rc<RefCell<Actor>>, update_order: i32) -> Self {
        CircleComponent::new(owner)
    }

    fn update(&mut self, _delta_time: f32) {}

    fn get_update_order(&self) -> i32 {
        self.base.get_update_order()
    }
}

impl Drop for CircleComponent {
    fn drop(&mut self) {
        self.base.owner.borrow_mut().remove_component(Rc::new(RefCell::new(self.clone())));
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
