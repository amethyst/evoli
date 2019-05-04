use amethyst;

use amethyst::{
    core::{transform::Transform, Time},
    ecs::*,
    input::InputEvent,
    prelude::*,
    renderer::*,
    State,
    shrev::EventChannel,
};
use rand::{thread_rng, Rng};

use crate::{
    components::{creatures, creatures::CreatureType},
    resources::{audio::initialise_audio, prefabs::{
        CreaturePrefabs,
        initialize_prefabs,
    }, world_bounds::WorldBounds},
    states::{
        paused::PausedState,
        CustomStateEvent
    },
    systems::*,
};
use std::f32::consts::PI;

pub struct MainGameState {
    dispatcher: Dispatcher<'static, 'static>,
    ui_dispatcher: Dispatcher<'static, 'static>,
}

impl Default for MainGameState {
    fn default() -> Self {
        MainGameState {
            dispatcher: DispatcherBuilder::new()
                .with(decision::DecisionSystem, "decision_system", &[])
                .with(wander::WanderSystem, "wander_system", &["decision_system"])
                .with(
                    movement::MovementSystem,
                    "movement_system",
                    &["wander_system"],
                )
                .with(
                    collision::CollisionSystem,
                    "collision_system",
                    &["movement_system"],
                )
                .with(
                    collision::EnforceBoundsSystem,
                    "enforce_bounds_system",
                    &["movement_system"],
                )
                .with(
                    collision::DebugCollisionEventSystem::default(),
                    "debug_collision_event_system",
                    &["collision_system"],
                )
                .with(collision::DebugColliderSystem, "debug_collider_system", &[])
                .with(
                    debug::DebugSystem,
                    "debug_system",
                    &["collision_system", "enforce_bounds_system"],
                )
                .with(digestion::DigestionSystem, "digestion_system", &[])
                .with(
                    digestion::StarvationSystem,
                    "starvation_system",
                    &["digestion_system"],
                )
                .with(
                    digestion::DebugFullnessSystem,
                    "debug_fullness_system",
                    &["digestion_system"],
                )
                .with(combat::CooldownSystem, "cooldown_system", &[])
                .with(
                    combat::FindAttackSystem::default(),
                    "find_attack_system",
                    &["cooldown_system"],
                )
                .with(
                    combat::PerformDefaultAttackSystem::default(),
                    "perform_default_attack_system",
                    &["find_attack_system"],
                )
                .with(
                    health::DeathByHealthSystem,
                    "death_by_health_system",
                    &["perform_default_attack_system"],
                )
                .with(health::DebugHealthSystem, "debug_health_system", &[])
                .build(),
            ui_dispatcher: DispatcherBuilder::new()
                .with(
                    time_control::TimeControlSystem::default(),
                    "time_control",
                    &[],
                )
                .with(
                    spawner::CreatureSpawnerSystem::default(),
                    "creature_spawner",
                    &[],
                )
                .with(
                    spawner::DebugSpawnTriggerSystem::default(),
                    "debug_spawn_trigger",
                    &[],
                )
                .build(),
        }
    }
}

impl<'a> State<GameData<'a, 'a>, CustomStateEvent> for MainGameState {
    fn handle_event(
        &mut self,
        data: StateData<GameData<'a, 'a>>,
        event: CustomStateEvent,
    ) -> Trans<GameData<'a, 'a>, CustomStateEvent> {
        match event {
            CustomStateEvent::Window(_) => (), // Events related to the window and inputs.
            CustomStateEvent::Ui(_) => (),     // Ui event. Button presses, mouse hover, etc...
            CustomStateEvent::Input(input_event) => match input_event {
                InputEvent::ActionPressed(action_name) => match action_name.as_ref() {
                    "TogglePause" => return Trans::Push(Box::new(PausedState::default())),
                    "SpeedUp" => {
                        let mut time_resource = data.world.write_resource::<Time>();
                        let current_time_scale = time_resource.time_scale();
                        time_resource.set_time_scale(2.0 * current_time_scale);
                    }
                    "SlowDown" => {
                        let mut time_resource = data.world.write_resource::<Time>();
                        let current_time_scale = time_resource.time_scale();
                        time_resource.set_time_scale(0.5 * current_time_scale);
                    }
                    _ => (),
                },
                _ => (),
            },
        };
        Trans::None
    }

    fn on_start(&mut self, mut data: StateData<'_, GameData<'a, 'a>>) {
        self.dispatcher.setup(&mut data.world.res);
        self.ui_dispatcher.setup(&mut data.world.res);

        data.world.add_resource(DebugLinesParams {
            line_width: 1.0 / 20.0,
        });

        data.world
            .add_resource(DebugLines::new().with_capacity(100));
        data.world
            .add_resource(WorldBounds::new(-12.75, 12.75, -11.0, 11.0));

        initialise_audio(data.world);
        time_control::create_time_control_ui(&mut data.world);
        initialize_prefabs(&mut data.world);

        // Add some plants
        let (left, right, bottom, top) = {
            let wb = data.world.read_resource::<WorldBounds>();
            (wb.left, wb.right, wb.bottom, wb.top)
        };

        {
            let mut spawn_events = data
                .world
                .write_resource::<EventChannel<spawner::CreatureSpawnEvent>>();
            let mut rng = thread_rng();
            for _ in 0..25 {
                let x = rng.gen_range(left, right);
                let y = rng.gen_range(bottom, top);
                let scale = rng.gen_range(0.8f32, 1.2f32);
                let rotation = rng.gen_range(0.0f32, PI);
                let mut transform = Transform::default();
                transform.set_xyz(x, y, 0.0);
                transform.set_scale(scale, scale, 1.0);
                transform.set_rotation_euler(0.0, 0.0, rotation);
                spawn_events.single_write(spawner::CreatureSpawnEvent {
                    creature_type: CreatureType::Plant,
                    transform,
                });
            }
        }

        // Setup camera
        let (width, height) = {
            let dim = data.world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let mut transform = Transform::default();
        transform.set_position([0.0, 0.0, 12.0].into());

        data.world
            .create_entity()
            .named("Main camera")
            .with(Camera::from(Projection::perspective(
                width / height,
                std::f32::consts::FRAC_PI_2,
            )))
            .with(transform)
            .build();
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'a, 'a>>,
    ) -> Trans<GameData<'a, 'a>, CustomStateEvent> {
        self.dispatcher.dispatch(&mut data.world.res);
        data.data.update(&data.world);
        Trans::None
    }

    fn shadow_update(&mut self, data: StateData<'_, GameData<'a, 'a>>) {
        self.ui_dispatcher.dispatch(&mut data.world.res);
    }
}
