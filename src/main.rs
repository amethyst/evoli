use rand::prelude::*;

use amethyst;

use amethyst::assets::{PrefabLoader, PrefabLoaderSystem, RonFormat};
use amethyst::{
    core::transform::{Transform, TransformBundle},
    core::Time,
    ecs::*,
    input::InputBundle,
    prelude::*,
    renderer::*,
    utils::application_root_dir,
};

mod components;
mod resources;
mod systems;

use crate::components::creatures;
use crate::resources::world_bounds::*;
use crate::systems::*;

struct ExampleState;
impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.add_resource(DebugLinesParams {
            line_width: 1.0 / 20.0,
        });

        data.world.register::<creatures::HerbivoreTag>();
        data.world.register::<creatures::CarnivoreTag>();

        data.world
            .add_resource(DebugLines::new().with_capacity(100));
        data.world
            .add_resource(WorldBounds::new(-12.75, 12.75, -11.0, 11.0));

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
                creatures::create_carnivore(data.world, (x - 5.0), (y - 5.0), &carnivore_sprite);
            }
        }

        for i in 0..2 {
            for j in 0..2 {
                let (x, y) = (4.0 * i as f32, 4.0 * j as f32);
                creatures::create_herbivore(data.world, (x - 5.0), (y - 5.0), &herbivore_sprite);
            }
        }

        // Add some plants
        data.world.register::<creatures::PlantTag>(); // Need to manually register component, not part of a system yet.
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
        for _ in 0..100 {
            let x = rng.gen_range(left, right);
            let y = rng.gen_range(bottom, top);
            creatures::create_plant(data.world, x, y, &plant_sprite);
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
            .with(Camera::from(Projection::perspective(
                width / height,
                std::f32::consts::FRAC_PI_2,
            )))
            .with(transform)
            .build();
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
            .with_pass(DrawDebugLines::<PosColorNorm>::new()),
    );

    let display_config = DisplayConfig::load(display_config_path);

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with(
            PrefabLoaderSystem::<creatures::CreaturePrefabData>::default(),
            "",
            &[],
        )
        .with(wander::WanderSystem, "wander_system", &[])
        .with(
            movement::MovementSystem,
            "movement_system",
            &["wander_system"],
        )
        .with(
            collision::EnforceBoundsSystem,
            "enforce_bounds_system",
            &["movement_system"],
        )
        .with(DebugSystem, "debug_system", &["enforce_bounds_system"])
        .with(digestion::DigestionSystem, "digestion_system", &[])
        .with(digestion::StarvationSystem, "starvation_system", &["digestion_system"])
        .with(digestion::DebugFullnessSystem, "debug_fullness_system", &["digestion_system"])
        .with_bundle(TransformBundle::new().with_dep(&["enforce_bounds_system"]))?
        .with_bundle(RenderBundle::new(pipe, Some(display_config)))?;

    let mut game = Application::new(resources, ExampleState, game_data)?;
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
