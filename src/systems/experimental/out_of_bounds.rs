use crate::resources::world_bounds::WorldBounds;
use amethyst::{core::transform::components::Transform, ecs::*};

use crate::components::creatures::DespawnWhenOutOfBoundsTag;

/// Deletes any entity tagged with DespawnWhenOutOfBoundsTag if they are detected to be outside
/// the world bounds.
#[derive(Default)]
pub struct OutOfBoundsDespawnSystem;

impl<'s> System<'s> for OutOfBoundsDespawnSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, DespawnWhenOutOfBoundsTag>,
        ReadExpect<'s, WorldBounds>,
    );

    fn run(&mut self, (entities, locals, tags, bounds): Self::SystemData) {
        for (entity, local, _) in (&*entities, &locals, &tags).join() {
            let pos = local.translation();
            if pos.x > bounds.right
                || pos.x < bounds.left
                || pos.y > bounds.top
                || pos.y < bounds.bottom
            {
                let _ = entities.delete(entity);
            }
        }
    }
}
