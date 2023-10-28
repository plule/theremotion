use std::f32::consts::PI;

use nalgebra::{UnitQuaternion, Vector3};

const HALF_PI: f32 = PI / 2.0;

#[derive(Debug, PartialEq, Eq)]
pub enum HandType {
    Left,
    Right,
}

pub struct HandMessage {
    pub hand_type: HandType,
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub rotation: UnitQuaternion<f32>,
    pub pinch: f32,
    pub grab: f32,
}

impl HandMessage {
    pub fn x_factor(&self) -> f32 {
        match self.hand_type {
            // The left hand goes away from the body in the negative x
            HandType::Left => -1.0,
            // The right hand goes away from the body in the positive x
            HandType::Right => 1.0,
        }
    }

    pub fn position_from_body(&self) -> Vector3<f32> {
        Vector3::new(
            self.x_factor() * self.position.x,
            self.position.y,
            self.position.z,
        )
    }

    pub fn velocity_from_body(&self) -> Vector3<f32> {
        Vector3::new(
            self.x_factor() * self.velocity.x,
            self.velocity.y,
            self.velocity.z,
        )
    }

    pub fn rotation_from_body(&self) -> Option<f32> {
        let angle = -self.rotation.euler_angles().2 * self.x_factor();
        if angle < PI && angle > -HALF_PI {
            Some(angle)
        } else {
            None
        }
    }
}
