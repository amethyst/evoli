use crate::resources::world_bounds::WorldBounds;
use amethyst::{
    ecs::{System, Write, WriteExpect},
    renderer::{debug_drawing::DebugLines, palette::Srgba},
};

pub struct DebugSystem;
impl<'s> System<'s> for DebugSystem {
    type SystemData = (Write<'s, DebugLines>, WriteExpect<'s, WorldBounds>);

    fn run(&mut self, (mut debug_lines_resource, bounds): Self::SystemData) {
        let color = (0.8, 0.04, 0.6, 1.0);
        debug_lines_resource.draw_line(
            [bounds.left, bounds.bottom, 0.0].into(),
            [bounds.right, bounds.bottom, 0.0].into(),
            Srgba::from_components(color),
        );

        debug_lines_resource.draw_line(
            [bounds.left, bounds.top, 0.0].into(),
            [bounds.right, bounds.top, 0.0].into(),
            Srgba::from_components(color),
        );

        debug_lines_resource.draw_line(
            [bounds.left, bounds.bottom, 0.0].into(),
            [bounds.left, bounds.top, 0.0].into(),
            Srgba::from_components(color),
        );

        debug_lines_resource.draw_line(
            [bounds.right, bounds.bottom, 0.0].into(),
            [bounds.right, bounds.top, 0.0].into(),
            Srgba::from_components(color),
        );

        debug_lines_resource.draw_line(
            [0.0, 0.0, 0.0].into(),
            [1.0, 0.0, 0.0].into(),
            Srgba::new(1.0, 0.0, 0.0, 1.0),
        );
        debug_lines_resource.draw_line(
            [0.0, 0.0, 0.0].into(),
            [0.0, 1.0, 0.0].into(),
            Srgba::new(0.0, 1.0, 0.0, 1.0),
        );
        debug_lines_resource.draw_line(
            [0.0, 0.0, 0.0].into(),
            [0.0, 0.0, 1.0].into(),
            Srgba::new(0.0, 0.0, 1.0, 1.0),
        );
    }
}
