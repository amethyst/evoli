use rand::prelude::*;

use amethyst;

use amethyst::assets::{PrefabLoader, PrefabLoaderSystem, RonFormat};
use amethyst::{
    core::transform::{Transform, TransformBundle},
    core::Named,
    core::Time,
    ecs::*,
    input::{is_key_down, InputBundle},
    prelude::*,
    renderer::*,
    ui::UiTransform,
    utils::application_root_dir,
};

mod components;
mod resources;
mod states;
mod systems;

use crate::components::combat::{
    create_factions, Cooldown, Damage, FactionEnemies, HasFaction, Speed,
};
use crate::components::creatures::{
    self, CarnivoreTag, HerbivoreTag, IntelligenceTag, Movement, PlantTag, Wander,
};
use crate::components::digestion::{Digestion, Fullness};
use crate::components::health::Health;
use crate::resources::world_bounds::*;
use crate::states::paused::PausedState;
use crate::systems::collision::DebugCollisionEventSystem;
use crate::systems::*;

amethyst_inspector::inspector![
    Named,
    Transform,
    UiTransform,
    Rgba,
    Movement,
    Wander,
    Digestion,
    Fullness,
    Damage,
    Speed,
    Cooldown,
    HasFaction,
    Health,
    CarnivoreTag,
    HerbivoreTag,
    PlantTag,
    IntelligenceTag,
    Hidden,
    HiddenPropagate,
];

struct ExampleState {
    dispatcher: Dispatcher<'static, 'static>,
}

impl Default for ExampleState {
    fn default() -> Self {
        ExampleState {
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
                    DebugCollisionEventSystem::default(),
                    "debug_collision_event_system",
                    &["collision_system"],
                )
                .with(collision::DebugColliderSystem, "debug_collider_system", &[])
                .with(
                    DebugSystem,
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
        }
    }
}

impl SimpleState for ExampleState {
    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::P) {
                return Trans::Push(Box::new(PausedState));
            }
            if is_key_down(&event, VirtualKeyCode::Add) {
                let mut time_resource = data.world.write_resource::<Time>();
                let current_time_scale = time_resource.time_scale();
                time_resource.set_time_scale(2.0 * current_time_scale);
            }
            if is_key_down(&event, VirtualKeyCode::Subtract) {
                let mut time_resource = data.world.write_resource::<Time>();
                let current_time_scale = time_resource.time_scale();
                time_resource.set_time_scale(0.5 * current_time_scale);
            }
        }

        return Trans::None;
    }

    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.dispatcher.setup(&mut data.world.res);

        data.world.add_resource(DebugLinesParams {
            line_width: 1.0 / 20.0,
        });

        data.world
            .add_resource(DebugLines::new().with_capacity(100));
        data.world
            .add_resource(WorldBounds::new(-12.75, 12.75, -11.0, 11.0));

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
                    (x - 5.0),
                    (y - 5.0),
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
                    (x - 5.0),
                    (y - 5.0),
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
        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let resources = application_root_dir().clone() + "/resources";
    let display_config_path = resources.clone() + "/display_config.ron";
    let key_bindings_path = resources.clone() + "/input.ron";

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.002, 0.01, 0.01, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                None,
            ))
            .with_pass(DrawDebugLines::<PosColorNorm>::new())
            .with_pass(amethyst_imgui::DrawUi::default()),
    );

    let display_config = DisplayConfig::load(display_config_path);

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with(amethyst_imgui::BeginFrame::default(), "imgui_begin", &[])
        .with_barrier()
        .with(
            PrefabLoaderSystem::<creatures::CreaturePrefabData>::default(),
            "",
            &[],
        )
        .with_bundle(TransformBundle::new())?
        .with(
            amethyst_inspector::InspectorHierarchy,
            "inspector_hierarchy",
            &[],
        )
        .with(Inspector, "inspector", &["inspector_hierarchy"])
        .with_barrier()
        .with(
            amethyst_imgui::EndFrame::default(),
            "imgui_end",
            &["imgui_begin"],
        )
        .with_bundle(RenderBundle::new(pipe, Some(display_config)))?;

    let mut game = Application::new(resources, ExampleState::default(), game_data)?;
    game.run();
    Ok(())
}

struct DebugSystem;
impl<'s> System<'s> for DebugSystem {
    type SystemData = (Write<'s, DebugLines>, Write<'s, WorldBounds>);

    fn run(&mut self, (mut debug_lines_resource, bounds): Self::SystemData) {
        let color = [0.8, 0.04, 0.6, 1.0];
        debug_lines_resource.draw_line(
            [bounds.left, bounds.bottom, 0.0].into(),
            [bounds.right, bounds.bottom, 0.0].into(),
            color.into(),
        );

        debug_lines_resource.draw_line(
            [bounds.left, bounds.top, 0.0].into(),
            [bounds.right, bounds.top, 0.0].into(),
            color.into(),
        );

        debug_lines_resource.draw_line(
            [bounds.left, bounds.bottom, 0.0].into(),
            [bounds.left, bounds.top, 0.0].into(),
            color.into(),
        );

        debug_lines_resource.draw_line(
            [bounds.right, bounds.bottom, 0.0].into(),
            [bounds.right, bounds.top, 0.0].into(),
            color.into(),
        );

        debug_lines_resource.draw_line(
            [0.0, 0.0, 0.0].into(),
            [1.0, 0.0, 0.0].into(),
            [1.0, 0.0, 0.0, 1.0].into(),
        );
        debug_lines_resource.draw_line(
            [0.0, 0.0, 0.0].into(),
            [0.0, 1.0, 0.0].into(),
            [0.0, 1.0, 0.0, 1.0].into(),
        );
        debug_lines_resource.draw_line(
            [0.0, 0.0, 0.0].into(),
            [0.0, 0.0, 1.0].into(),
            [0.0, 0.0, 1.0, 1.0].into(),
        );
    }
}
