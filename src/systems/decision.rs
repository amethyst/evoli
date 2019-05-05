use amethyst::core::nalgebra::*;
use amethyst::core::transform::Transform;
use amethyst::core::Time;
use amethyst::ecs::*;

use crate::components::combat::{FactionEnemies, HasFaction};
use crate::components::creatures::*;

pub struct DecisionSystem;
impl<'s> System<'s> for DecisionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Movement>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, HasFaction<Entity>>,
        ReadStorage<'s, FactionEnemies>,
        ReadStorage<'s, IntelligenceTag>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, mut movements, transforms, has_faction, faction_enemies, intelligence_tag, time): Self::SystemData,
    ) {
        let delta_time = time.delta_seconds();
        for (movement, transform, faction, _) in
            (&mut movements, &transforms, &has_faction, &intelligence_tag).join()
        {
            let enemies_opt = faction_enemies.get(faction.faction);
            if enemies_opt.is_none() {
                continue;
            }

            let mut shortest: Option<Vector3<f32>> = None;
            let mut min_sq_distance = 5.0f32.powi(2);

            let enemies = enemies_opt.unwrap();

            for (other_transform, _entity, enemy_faction) in
                (&transforms, &entities, &has_faction).join()
            {
                if !enemies.is_enemy(&enemy_faction.faction) {
                    continue;
                }

                let position = transform.translation();
                let other_position = other_transform.translation();
                let difference = other_position - position;
                let sq_distance = difference.magnitude_squared();
                if sq_distance < min_sq_distance {
                    min_sq_distance = sq_distance;
                    shortest = Some(difference);
                }
            }

            if let Some(vector) = shortest {
                let turn_rate = 10.0;
                movement.velocity += vector * turn_rate * delta_time;
            }
        }
    }
}
