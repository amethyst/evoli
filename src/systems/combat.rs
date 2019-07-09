use amethyst::{
    core::Time,
    ecs::*,
    shrev::{EventChannel, ReaderId},
};

use crate::components::combat;
use crate::components::combat::{Cooldown, Damage, Health, Speed};
use crate::components::digestion::{Fullness, Nutrition};
use crate::systems::collision::CollisionEvent;
//#[cfg(test)]
//use amethyst::Error;
//#[cfg(test)]
//use amethyst_test::AmethystApplication;
use std::f32;
use std::time::Duration;

pub struct CooldownSystem;

impl<'s> System<'s> for CooldownSystem {
    type SystemData = (WriteStorage<'s, Cooldown>, Entities<'s>, Read<'s, Time>);

    fn run(&mut self, (mut cooldowns, entities, time): Self::SystemData) {
        let mut to_remove = Vec::new();

        for (mut cooldown, entity) in (&mut cooldowns, &*entities).join() {
            match cooldown.time_left.checked_sub(time.delta_time()) {
                Some(time_left) => {
                    cooldown.time_left = time_left;
                }
                None => {
                    to_remove.push(entity);
                }
            }
        }

        for entity in &to_remove {
            cooldowns.remove(*entity);
        }
    }
}

///
///
///
pub struct AttackEvent {
    pub attacker: Entity,
    pub defender: Entity,
}

#[derive(Default)]
pub struct PerformDefaultAttackSystem {
    event_reader: Option<ReaderId<AttackEvent>>,
}

impl<'s> System<'s> for PerformDefaultAttackSystem {
    type SystemData = (
        Read<'s, EventChannel<AttackEvent>>,
        ReadStorage<'s, Damage>,
        WriteStorage<'s, Cooldown>,
        ReadStorage<'s, Speed>,
        WriteStorage<'s, Health>,
        WriteStorage<'s, Fullness>,
        ReadStorage<'s, Nutrition>,
    );

    fn run(
        &mut self,
        (attack_events, damages, mut cooldowns, speeds, mut healths, mut fullnesses, nutritions): Self::SystemData,
    ) {
        let event_reader = self
            .event_reader
            .as_mut()
            .expect("`PerformDefaultAttackSystem::setup` was not called before `PerformDefaultAttackSystem::run`");

        for event in attack_events.read(event_reader) {
            let mut attack_set = BitSet::new();
            attack_set.add(event.attacker.id());
            let mut defender_set = BitSet::new();
            defender_set.add(event.defender.id());

            let mut cooldown = None;

            for (damage, _, speed, _) in (&damages, !&cooldowns, &speeds, &attack_set).join() {
                for (mut health, _) in (&mut healths, &defender_set).join() {
                    health.value = health.value - damage.damage;
                    cooldown = Some(Cooldown::new(Duration::from_millis(
                        (1000.0 / speed.attacks_per_second) as u64,
                    )));
                }
            }

            for (mut fullness, _) in (&mut fullnesses, &attack_set).join() {
                for (health, nutrition, _) in (&healths, &nutritions, &defender_set).join() {
                    if health.value < f32::EPSILON {
                        fullness.value = fullness.value + nutrition.value;
                    }
                }
            }

            if let Some(value) = cooldown {
                cooldowns
                    .insert(event.attacker, value)
                    .expect("Unreachable: we are inserting now.");
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.event_reader = Some(
            res.fetch_mut::<EventChannel<AttackEvent>>()
                .register_reader(),
        )
    }
}

#[derive(Default)]
pub struct FindAttackSystem {
    event_reader: Option<ReaderId<CollisionEvent>>,
}

// Determine if a collision will trigger an attack. If that is the case, generate an `AttackEvent`
impl<'s> System<'s> for FindAttackSystem {
    type SystemData = (
        Read<'s, EventChannel<CollisionEvent>>,
        Write<'s, EventChannel<AttackEvent>>,
        ReadStorage<'s, combat::HasFaction<Entity>>,
        ReadStorage<'s, combat::FactionPrey<Entity>>,
    );

    fn run(
        &mut self,
        (collision_events, mut attack_events, has_faction, faction_preys): Self::SystemData,
    ) {
        let event_reader = self
            .event_reader
            .as_mut()
            .expect("`FindAttackSystem::setup` was not called before `FindAttackSystem::run`");

        for event in collision_events.read(event_reader) {
            let opt_factions = has_faction
                .get(event.entity_a)
                .and_then(|a| has_faction.get(event.entity_b).map(|b| (a, b)));

            if let Some((faction_a, faction_b)) = opt_factions {
                let preys_a = faction_preys.get(faction_a.faction);
                if let Some(preys) = preys_a {
                    if preys.is_prey(&faction_b.faction) {
                        attack_events.single_write(AttackEvent {
                            attacker: event.entity_a,
                            defender: event.entity_b,
                        });
                    }
                }

                let preys_b = faction_preys.get(faction_b.faction);
                if let Some(preys) = preys_b {
                    if preys.is_prey(&faction_a.faction) {
                        attack_events.single_write(AttackEvent {
                            attacker: event.entity_b,
                            defender: event.entity_a,
                        });
                    }
                }
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.event_reader = Some(
            res.fetch_mut::<EventChannel<CollisionEvent>>()
                .register_reader(),
        )
    }
}

//#[test]
//fn test_cooldown_is_reduced() -> Result<(), Error> {
//AmethystApplication::blank()
//.with_system(CooldownSystem, "cooldown_system", &[])
//.with_setup(|world| {
//world
//.create_entity()
//.with(Cooldown::new(Duration::from_millis(5000)))
//.build();
//})
//.with_assertion(|world| {
//let entity = world.entities().entity(0);
//let cooldowns = world.read_storage::<Cooldown>();
//let cooldown = cooldowns.get(entity).unwrap();
//assert!(cooldown.time_left.as_millis() < 5000);
//})
//.run()
//}

//#[test]
//fn test_cooldown_is_removed() -> Result<(), Error> {
//AmethystApplication::blank()
//.with_system(CooldownSystem, "cooldown_system", &[])
//.with_setup(|world| {
//world
//.create_entity()
//.with(Cooldown::new(Duration::from_millis(0)))
//.build();
//})
//.with_assertion(|world| {
//let entity = world.entities().entity(0);
//let cooldowns = world.read_storage::<Cooldown>();
//let cooldown = cooldowns.get(entity);
//assert!(cooldown.is_none());
//})
//.run()
//}
