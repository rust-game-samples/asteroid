use std::rc::Rc;
use std::cell::RefCell;
use crate::actor::Actor;

pub trait Component {
    fn new(owner: Rc<RefCell<Actor>>, update_order: i32) -> Self
    where
        Self: Sized;

    fn drop(&mut self) {}

    fn process_input(&mut self, _key_state: &[u8]) {}

    fn update(&mut self, delta_time: f32);

    fn on_update_world_transform(&mut self) {}

    fn get_update_order(&self) -> i32;
}

#[derive(Clone)]
pub struct BaseComponent {
    owner: Rc<RefCell<Actor>>,
    update_order: i32,
}

impl BaseComponent {
    pub fn new(owner: Rc<RefCell<Actor>>, update_order: i32) -> Self {
        let mut component = BaseComponent {
            owner: owner.clone(),
            update_order,
        };

        // Add to actor's vector of components
        owner.borrow_mut().add_component(Rc::new(RefCell::new(component.clone())));

        component
    }
}

impl Component for BaseComponent {
    fn new(owner: Rc<RefCell<Actor>>, update_order: i32) -> Self {
        BaseComponent {
            owner,
            update_order,
        }
    }

    fn update(&mut self, _delta_time: f32) {
        // Default implementation for the update method.
    }

    fn get_update_order(&self) -> i32 {
        self.update_order
    }
}

impl Drop for BaseComponent {
    fn drop(&mut self) {
        self.owner.borrow_mut().remove_component(Rc::new(RefCell::new(self.clone())));
    }
}
