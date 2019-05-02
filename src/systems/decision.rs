use amethyst::core::nalgebra::*;
use amethyst::core::transform::Transform;
use amethyst::core::Time;
use amethyst::ecs::*;

use crate::components::creatures::*;

pub struct DecisionSystem;
impl<'s> System<'s> for DecisionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Movement>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, CarnivoreTag>,
        ReadStorage<'s, HerbivoreTag>,
        ReadStorage<'s, IntelligenceTag>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, mut movements, transforms, carnivore_tag, herbivore_tag, intelligence_tag, time): Self::SystemData,
    ) {
        let delta_time = time.delta_seconds();
        for (movement, transform, _, _) in (
            &mut movements,
            &transforms,
            &carnivore_tag,
            &intelligence_tag,
        )
            .join()
        {
            let mut shortest: Option<Vector3<f32>> = None;

            for (other_transform, _entity, _) in (&transforms, &entities, &herbivore_tag).join() {
                let position = transform.translation();
                let other_position = other_transform.translation();
                let difference = other_position - position;
                if difference.magnitude_squared() < 5.0_f32.powi(2) {
                    shortest = match shortest {
                        Some(vector) => {
                            if difference.magnitude_squared() < vector.magnitude_squared() {
                                Some(difference)
                            } else {
                                Some(vector)
                            }
                        }
                        None => Some(difference),
                    }
                }
            }

            if let Some(vector) = shortest {
                let turn_rate = 10.0;
                movement.velocity += vector * turn_rate * delta_time;
            }
        }
    }
}
