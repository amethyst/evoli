use amethyst::{
    core::{math::Vector2, timing::Time},
    ecs::*,
    input::{InputHandler, StringBindings},
};

use crate::resources::wind::Wind;
use std::f32;

/// Wind speed cannot decrease below this number.
const MIN_WIND_SPEED: f32 = 0.0;
/// Wind speed cannot increase above this number.
const MAX_WIND_SPEED: f32 = 5.0;
/// Speed with which to rotate wind speed in radians per second.
const WIND_TURN_SPEED: f32 = f32::consts::FRAC_PI_4;
/// Speed with which to increase or decrease wind speed in meters?? per second per second.
const WIND_ACCELERATION: f32 = 2.0;

/// DebugWindControlSystem allows players to change the wind speed and direction at runtime.
/// Use the ChangeWindDirection input axis to change the wind direction at WIND_TURN_SPEED radians per second.
/// Use the ChangeWindSpeed input axis to change the wind speed between MIN_WIND_SPEED and MAX_WIND_SPEED.
#[derive(Default)]
pub struct DebugWindControlSystem;

impl<'s> System<'s> for DebugWindControlSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, Wind>,
        Read<'s, Time>,
    );

    fn run(&mut self, (input, mut wind, time): Self::SystemData) {
        let change_direction = input
            .axis_value("ChangeWindDirection")
            .filter(|signum| signum.abs() > std::f32::EPSILON);
        let change_speed = input
            .axis_value("ChangeWindSpeed")
            .filter(|signum| signum.abs() > std::f32::EPSILON);
        if change_direction.is_none() && change_speed.is_none() {
            return;
        }
        let new_angle = calc_wind_angle(change_direction, &wind, &time);
        let new_speed = calc_wind_speed(change_speed, &wind, &time);
        wind.wind = Vector2::new(new_speed * new_angle.cos(), new_speed * new_angle.sin());
        println!(
            "Changed wind vector to: ({:?},{:?}) angle={:?} speed={:?}",
            wind.wind.x, wind.wind.y, new_angle, new_speed
        );
    }
}

fn calc_wind_angle(input_signum: Option<f32>, wind: &Wind, time: &Time) -> f32 {
    let old_wind_angle = wind.wind.y.atan2(wind.wind.x);
    if let Some(signum) = input_signum {
        old_wind_angle + signum * WIND_TURN_SPEED * time.delta_seconds()
    } else {
        old_wind_angle
    }
}

fn calc_wind_speed(input_signum: Option<f32>, wind: &Wind, time: &Time) -> f32 {
    let magnitude = wind.wind.magnitude();
    if let Some(signum) = input_signum {
        (magnitude + signum * WIND_ACCELERATION * time.delta_seconds())
            .max(MIN_WIND_SPEED)
            .min(MAX_WIND_SPEED)
    } else {
        magnitude
    }
}
