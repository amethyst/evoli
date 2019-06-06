use amethyst::{
    core::nalgebra::{Vector2, Vector3},
    ecs::{Component, DenseVecStorage},
};


pub struct GlobePosition {
    angles: Vector2<f32>,
    altitude: f32,
}

impl GlobePosition {
    pub fn to_world_position_and_rotation(&self) -> (Vector3<f32>, Vector3<f32>) {
        let cos_theta = self.angles[1].cos();
        let sin_theta = self.angles[1].sin();
        let cos_phi = self.angles[0].cos();
        let sin_phi = self.angles[0].sin();

        (self.altitude * Vector3::new(cos_phi * sin_theta, sin_phi * sin_theta, cos_theta), Vector3::new()
    }

    pub fn from_world_position(pos: &Vector3<f32>) -> Self {
        let altitude = pos.coords.norm();
        if altitude == 0.0 {
            return GlobePosition {
                angles: Vector2::new(0.0, 0.0),
                altitude,
            };
        }

        let theta = (pos[2] / altitude).acos();
        let phi = pos[1].atan2(pos[0]);

        GlobePosition {
            angles: Vector2::new(phi, theta),
            altitude,
        }
    }
}

pub struct GlobeVelocity {
    angle_velocity: Vector2<f32>,
    altitude_velocity: f32,
}