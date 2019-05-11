use amethyst::core::nalgebra::Vector3;
use amethyst::{
    core::Transform,
    ecs::{join::Join, Entities, Entity, Read, ReadStorage, System, WriteStorage},
};

use std::cmp::Ordering;

use crate::components::collider;
use crate::components::combat::{FactionPrey, Factions, HasFaction};
use crate::components::creatures::Movement;
use crate::resources::world_bounds::WorldBounds;
use crate::systems::behaviors::decision::{Closest, Friend, Predator, Prey, Query};

#[derive(Default)]
pub struct Avoid;

pub struct QueryAvoidSystem;
impl<'s> System<'s> for QueryAvoidSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, collider::Circle>,
        ReadStorage<'s, Query<Prey>>,
        ReadStorage<'s, Query<Predator>>,
        ReadStorage<'s, Query<Friend>>,
        ReadStorage<'s, FactionPrey<Entity>>,
        WriteStorage<'s, Query<Avoid>>,
    );

    fn run(
        &mut self,
        (
            entities,
            circles,
            prey_queries,
            predator_queries,
            friend_queries,
            faction_prey,
            mut query,
        ): Self::SystemData,
    ) {
        for (entity, _) in (&entities, &faction_prey).join() {
            let preys = &prey_queries.get(entity).unwrap().0;
            let predators = &predator_queries.get(entity).unwrap().0;
            let friends = &friend_queries.get(entity).unwrap().0;

            if let Ok(entry) = query.entry(entity) {
                entry.or_insert(Query::<Avoid>::new()).0.clear();
            }
            let mut avoid_set = &mut query.get_mut(entity).unwrap().0;

            // All circle colliders that are neither preys nor predators
            for (other_entity, circle, _, _, _) in
                (&entities, &circles, &!preys, &!predators, &!friends).join()
            {
                avoid_set.add(other_entity.id());
            }
        }
    }
}

fn closet_wall(location: &Vector3<f32>, bounds: &WorldBounds) -> Vector3<f32> {
    let mut bounds_left = location.clone();
    bounds_left.x = bounds.left;
    let mut bounds_right = location.clone();
    bounds_right.x = bounds.right;
    let mut bounds_top = location.clone();
    bounds_top.y = bounds.top;
    let mut bounds_bottom = location.clone();
    bounds_bottom.y = bounds.bottom;

    [bounds_left, bounds_right, bounds_top, bounds_bottom]
        .iter()
        .map(|v| v - location)
        .min_by(|a, b| {
            if a.magnitude_squared() < b.magnitude_squared() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .unwrap()
}

#[test]
fn test_closest_wall() {
    let bounds = WorldBounds::new(-5.0, 5.0, 5.0, -5.0);
    let location = Vector3::new(-4.0, 0.0, 0.0);
    let expected_direction = Vector3::new(-1.0, 0.0, 0.0);
    assert_eq!(closet_wall(&location, &bounds), expected_direction);

    let location2 = Vector3::new(0.0, 4.0, 0.0);
    let expected_direction2 = Vector3::new(0.0, 1.0, 0.0);
    assert_eq!(closet_wall(&location2, &bounds), expected_direction2);
}

pub struct ClosestWallSystem;
impl<'s> System<'s> for ClosestWallSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Movement>,
        Read<'s, WorldBounds>,
        WriteStorage<'s, Closest<Avoid>>,
    );

    fn run(
        &mut self,
        (entities, transforms, movements, world_bounds, mut closest_avoid): Self::SystemData,
    ) {
        for (entity, transform, _, mut closest_avoid) in
            (&entities, &transforms, &movements, closest_avoid.entries()).join()
        {
            let closest_wall_direction = closet_wall(&transform.translation(), &world_bounds);
            if closest_wall_direction.magnitude_squared() < 5.0f32.powi(2)
                && closest_wall_direction.magnitude_squared() > 0.0
            {
                let mut avoid =
                    closest_avoid.or_insert(Closest::<Avoid>::new(closest_wall_direction));
                if avoid.distance.magnitude_squared() > closest_wall_direction.magnitude_squared() {
                    *avoid = Closest::<Avoid>::new(closest_wall_direction);
                }
            }
        }
    }
}
