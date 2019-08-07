use amethyst::renderer::{debug_drawing::DebugLinesComponent, palette::Srgba};
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::{core::math::Point3, core::Transform, ecs::prelude::*};
use log::info;
use std::f32;
#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

use crate::components::collider;
use crate::components::creatures;
use crate::resources::world_bounds::*;

pub struct EnforceBoundsSystem;

impl<'s> System<'s> for EnforceBoundsSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, creatures::CreatureTag>,
        ReadExpect<'s, WorldBounds>,
    );

    fn run(&mut self, (mut locals, tags, bounds): Self::SystemData) {
        for (local, _) in (&mut locals, &tags).join() {
            let pos = local.translation().clone();
            if pos.x > bounds.right {
                local.translation_mut().x = bounds.right;
            } else if pos.x < bounds.left {
                local.translation_mut().x = bounds.left;
            }

            if pos.y > bounds.top {
                local.translation_mut().y = bounds.top;
            } else if pos.y < bounds.bottom {
                local.translation_mut().y = bounds.bottom;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

impl CollisionEvent {
    pub fn new(entity_a: Entity, entity_b: Entity) -> CollisionEvent {
        CollisionEvent { entity_a, entity_b }
    }
}

/// The collision system uses a simple way to calculate collision, at the cost of performance. This is
/// intended! If there are a lot of entities, collisions should be handled by a real physics engine. As soon
/// as a physics integration for Amethyst exists, we are going to switch to that for collision detection.
pub struct CollisionSystem;
impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        ReadStorage<'s, collider::Circle>,
        WriteStorage<'s, creatures::Movement>,
        WriteStorage<'s, Transform>,
        Entities<'s>,
        Write<'s, EventChannel<CollisionEvent>>,
    );

    fn run(
        &mut self,
        (circles, mut movements, locals, entities, mut collision_events): Self::SystemData,
    ) {
        #[cfg(feature = "profiler")]
        profile_scope!("collision_system");
        for (circle_a, movement, local_a, entity_a) in
            (&circles, &mut movements, &locals, &entities).join()
        {
            for (circle_b, local_b, entity_b) in (&circles, &locals, &entities).join() {
                if entity_a == entity_b {
                    continue;
                }

                let allowed_distance = circle_a.radius + circle_b.radius;
                let direction = local_a.translation() - local_b.translation();
                if direction.magnitude_squared() < allowed_distance * allowed_distance {
                    collision_events.single_write(CollisionEvent::new(entity_a, entity_b));

                    if direction.magnitude() < f32::EPSILON {
                        movement.velocity = -movement.velocity;
                    } else {
                        let norm_direction = direction.normalize();
                        movement.velocity = norm_direction * movement.velocity.magnitude();
                    }
                }
            }
        }
    }
}

pub struct DebugColliderSystem;

impl<'s> System<'s> for DebugColliderSystem {
    type SystemData = (
        ReadStorage<'s, collider::Circle>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, DebugLinesComponent>,
    );

    fn run(&mut self, (circles, locals, mut debug_lines_comps): Self::SystemData) {
        for (circle, local, db_comp) in (&circles, &locals, &mut debug_lines_comps).join() {
            let mut position = local.global_matrix().column(3).xyz();
            position[2] += 1.0;
            db_comp.add_circle_2d(
                Point3::from(position),
                circle.radius,
                16,
                Srgba::new(1.0, 0.5, 0.5, 1.0),
            );
        }
    }
}

#[derive(Default)]
pub struct DebugCollisionEventSystem {
    event_reader: Option<ReaderId<CollisionEvent>>,
}

impl<'s> System<'s> for DebugCollisionEventSystem {
    type SystemData = (Write<'s, EventChannel<CollisionEvent>>,);

    fn run(&mut self, (collision_events,): Self::SystemData) {
        let event_reader = self
            .event_reader
            .as_mut()
            .expect("`DebugCollisionEventSystem::setup` was not called before `DebugCollisionEventSystem::run`");

        for event in collision_events.read(event_reader) {
            info!("Received collision event {:?}", event)
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.event_reader = Some(
            res.fetch_mut::<EventChannel<CollisionEvent>>()
                .register_reader(),
        );
    }
}
