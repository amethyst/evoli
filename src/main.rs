#[macro_use]
extern crate log;

use amethyst::assets::PrefabLoaderSystemDesc;
use amethyst::{
    assets::Processor,
    audio::{AudioBundle, DjSystem},
    core::frame_limiter::FrameRateLimitStrategy,
    core::transform::TransformBundle,
    gltf::GltfSceneLoaderSystemDesc,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        sprite_visibility::SpriteVisibilitySortingSystem, types::DefaultBackend,
        visibility::VisibilitySortingSystem, RenderingSystem, SpriteSheet,
    },
    ui::{UiGlyphsSystemDesc},
    utils::application_root_dir,
    window::{DisplayConfig, WindowBundle},
    ui::{RenderUi, ToNativeWidget, UiBundle, UiCreator, UiTransformData, UiWidget},
};

use amethyst::renderer::plugins::{RenderPbr3D, RenderToWindow};
use amethyst::renderer::RenderingBundle;

mod components;
mod render_graph;
mod resources;
mod states;
mod systems;
mod utils;

use crate::components::{combat, creatures};
use crate::render_graph::RenderGraph;
use crate::resources::audio::Music;
use crate::states::loading::LoadingState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let resources = application_root_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
        + "/resources";
    let display_config_path = resources.clone() + "/display_config.ron";
    let key_bindings_path = resources.clone() + "/input.ron";

    let display_config = DisplayConfig::load(display_config_path)?;

    let render_display_config = DisplayConfig {
        title: "Evoli".to_string(),
        dimensions: Some((1024, 768)),
        ..Default::default()
    };

    // The global game data. Here we register all systems and bundles that will run for every game state. The game states
    // will define additional dispatchers for state specific systems. Note that the dispatchers will run in sequence,
    // so this setup sacrifices performance for modularity (for now).
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_system_desc(
            PrefabLoaderSystemDesc::<creatures::CreaturePrefabData>::default(),
            "creature_loader",
            &[],
        )
        .with_system_desc(
            GltfSceneLoaderSystemDesc::default(),
            "gltf_loader",
            &["creature_loader"],
        )
        .with_system_desc(
            PrefabLoaderSystemDesc::<combat::FactionPrefabData>::default(),
            "",
            &[],
        )
        .with(
            DjSystem::new(|music: &mut Music| music.music.next()),
            "dj",
            &[],
        )
        .with_bundle(AudioBundle::default())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config(render_display_config))
                .with_plugin(RenderPbr3D::default())
                .with_plugin(RenderUi::default())
        )?;

    // Set up the core application.
    let mut game: Application<GameData> =
        CoreApplication::build(resources.clone(), LoadingState::new(resources))?
            .with_frame_limit(FrameRateLimitStrategy::Sleep, 60)
            .build(game_data)?;
    game.run();
    Ok(())
}
