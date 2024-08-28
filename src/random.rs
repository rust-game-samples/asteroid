use rand::{thread_rng, Rng};
use crate::math::{Vector2, Vector3};

pub struct Random;

impl Random {
    pub fn get_float() -> f32 {
        Random::get_float_range(0.0, 1.0)
    }

    pub fn get_float_range(min: f32, max: f32) -> f32 {
        let mut rng = thread_rng();
        rng.gen_range(min..max)
    }

    pub fn get_int_range(min: i32, max: i32) -> i32 {
        let mut rng = thread_rng();
        rng.gen_range(min..=max)
    }

    pub fn get_vector2(min: Vector2, max: Vector2) -> Vector2 {
        let r = Vector2::new(Random::get_float(), Random::get_float());
        min + (max - min) * r
    }

    pub fn get_vector3(min: Vector3, max: Vector3) -> Vector3 {
        let r = Vector3::new(
            Random::get_float(),
            Random::get_float(),
            Random::get_float(),
        );
        min + (max - min) * r
    }
}
