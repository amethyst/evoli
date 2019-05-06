use amethyst::{core::timing::Time, core::transform::Transform, ecs::*, shrev::EventChannel};

use rand::{thread_rng, RngCore};

use crate::systems::spawner::CreatureSpawnEvent;

#[derive(Default)]
pub struct BeeSpawnSystem {
    bee_timer: f32,
}

impl<'s> System<'s> for BeeSpawnSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Write<'s, EventChannel<CreatureSpawnEvent>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, lazy_update, mut spawn_events, time): Self::SystemData) {
        let delta_seconds = time.delta_seconds();
        self.bee_timer -= delta_seconds;
        if self.bee_timer <= 0.0 {
            let mut creature_entity_builder = lazy_update.create_entity(&entities);
            let mut rng = thread_rng();
            self.bee_timer = 1.0f32;
            let x = (rng.next_u32() % 100) as f32 / 5.0 - 10.0;
            let y = (rng.next_u32() % 100) as f32 / 5.0 - 10.0;
            let mut transform = Transform::default();
            transform.set_xyz(x, y, 0.0);
            transform.set_scale(0.3, 0.3, 1.0);
            creature_entity_builder = creature_entity_builder.with(transform);
            spawn_events.single_write(CreatureSpawnEvent {
                creature_type: "Bee".to_string(),
                entity: creature_entity_builder.build(),
            });
        }
    }
}
