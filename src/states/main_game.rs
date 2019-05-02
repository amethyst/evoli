use amethyst;
use amethyst::assets::{PrefabLoader, RonFormat};
use amethyst::{
    core::transform::Transform,
    core::Time,
    ecs::*,
    input::{is_key_down, InputEvent},
    prelude::*,
    renderer::*,
    shrev::{EventChannel, ReaderId},
};
use rand::{thread_rng, Rng};

use crate::components::combat::create_factions;
use crate::components::creatures;
use crate::resources::audio::initialise_audio;
use crate::resources::world_bounds::*;
use crate::states::paused::PausedState;
use crate::systems::*;

pub struct MainGameState {
    dispatcher: Dispatcher<'static, 'static>,
    ui_dispatcher: Dispatcher<'static, 'static>,

    input_event_reader_id: Option<ReaderId<InputEvent<String>>>,
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
                .build(),

            input_event_reader_id: None,
        }
    }
}

impl SimpleState for MainGameState {
    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Window(window_event) => {
                if is_key_down(&window_event, VirtualKeyCode::Add) {
                    let mut time_resource = data.world.write_resource::<Time>();
                    let current_time_scale = time_resource.time_scale();
                    time_resource.set_time_scale(2.0 * current_time_scale);
                }
                if is_key_down(&window_event, VirtualKeyCode::Subtract) {
                    let mut time_resource = data.world.write_resource::<Time>();
                    let current_time_scale = time_resource.time_scale();
                    time_resource.set_time_scale(0.5 * current_time_scale);
                }
            }
            _ => (),
        }

        return Trans::None;
    }

    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.dispatcher.setup(&mut data.world.res);
        self.ui_dispatcher.setup(&mut data.world.res);

        {
            let mut input_event_channel = data
                .world
                .write_resource::<EventChannel<InputEvent<String>>>();
            self.input_event_reader_id = Some(input_event_channel.register_reader());
        }

        data.world.add_resource(DebugLinesParams {
            line_width: 1.0 / 20.0,
        });

        data.world
            .add_resource(DebugLines::new().with_capacity(100));
        data.world
            .add_resource(WorldBounds::new(-12.75, 12.75, -11.0, 11.0));

        initialise_audio(data.world);
        time_control::create_time_control_ui(&mut data.world);

        let (plants, herbivores, carnivores) = create_factions(data.world);

        let carnivore_sprite =
            data.world
                .exec(|loader: PrefabLoader<'_, creatures::CreaturePrefabData>| {
                    loader.load("prefabs/carnivore.ron", RonFormat, (), ())
                });

        let herbivore_sprite =
            data.world
                .exec(|loader: PrefabLoader<'_, creatures::CreaturePrefabData>| {
                    loader.load("prefabs/herbivore.ron", RonFormat, (), ())
                });

        for i in 0..2 {
            for j in 0..2 {
                let (x, y) = (4.0 * i as f32, 4.0 * j as f32);
                creatures::create_carnivore(
                    data.world,
                    x - 5.0,
                    y - 5.0,
                    &carnivore_sprite,
                    carnivores,
                );
            }
        }

        for i in 0..2 {
            for j in 0..2 {
                let (x, y) = (4.0 * i as f32, 4.0 * j as f32);
                creatures::create_herbivore(
                    data.world,
                    x - 5.0,
                    y - 5.0,
                    &herbivore_sprite,
                    herbivores,
                );
            }
        }

        // Add some plants
        let plant_sprite =
            data.world
                .exec(|loader: PrefabLoader<'_, creatures::CreaturePrefabData>| {
                    loader.load("prefabs/plant.ron", RonFormat, (), ())
                });
        let (left, right, bottom, top) = {
            let wb = data.world.read_resource::<WorldBounds>();
            (wb.left, wb.right, wb.bottom, wb.top)
        };
        let mut rng = thread_rng();
        for _ in 0..25 {
            let x = rng.gen_range(left, right);
            let y = rng.gen_range(bottom, top);
            creatures::create_plant(data.world, x, y, &plant_sprite, plants);
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

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        self.dispatcher.dispatch(&mut data.world.res);

        let input_event_channel = data
            .world
            .read_resource::<EventChannel<InputEvent<String>>>();
        for event in input_event_channel.read(self.input_event_reader_id.as_mut().unwrap()) {
            match event {
                InputEvent::ActionPressed(action_name) => {
                    if action_name == "Pause" {
                        return Trans::Push(Box::new(PausedState::default()));
                    }
                }
                _ => (),
            }
        }
        Trans::None
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        {
            let mut input_event_channel = data
                .world
                .write_resource::<EventChannel<InputEvent<String>>>();
            self.input_event_reader_id = Some(input_event_channel.register_reader());
        }
    }

    fn shadow_update(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.ui_dispatcher.dispatch(&mut data.world.res);
    }
}
