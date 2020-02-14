use crate::resources::world_bounds::WorldBounds;
use amethyst::{
    core::{
        math::{Vector2, Vector3},
        timing::Time,
        transform::components::Transform,
    },
    ecs::*,
    shrev::EventChannel,
};

use rand::{thread_rng, Rng};
use std::f32;

use crate::{
    components::creatures::FallingTag, components::creatures::Movement,
    components::creatures::TopplegrassTag, resources::wind::Wind,
    systems::spawner::CreatureSpawnEvent,
};

/// A new topplegrass entity is spawned periodically, SPAWN_INTERVAL is the period in seconds.
/// Spawn interval is currently set quite fast, for testing purposes. In the final game,
/// a spawn internal of at least a few minutes might be better.
const SPAWN_INTERVAL: f32 = 10.0;
/// The standard scaling to apply to the entity.
const TOPPLEGRASS_BASE_SCALE: f32 = 0.002;
/// At which height the topplegrass entity should spawn.
const HEIGHT: f32 = 0.5;
/// If we knew the radius of the toppleweed, we could calculate the perfect angular velocity,
/// but instead we'll use this magic value we got through trial and error.
/// It should be close enough to the actual value that the entity doesn't appear to slip.
const ANGULAR_V_MAGIC: f32 = 2.0;
/// The minimum velocity that a topplegrass entity must have in order to start jumping up into the air.
/// This is to prevent topplegrass from jumping in a weird way when there is (almost) no wind.
const JUMP_THRESHOLD: f32 = 1.0;
/// The chance per elapsed second since last frame that any given non-falling
/// topplegrass will jump up into the air slightly.
/// Not a great way of doing it, but probably good enough until we get a physics system?
const JUMP_PROBABILITY: f32 = 4.0;

/// Periodically schedules a Topplegrass entity to be spawned in through a CreatureSpawnEvent.
#[derive(Default)]
pub struct TopplegrassSpawnSystem {
    secs_to_next_spawn: f32,
}

impl<'s> System<'s> for TopplegrassSpawnSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Write<'s, EventChannel<CreatureSpawnEvent>>,
        Read<'s, Time>,
        Read<'s, WorldBounds>,
        Read<'s, Wind>,
    );

    fn run(
        &mut self,
        (entities, lazy_update, mut spawn_events, time, world_bounds, wind): Self::SystemData,
    ) {
        if self.ready_to_spawn(time.delta_seconds()) {
            let mut transform = Transform::default();
            transform.set_scale(Vector3::new(
                TOPPLEGRASS_BASE_SCALE,
                TOPPLEGRASS_BASE_SCALE,
                TOPPLEGRASS_BASE_SCALE,
            ));
            transform.append_translation(Self::gen_spawn_location(&wind, &world_bounds));
            let entity = lazy_update.create_entity(&entities).with(transform).build();
            spawn_events.single_write(CreatureSpawnEvent {
                creature_type: "Topplegrass".to_string(),
                entity,
            });
        }
    }
}

impl TopplegrassSpawnSystem {
    /// Checks the time elapsed since the last spawn. If the system is ready to spawn another
    /// entity, the timer will be reset and this function will return true.
    fn ready_to_spawn(&mut self, delta_seconds: f32) -> bool {
        self.secs_to_next_spawn -= delta_seconds;
        if self.secs_to_next_spawn.is_sign_negative() {
            self.secs_to_next_spawn = SPAWN_INTERVAL;
            true
        } else {
            false
        }
    }

    /// Returns a Vector3<f32> representing the position in which to spawn the next entity.
    /// Entities will be spawned at a random point on one of the four world borders; specifically,
    /// the one that the wind direction is facing away from. In other words: upwind from the
    /// center of the world.
    fn gen_spawn_location(wind: &Wind, bounds: &WorldBounds) -> Vector3<f32> {
        let mut rng = thread_rng();
        if Self::wind_towards_direction(wind.wind, Vector2::new(1.0, 0.0)) {
            Vector3::new(
                bounds.left,
                rng.gen_range(bounds.bottom, bounds.top),
                HEIGHT,
            )
        } else if Self::wind_towards_direction(wind.wind, Vector2::new(0.0, 1.0)) {
            Vector3::new(
                rng.gen_range(bounds.left, bounds.right),
                bounds.bottom,
                HEIGHT,
            )
        } else if Self::wind_towards_direction(wind.wind, Vector2::new(-1.0, 0.0)) {
            Vector3::new(
                bounds.right,
                rng.gen_range(bounds.bottom, bounds.top),
                HEIGHT,
            )
        } else {
            Vector3::new(rng.gen_range(bounds.left, bounds.right), bounds.top, HEIGHT)
        }
    }

    /// Returns true if and only if the given wind vector is roughly in line with the given
    /// cardinal_direction vector, within a margin of a 1/4 PI RAD.
    fn wind_towards_direction(wind: Vector2<f32>, cardinal_direction: Vector2<f32>) -> bool {
        wind.angle(&cardinal_direction).abs() < f32::consts::FRAC_PI_4
    }
}

/// Controls the rolling animation of the Topplegrass.
/// Also makes the entity skip up into the air every so often, to simulate it bumping into small
/// rocks or the wind catching it or something.
#[derive(Default)]
pub struct TopplingSystem;

impl<'s> System<'s> for TopplingSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Movement>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, TopplegrassTag>,
        WriteStorage<'s, FallingTag>,
        Read<'s, Wind>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, mut movements, mut transforms, topple_tags, mut falling_tags, wind, time): Self::SystemData,
    ) {
        let mut rng = thread_rng();
        // Set topplegrass velocity to equal wind velocity.
        // Rotate topplegrass.
        for (movement, transform, _) in (&mut movements, &mut transforms, &topple_tags).join() {
            transform.prepend_rotation_x_axis(
                -ANGULAR_V_MAGIC * movement.velocity.y * time.delta_seconds(),
            );
            transform.prepend_rotation_y_axis(
                ANGULAR_V_MAGIC * movement.velocity.x * time.delta_seconds(),
            );
            movement.velocity.x = wind.wind.x;
            movement.velocity.y = wind.wind.y;
        }
        // Select some of the topplegrass that are on ground to jump up into the air slightly.
        let airborne = (&entities, &mut movements, &topple_tags, !&falling_tags)
            .join()
            .filter_map(|(entity, movement, _, _)| {
                if movement.velocity.magnitude() > JUMP_THRESHOLD
                    && rng.gen::<f32>() < JUMP_PROBABILITY * time.delta_seconds()
                {
                    movement.velocity.z = rng.gen_range(0.4, 0.7);
                    Some(entity)
                } else {
                    None
                }
            })
            .collect::<Vec<Entity>>();
        // Attach the falling tag to the selected topplegrass entities, which lets the GravitySystem
        // know to start affecting it.
        for entity in airborne {
            falling_tags
                .insert(entity, FallingTag)
                .expect("Unable to add falling tag to entity");
        }
        // Check which entities are no longer falling (because they reached the ground); remove
        // their falling tag, set their vertical speed to zero (we don't bounce) and correct their position.
        let no_longer_falling = (
            &entities,
            &mut transforms,
            &mut movements,
            &falling_tags,
            &topple_tags,
        )
            .join()
            .filter_map(|(entity, transform, movement, _, _)| {
                if transform.translation().z <= HEIGHT && movement.velocity.z.is_sign_negative() {
                    transform.translation_mut().z = HEIGHT;
                    movement.velocity.z = 0.0;
                    Some(entity)
                } else {
                    None
                }
            })
            .collect::<Vec<Entity>>();
        for entity in no_longer_falling {
            falling_tags.remove(entity);
        }
    }
}
