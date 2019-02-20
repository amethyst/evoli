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

        data.world
            .add_resource(DebugLines::new().with_capacity(100));
        data.world
            .add_resource(WorldBounds::new(-12.75, 12.75, -11.0, 11.0));

        let carnivorous_sprite =
            data.world
                .exec(|loader: PrefabLoader<'_, creatures::CreaturePrefabData>| {
                    loader.load("prefabs/carnivorous.ron", RonFormat, (), ())
                });

        for i in 0..10 {
            for j in 0..10 {
                let (x, y) = (i as f32, j as f32);
                creatures::create_carnivore(
                    data.world,
                    (x - 5.0) / 10.0,
                    (y - 5.0) / 10.0,
                    &carnivorous_sprite,
                );
            }
        }

        // Setup camera
        let (width, height) = {
            let dim = data.world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let mut local_transform = Transform::default();
        local_transform.set_position([0.0, 0.0, 12.0].into());

        data.world
            .create_entity()
            .with(Camera::from(Projection::perspective(
                width / height,
                std::f32::consts::FRAC_PI_2,
            )))
            .with(local_transform)
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
        .with_bundle(TransformBundle::new().with_dep(&["enforce_bounds_system"]))?
        .with_bundle(RenderBundle::new(pipe, Some(display_config)))?;

    let mut game = Application::new(resources, ExampleState, game_data)?;
    game.run();
    Ok(())
}

struct DebugSystem;
impl<'s> System<'s> for DebugSystem {
    type SystemData = (
        Write<'s, DebugLines>,
        Write<'s, WorldBounds>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut debug_lines_resource, mut bounds, time): Self::SystemData) {
        let tcos = (time.absolute_time_seconds() as f32).cos() * 0.02;

        bounds.top += tcos;
        bounds.bottom -= tcos;
        bounds.left -= tcos;
        bounds.right += tcos;

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
