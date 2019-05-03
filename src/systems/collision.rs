use amethyst::renderer::DebugLines;
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::{core::Transform, ecs::*, ecs::world::Generation};
use log::info;
use std::f32;

use crate::components::collider;
use crate::components::creatures;
use crate::resources::world_bounds::*;

pub struct EnforceBoundsSystem;

impl<'s> System<'s> for EnforceBoundsSystem {
    type SystemData = (WriteStorage<'s, Transform>, Read<'s, WorldBounds>);

    fn run(&mut self, (mut locals, bounds): Self::SystemData) {
        for local in (&mut locals).join() {
            if local.translation().x > bounds.right {
                local.translation_mut().x = bounds.right;
            } else if local.translation().x < bounds.left {
                local.translation_mut().x = bounds.left;
            }

            if local.translation().y > bounds.top {
                local.translation_mut().y = bounds.top;
            } else if local.translation().y < bounds.bottom {
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
        Write<'s, DebugLines>,
    );

    fn run(&mut self, (circles, locals, mut debug_lines): Self::SystemData) {
        for (circle, local) in (&circles, &locals).join() {
            let position = local.translation();
            debug_lines.draw_line(
                [position.x - circle.radius, position.y, 0.0].into(),
                [position.x + circle.radius, position.y, 0.0].into(),
                [1.0, 0.5, 0.5, 1.0].into(),
            );
            debug_lines.draw_line(
                [position.x, position.y - circle.radius, 0.0].into(),
                [position.x, position.y + circle.radius, 0.0].into(),
                [1.0, 0.5, 0.5, 1.0].into(),
            );
        }
    }
}

pub struct DebugCollisionEventSystem {
    entity: Entity
}

impl DebugCollisionEventSystem {
    pub fn new(entity: Entity) -> DebugCollisionEventSystem {
        DebugCollisionEventSystem {
            entity
        }
    }
}

pub struct MyReader(ReaderId<CollisionEvent>);
impl Component for MyReader {
    type Storage = HashMapStorage<Self>;
}

impl<'s> System<'s> for DebugCollisionEventSystem {
    type SystemData = (
        Write<'s, EventChannel<CollisionEvent>>,
        WriteStorage<'s, MyReader>,
    );

    fn run(&mut self, (mut collision_events, mut readers): Self::SystemData) {
        let mut reader_opt = readers.get_mut(self.entity);
        let mut reader = if reader_opt.is_none() {
            readers.insert(self.entity, MyReader(collision_events.register_reader()))
                .expect("unreachable");
            readers.get_mut(self.entity).unwrap()
        }
        else {
            reader_opt.unwrap()
        };

        for event in collision_events.read(&mut reader.0) {
            info!("Received collision event {:?}", event)
        }
    }
}
