use amethyst;

use amethyst::{
    core::math::{clamp, Rotation3, Vector3},
    core::{transform::Transform, ArcThreadPool, Time},
    ecs::*,
    input::InputEvent,
    prelude::*,
    renderer::{
        camera::{Camera, Projection},
        palette::rgb::Srgba,
        resources::Tint,
    },
    shrev::EventChannel,
    window::ScreenDimensions,
};

use crate::systems::behaviors::decision::{
    ClosestSystem, Predator, Prey, QueryPredatorsAndPreySystem, SeekSystem,
};
use crate::systems::behaviors::obstacle::{ClosestObstacleSystem, Obstacle};
use crate::{
    resources::{
        debug::DebugConfig, prefabs::UiPrefabRegistry, spatial_grid::SpatialGrid,
        world_bounds::WorldBounds,
    },
    states::menu::MenuState,
    systems::*,
};
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

const TIME_SCALE_FACTOR: f32 = 2.0;
const TIME_SCALE_RANGE: (f32, f32) = (1.0 / 4.0, 1.0 * 4.0);

pub struct MainGameState {
    dispatcher: Dispatcher<'static, 'static>,
    debug_dispatcher: Dispatcher<'static, 'static>,
    ui_dispatcher: Dispatcher<'static, 'static>,
    ui: Option<Entity>,
    camera: Option<Entity>,
    paused: bool,
    desired_time_scale: f32,
}

impl MainGameState {
    pub fn new(world: &mut World) -> Self {
        // For profiling, the dispatcher needs to specify the pool that is created for us by `ApplicationBuilder::new`.
        // This thread pool will include the necessary setup for `profile_scope`.
        let pool = world.read_resource::<ArcThreadPool>().clone();
        MainGameState {
            dispatcher: DispatcherBuilder::new()
                .with_pool(pool)
                .with(
                    camera_movement::CameraMovementSystem::default(),
                    "camera_movement",
                    &[],
                )
                .with(perception::SpatialGridSystem, "spatial_grid", &[])
                .with(
                    perception::EntityDetectionSystem,
                    "entity_detection",
                    &["spatial_grid"],
                )
                .with(
                    QueryPredatorsAndPreySystem,
                    "query_predators_and_prey_system",
                    &[],
                )
                .with(ClosestObstacleSystem, "closest_obstacle_system", &[])
                .with(
                    ClosestSystem::<Prey>::default(),
                    "closest_prey_system",
                    &["query_predators_and_prey_system"],
                )
                .with(
                    ClosestSystem::<Predator>::default(),
                    "closest_predator_system",
                    &["query_predators_and_prey_system"],
                )
                .with(
                    SeekSystem::<Prey>::new(
                        Rotation3::from_axis_angle(&Vector3::z_axis(), 0.0),
                        1.0,
                    ),
                    "seek_prey_system",
                    &["closest_prey_system"],
                )
                .with(
                    SeekSystem::<Predator>::new(
                        // 180 degrees, run away!
                        Rotation3::from_axis_angle(&Vector3::z_axis(), std::f32::consts::PI),
                        1.0,
                    ),
                    "avoid_predator_system",
                    &["closest_predator_system"],
                )
                .with(
                    SeekSystem::<Obstacle>::new(
                        // 120 degrees. A little more than perpendicular so the creature
                        // tries to steer away from the wall rather than just follow it.
                        Rotation3::from_axis_angle(
                            &Vector3::z_axis(),
                            2f32 * std::f32::consts::FRAC_PI_3,
                        ),
                        5.0,
                    ),
                    "avoid_obstacle_system",
                    &["closest_obstacle_system"],
                )
                .with(behaviors::ricochet::RicochetSystem, "ricochet_system", &[])
                .with(
                    behaviors::wander::WanderSystem,
                    "wander_system",
                    &[
                        "seek_prey_system",
                        "avoid_predator_system",
                        "avoid_obstacle_system",
                        "ricochet_system",
                    ],
                )
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
                .with(digestion::DigestionSystem, "digestion_system", &[])
                .with(
                    digestion::StarvationSystem,
                    "starvation_system",
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
                .with(
                    health::DebugHealthSystem::default(),
                    "debug_health_system",
                    &[],
                )
                .with(
                    spawner::DebugSpawnTriggerSystem::default(),
                    "debug_spawn_trigger",
                    &[],
                )
                .with(
                    swarm_behavior::SwarmSpawnSystem::default(),
                    "swarm_spawn",
                    &[],
                )
                .with(
                    swarm_behavior::SwarmBehaviorSystem::default(),
                    "swarm_behavior",
                    &[],
                )
                .with(
                    swarm_behavior::SwarmCenterSystem::default(),
                    "swarm_center",
                    &[],
                )
                .with(
                    spawner::CreatureSpawnerSystem::default(),
                    "creature_spawner",
                    &["debug_spawn_trigger", "swarm_spawn"],
                )
                .build(),
            debug_dispatcher: DispatcherBuilder::new()
                .with(
                    collision::DebugCollisionEventSystem::default(),
                    "debug_collision_event_system",
                    &[],
                )
                .with(collision::DebugColliderSystem, "debug_collider_system", &[])
                .with(debug::DebugSystem, "debug_system", &[])
                .with(digestion::DebugFullnessSystem, "debug_fullness_system", &[])
                .with(
                    perception::DebugEntityDetectionSystem,
                    "debug_entity_detection",
                    &[],
                )
                .build(),
            ui_dispatcher: DispatcherBuilder::new()
                .with(
                    main_game_ui::MainGameUiSystem::default(),
                    "main_game_ui",
                    &[],
                )
                .build(),
            ui: None,
            camera: None,
            paused: false,
            desired_time_scale: 1.0,
        }
    }

    // push desired_time_scale into effect
    fn update_time_scale(&self, world: &mut World) {
        world
            .write_resource::<Time>()
            .set_time_scale(if self.paused {
                0.0
            } else {
                self.desired_time_scale
            });
    }

    fn handle_action(&mut self, action: &str, world: &mut World) -> SimpleTrans {
        if action == "ToggleDebug" {
            let mut debug_config = world.write_resource::<DebugConfig>();
            debug_config.visible = !debug_config.visible;
            Trans::None
        } else if action == main_game_ui::PAUSE_BUTTON.action {
            self.paused = !self.paused;
            self.update_time_scale(world);
            Trans::None
        } else if action == main_game_ui::SPEED_UP_BUTTON.action {
            self.desired_time_scale = clamp(
                self.desired_time_scale * TIME_SCALE_FACTOR,
                TIME_SCALE_RANGE.0,
                TIME_SCALE_RANGE.1,
            );
            self.update_time_scale(world);
            Trans::None
        } else if action == main_game_ui::SLOW_DOWN_BUTTON.action {
            self.desired_time_scale = clamp(
                self.desired_time_scale / TIME_SCALE_FACTOR,
                TIME_SCALE_RANGE.0,
                TIME_SCALE_RANGE.1,
            );
            self.update_time_scale(world);
            Trans::None
        } else if action == main_game_ui::MENU_BUTTON.action {
            Trans::Switch(Box::new(MenuState::default()))
        } else {
            Trans::None
        }
    }
}

impl SimpleState for MainGameState {
    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Window(_) => Trans::None, // Events related to the window and inputs.
            StateEvent::Ui(_) => Trans::None,     // Ui event. Button presses, mouse hover, etc...
            StateEvent::Input(input_event) => {
                if let InputEvent::ActionPressed(action) = input_event {
                    self.handle_action(&action, data.world)
                } else {
                    Trans::None
                }
            }
        }
    }

    fn on_start(&mut self, data: StateData<GameData>) {
        info!("start main game");

        self.dispatcher.setup(&mut data.world.res);
        self.debug_dispatcher.setup(&mut data.world.res);
        self.ui_dispatcher.setup(&mut data.world.res);

        // Setup debug config resource
        data.world.add_resource(DebugConfig::default());
        data.world.add_resource(SpatialGrid::new(1.0f32));

        // main game ui
        let ui_prefab = data
            .world
            .read_resource::<UiPrefabRegistry>()
            .find(data.world, "main game");
        if let Some(ui_prefab) = ui_prefab {
            self.ui = Some(data.world.create_entity().with(ui_prefab).build());
        }

        data.world.register::<spawner::CreatureTag>();

        // Add some plants
        let (left, right, bottom, top) = {
            let wb = data.world.read_resource::<WorldBounds>();
            (wb.left, wb.right, wb.bottom, wb.top)
        };
        {
            let mut rng = thread_rng();
            for _ in 0..25 {
                let x = rng.gen_range(left, right);
                let y = rng.gen_range(bottom, top);
                let scale = rng.gen_range(0.8f32, 1.2f32);
                let rotation = rng.gen_range(0.0f32, PI);
                let mut transform = Transform::default();
                transform.set_translation_xyz(x, y, 0.01);
                transform.set_scale(Vector3::new(scale, scale, 1.0));
                transform.set_rotation_euler(0.0, 0.0, rotation);
                let plant_entity = data.world.create_entity().with(transform).build();
                let mut spawn_events = data
                    .world
                    .write_resource::<EventChannel<spawner::CreatureSpawnEvent>>();
                // TODO unfortunate naming here; plants are not creatures...OrganismSpawnEvent or just SpawnEvent?
                spawn_events.single_write(spawner::CreatureSpawnEvent {
                    creature_type: "Plant".to_string(),
                    entity: plant_entity,
                });
            }
        }
        //insert single nushi
        {
            let mut rng = thread_rng();
            let x = rng.gen_range(left, right);
            let y = rng.gen_range(bottom, top);
            let scale = 0.4f32;

            let mut transform = Transform::default();
            transform.set_translation_xyz(x, y, 0.02);
            transform.set_scale(Vector3::new(scale, scale, 1.0));

            let nushi_entity = data.world.create_entity().with(transform).build();
            let mut spawn_events = data
                .world
                .write_resource::<EventChannel<spawner::CreatureSpawnEvent>>();
            spawn_events.single_write(spawner::CreatureSpawnEvent {
                creature_type: "Nushi".to_string(),
                entity: nushi_entity,
            });
        }

        {
            let scale = 17.0f32;
            let mut transform = Transform::default();
            transform.set_scale(Vector3::new(scale, scale, 1.0));

            let tint = Tint(Srgba::new(0.5f32, 0.5f32, 0.5f32, 0.5f32));

            let ground_entity = data
                .world
                .create_entity()
                .with(transform)
                .with(tint)
                .build();
            let mut spawn_events = data
                .world
                .write_resource::<EventChannel<spawner::CreatureSpawnEvent>>();
            spawn_events.single_write(spawner::CreatureSpawnEvent {
                creature_type: "Ground".to_string(),
                entity: ground_entity,
            });
        }
        // Setup camera
        let (width, height) = {
            let dim = data.world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, 0.0, 12.0);

        self.camera = Some(
            data.world
                .create_entity()
                .named("Main camera")
                .with(Camera::from(Projection::perspective(
                    width / height,
                    std::f32::consts::FRAC_PI_2,
                    0.01f32,
                    1000.0f32,
                )))
                .with(transform)
                .build(),
        );

        // initialize time scale
        self.paused = false;
        data.world.write_resource::<Time>().set_time_scale(1.0);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        info!("stop main game");

        if let Some(ui) = self.ui {
            if data.world.delete_entity(ui).is_ok() {
                self.ui = None;
            }
        }
        if let Some(camera) = self.camera {
            if data.world.delete_entity(camera).is_ok() {
                self.camera = None;
            }
        }

        // delete all organisms (e.g. creatures, plants, etc.)
        let mut organisms: Vec<Entity> = Vec::new();
        for (entity, _) in (
            &data.world.entities(),
            &data.world.read_storage::<spawner::CreatureTag>(),
        )
            .join()
        {
            organisms.push(entity);
        }
        data.world
            .delete_entities(&organisms)
            .expect("failed to delete all organisms");

        // fix up time scale before we leave this state
        data.world.write_resource::<Time>().set_time_scale(1.0);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        self.dispatcher.dispatch(&data.world.res);

        let show_debug = {
            let debug_config = data.world.read_resource::<DebugConfig>();
            debug_config.visible
        };

        if show_debug {
            self.debug_dispatcher.dispatch(&data.world.res);
        }

        data.data.update(&data.world);

        self.ui_dispatcher.dispatch(&data.world.res);

        Trans::None
    }
}
