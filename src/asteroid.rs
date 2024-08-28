use crate::actor::Actor;
use crate::circle_component::CircleComponent;
use crate::game::Game;
// use crate::move_component::MoveComponent;
use crate::random::Random;
use crate::sprite_component::SpriteComponent;
use crate::math::Vector2;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Asteroid {
    actor: Actor,
    circle: Option<Rc<CircleComponent>>,
}

impl Asteroid {
    pub fn new(game: Rc<RefCell<Game>>) ->  Rc<RefCell<Self>> {
        let asteroid = Rc::new(
            RefCell::new(Self {
                actor: Actor::new(game.clone()),
                circle: None,
            }));

        // Initialize to random position/orientation
        let rand_pos = Random::get_vector2(Vector2::new(-512.0, -384.0), Vector2::new(512.0, 384.0));
        asteroid.borrow_mut().actor.set_position(rand_pos);

        asteroid.borrow_mut().actor.set_rotation(Random::get_float_range(0.0, std::f32::consts::PI * 2.0));

        // Create a sprite component
        let rc_asteroid_actor = Rc::new(RefCell::new(asteroid.borrow().actor.clone()));
        let mut sprite_component = SpriteComponent::new(rc_asteroid_actor.clone(), 100);
        sprite_component.set_texture(game.borrow_mut().get_texture("Assets/Asteroid.png").unwrap());

        // Create a move component, and set a forward speed
        // let mut move_component = MoveComponent::new(rc_asteroid_actor.clone(), 150);
        // move_component.set_forward_speed(150.0);

        // Create a circle component (for collision)
        let mut circle_component = CircleComponent::new(rc_asteroid_actor.clone());
        circle_component.set_radius(40.0);
        asteroid.borrow_mut().circle = Some(Rc::new(circle_component));

        // Add to mAsteroids in game
        game.borrow_mut().add_asteroid(asteroid.clone());

        asteroid
    }

    pub fn get_circle(&self) -> Option<Rc<CircleComponent>> {
        self.circle.clone()
    }
}

impl Drop for Asteroid {
    fn drop(&mut self) {
        let asteroid = Rc::new(RefCell::new(self.clone()));
        self.actor.get_game().borrow_mut().remove_asteroid(asteroid);
    }
}
