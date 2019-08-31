use amethyst::core::math::Vector2;
use serde::{Deserialize, Serialize};

/// Keeps track of the wind conditions in the world.
/// Currently, wind is represented by a 2D vector.
#[derive(Deserialize, Serialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Wind {
    pub wind: Vector2<f32>,
}

impl Wind {
    pub fn new(x: f32, y: f32) -> Wind {
        Wind {
            wind: Vector2::new(x, y),
        }
    }
}

impl Default for Wind {
    fn default() -> Self {
        Wind::new(0.0, 0.0)
    }
}
