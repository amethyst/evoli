use amethyst::core::nalgebra::Vector3;
use amethyst::{
    core::Transform,
    ecs::{join::Join, Entities, Read, ReadStorage, System, WriteStorage},
};

use std::cmp::Ordering;

use crate::components::creatures::Movement;
use crate::resources::world_bounds::WorldBounds;
use crate::systems::behaviors::decision::Closest;

#[derive(Default)]
pub struct Obstacle;

/// Determine the closest bounding wall based on a location
fn closest_wall(location: &Vector3<f32>, bounds: &WorldBounds) -> Vector3<f32> {
    let mut bounds_left = location.clone();
    bounds_left.x = bounds.left;
    let mut bounds_right = location.clone();
    bounds_right.x = bounds.right;
    let mut bounds_top = location.clone();
    bounds_top.y = bounds.top;
    let mut bounds_bottom = location.clone();
    bounds_bottom.y = bounds.bottom;

    // Iterates through each bound
    [bounds_left, bounds_right, bounds_top, bounds_bottom]
        .iter()
        // Calculates the distance between the wall & the location
        .map(|v| v - location)
        // Returns the minimum distance
        .min_by(|a, b| {
            if a.magnitude_squared() < b.magnitude_squared() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .unwrap()
}

pub struct ClosestObstacleSystem;
impl<'s> System<'s> for ClosestObstacleSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Movement>,
        Read<'s, WorldBounds>,
        WriteStorage<'s, Closest<Obstacle>>,
    );

    fn run(
        &mut self,
        (entities, transforms, movements, world_bounds, mut closest_obstacle): Self::SystemData,
    ) {
        // Right now the only obstacles are the world bound walls, so it's
        // safe to clear this out.
        closest_obstacle.clear();

        for (entity, transform, _) in (&entities, &transforms, &movements).join() {
            // Find the closest wall to this entity
            let wall_dir = closest_wall(&transform.translation(), &world_bounds);
            if wall_dir.magnitude_squared() < 3.0f32.powi(2) {
                closest_obstacle
                    .insert(entity, Closest::<Obstacle>::new(wall_dir))
                    .expect("Unable to add obstacle to entity");
            }
        }
    }
}
