use amethyst::{
    core::{
        math::{Vector2, Vector3},
        transform::Transform,
    },
    ecs::{Component, DenseVecStorage},
};

pub struct GlobePosition {
    angles: Vector2<f32>,
    altitude: f32,
}

impl GlobePosition {
    pub fn to_transform(&self) -> Transform {
        let mut transform = Transform::default();
        transform.append_rotation_z_axis(self.angles[1]);
        transform.append_rotation_x_axis(self.angles[0]);
        transform.append_translation_xyz(0.0, 0.0, 1.0);
        transform
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
