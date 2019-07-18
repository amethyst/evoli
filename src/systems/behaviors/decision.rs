use amethyst::{
    core::{math::*, transform::Transform, Time},
    ecs::*,
};

use std::f32;
use std::marker::PhantomData;

use crate::components::combat::{FactionPrey, HasFaction};
use crate::components::creatures::*;

/// A query is a component that contains the queried bit set that can be used to join with other components
pub struct Query<T>(BitSet, PhantomData<T>);
impl<T: shred::Resource> Component for Query<T> {
    type Storage = HashMapStorage<Self>;
}

impl<T> Query<T> {
    pub fn new() -> Query<T> {
        Query(BitSet::new(), PhantomData {})
    }
}

/// Tags
#[derive(Default)]
pub struct Prey;
#[derive(Default)]
pub struct Predator;

/// Write prey/predator queries to the faction entities. For each faction
/// we calculate the set of entities that they consider prey and the set of entities they
/// consider as predators.
pub struct QueryPredatorsAndPreySystem;
impl<'s> System<'s> for QueryPredatorsAndPreySystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, HasFaction<Entity>>,
        ReadStorage<'s, FactionPrey<Entity>>,
        WriteStorage<'s, Query<Prey>>,
        WriteStorage<'s, Query<Predator>>,
    );

    fn run(
        &mut self,
        (entities, has_faction, faction_preys_set, mut preys_query, mut predators_query): Self::SystemData,
    ) {
        for (faction, _) in (&entities, &faction_preys_set).join() {
            if !preys_query.contains(faction) {
                preys_query
                    .insert(faction, Query::<Prey>::new())
                    .expect("unreachable: we just queried");
            }
            if !predators_query.contains(faction) {
                predators_query
                    .insert(faction, Query::<Predator>::new())
                    .expect("unreachable: we just queried");
            }

            predators_query.get_mut(faction).unwrap().0.clear();
        }

        for (faction, faction_preys) in (&entities, &faction_preys_set).join() {
            let preys = preys_query.get_mut(faction).unwrap();
            preys.0.clear();
            for (prey, prey_faction) in (&entities, &has_faction).join() {
                if faction_preys.is_prey(&prey_faction.faction) {
                    preys.0.add(prey.id());
                }
            }
        }

        for (predator, predator_faction) in (&entities, &has_faction).join() {
            let predator_preys = faction_preys_set.get(predator_faction.faction).unwrap();
            for (prey_faction, _) in (&entities, &faction_preys_set).join() {
                if predator_preys.is_prey(&prey_faction) {
                    let predators = predators_query.get_mut(prey_faction).unwrap();
                    predators.0.add(predator.id());
                }
            }
        }
    }
}

/// A component that stores the distance to the closest entity. The type T is used to tag the entity.
pub struct Closest<T> {
    pub distance: Vector3<f32>,
    _phantom: PhantomData<T>,
}

impl<T> Closest<T> {
    pub fn new(distance: Vector3<f32>) -> Closest<T> {
        Closest {
            distance,
            _phantom: PhantomData {},
        }
    }
}

impl<T> Component for Closest<T>
where
    T: shred::Resource + Default,
{
    type Storage = DenseVecStorage<Self>;
}

/// A system that returns the closest entity of a query on the faction.
/// To make use of this system, you should attach a `Query<T>` to a faction. The system will
/// attach `Closest<T>` to all entities that have a faction where `Query<T>` is attached. The distance
/// between the entity and the queried entity needs to be at least 5.0f32. If the distance is higher,
/// `Closest<T>` will not be attached.
#[derive(Default)]
pub struct ClosestSystem<T: Default>(PhantomData<T>);

impl<'s, T> System<'s> for ClosestSystem<T>
where
    T: shred::Resource + Default,
{
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, HasFaction<Entity>>,
        ReadStorage<'s, Query<T>>,
        WriteStorage<'s, Closest<T>>,
    );

    fn run(
        &mut self,
        (entities, transforms, factions, faction_query, mut closest): Self::SystemData,
    ) {
        for (entity, transform, faction) in (&entities, &transforms, &factions).join() {
            // Remove the old value. The referenced might have moved or has been deleted.
            closest.remove(entity);

            // If the query is not attached to the faction, we don't calculate the closest entity.
            let query_entities = faction_query.get(faction.faction);
            if query_entities.is_none() {
                continue;
            }

            let mut closest_opt = None;
            let mut min_sq_distance = 5.0f32.powi(2);

            for (_, query_transform) in (&query_entities.unwrap().0, &transforms).join() {
                let position = transform.translation();
                let query_position = query_transform.translation();
                let difference = query_position - position;
                let sq_distance = difference.magnitude_squared();
                if sq_distance < min_sq_distance {
                    min_sq_distance = sq_distance;
                    closest_opt = Some(difference);
                }
            }

            if let Some(c) = closest_opt {
                let closest_component = Closest::new(c);
                closest
                    .insert(entity, closest_component)
                    .expect("unreachable: we just queried");
            }
        }
    }
}

/// Seek out the entity referenced by `Closest<T>` and apply a steering force
/// towards that entity. The steering force can be modified using the `attraction_modifier` factor.
/// By setting `attraction_modifier` to `-1` this system will behave like `Evade`.
pub struct SeekSystem<T> {
    attraction_modifier: Rotation3<f32>,
    attraction_magnitude: f32,
    _phantom: PhantomData<T>,
}

impl<T> SeekSystem<T> {
    pub fn new(attraction_modifier: Rotation3<f32>, attraction_magnitude: f32) -> SeekSystem<T> {
        SeekSystem {
            attraction_modifier,
            attraction_magnitude,
            _phantom: PhantomData {},
        }
    }
}

impl<'s, T> System<'s> for SeekSystem<T>
where
    T: shred::Resource + Default,
{
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Closest<T>>,
        Read<'s, Time>,
        WriteStorage<'s, Movement>,
    );

    fn run(&mut self, (_entities, closest_things, time, mut movements): Self::SystemData) {
        let delta_time = time.delta_seconds();
        for (movement, closest) in (&mut movements, &closest_things).join() {
            if closest.distance.norm() < f32::EPSILON {
                continue;
            }
            let target_velocity = closest.distance.normalize() * self.attraction_magnitude;
            let steering_force = target_velocity - movement.velocity;
            movement.velocity += self.attraction_modifier * steering_force * delta_time;
        }
    }
}
