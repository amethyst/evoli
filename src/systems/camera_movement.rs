use amethyst::{
    core::{Named, Time, Transform},
    ecs::*,
    input::{InputHandler, StringBindings},
    renderer::camera::Camera,
};

#[derive(Default)]
pub struct CameraMovementSystem {}

impl<'s> System<'s> for CameraMovementSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Named>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (cameras, names, mut transforms, input_handler, time): Self::SystemData) {
        let delta_time = time.delta_real_seconds();
        let move_factor = 12.0 * delta_time;
        for (_, name, transform) in (&cameras, &names, &mut transforms).join() {
            if name.name == "Main camera" {
                if input_handler.action_is_down("CameraMoveUp").unwrap() {
                    transform.move_up(move_factor);
                }
                if input_handler.action_is_down("CameraMoveDown").unwrap() {
                    transform.move_down(move_factor);
                }
                if input_handler.action_is_down("CameraMoveLeft").unwrap() {
                    transform.move_left(move_factor);
                }
                if input_handler.action_is_down("CameraMoveRight").unwrap() {
                    transform.move_right(move_factor);
                }
                if input_handler.action_is_down("CameraMoveForward").unwrap() {
                    transform.move_forward(move_factor);
                }
                if input_handler.action_is_down("CameraMoveBackward").unwrap() {
                    transform.move_backward(move_factor);
                }
            }
        }
    }
}
