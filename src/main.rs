use amethyst::assets::PrefabLoaderSystem;
use amethyst::{
    assets::Processor,
    audio::{AudioBundle, DjSystem},
    core::frame_limiter::FrameRateLimitStrategy,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        sprite_visibility::SpriteVisibilitySortingSystem, types::DefaultBackend, RenderingSystem,
        SpriteSheet,
    },
    ui::UiBundle,
    utils::application_root_dir,
    window::{DisplayConfig, WindowBundle},
};

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

//amethyst_inspector::inspector![
//Named,
//Transform,
//UiTransform,
//Rgba,
//Movement,
//Wander,
//Digestion,
//Fullness,
//Nutrition,
//Damage,
//Speed,
//Cooldown,
//Health,
//IntelligenceTag,
//Hidden,
//HiddenPropagate,
//];

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

    //    let pipe = Pipeline::build().with_stage(
    //        Stage::with_backbuffer()
    //            .clear_target([0.02, 0.15, 0.02, 1.0], 1.0)
    //            .with_pass(DrawFlat::<PosNormTex>::new().with_transparency(
    //                ColorMask::all(),
    //                ALPHA,
    //                Some(DepthMode::LessEqualWrite),
    //            ))
    //            .with_pass(DrawDebugLines::<PosColorNorm>::new())
    //            .with_pass(DrawUi::new())
    //            //.with_pass(amethyst_imgui::DrawUi::default()),
    //    );

    let display_config = DisplayConfig::load(display_config_path);

    // The global game data. Here we register all systems and bundles that will run for every game state. The game states
    // will define additional dispatchers for state specific systems. Note that the dispatchers will run in sequence,
    // so this setup sacrifices performance for modularity (for now).
    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        //.with(amethyst_imgui::BeginFrame::default(), "imgui_begin", &[])
        //.with_barrier()
        .with(
            PrefabLoaderSystem::<creatures::CreaturePrefabData>::default(),
            "",
            &[],
        )
        .with(
            PrefabLoaderSystem::<combat::FactionPrefabData>::default(),
            "",
            &[],
        )
        .with(
            DjSystem::new(|music: &mut Music| music.music.next()),
            "dj",
            &[],
        )
        .with_bundle(TransformBundle::new())?
        .with_bundle(AudioBundle::default())?
        //.with(
        //amethyst_inspector::InspectorHierarchy::default(),
        //"inspector_hierarchy",
        //&[],
        //)
        //.with(Inspector, "inspector", &["inspector_hierarchy"])
        //.with_barrier()
        //.with(
        //amethyst_imgui::EndFrame::default(),
        //"imgui_end",
        //&["imgui_begin"],
        //)
        .with_bundle(WindowBundle::from_config(display_config))?
        .with_bundle(UiBundle::<DefaultBackend, StringBindings>::new())?
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        .with(
            SpriteVisibilitySortingSystem::new(),
            "sprite_visibility_system",
            &["transform_system"],
        )
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            RenderGraph::default(),
        ));

    // Set up the core application with a custom state event that allows us to access input events
    // in the game states. The `CustomStateEventReader` is automatically derived based on `CustomStateEvent`.
    let mut game: Application<GameData> =
        CoreApplication::build(resources, LoadingState::default())?
            .with_frame_limit(FrameRateLimitStrategy::Sleep, 60)
            .build(game_data)?;
    game.run();
    Ok(())
}
