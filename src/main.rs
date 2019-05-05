#[macro_use]
extern crate amethyst_derive;

use amethyst;
use amethyst::assets::PrefabLoaderSystem;
use amethyst::{
    audio::AudioBundle,
    core::transform::{Transform, TransformBundle},
    core::Named,
    ecs::*,
    input::InputBundle,
    prelude::*,
    renderer::*,
    ui::{DrawUi, NoCustomUi, UiBundle, UiTransform},
    utils::application_root_dir,
};

mod components;
mod resources;
mod states;
mod systems;

use crate::components::combat::Health;
use crate::components::combat::{Cooldown, Damage, Speed};
use crate::components::creatures::{self, IntelligenceTag, Movement, Wander};
use crate::components::digestion::{Digestion, Fullness};
use crate::resources::audio::Music;
use crate::states::{loading::LoadingState, CustomStateEvent, CustomStateEventReader};
use crate::components::combat;

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
    Health,
    IntelligenceTag,
    Hidden,
    HiddenPropagate,
];

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
            .with_pass(DrawUi::new())
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
        .with(
            PrefabLoaderSystem::<combat::FactionPrefabData>::default(),
            "",
            &[],
        )
        .with_bundle(TransformBundle::new())?
        .with_bundle(AudioBundle::new(|music: &mut Music| music.music.next()))?
        .with(
            amethyst_inspector::InspectorHierarchy::default(),
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
        .with_bundle(RenderBundle::new(pipe, Some(display_config)))?
        .with_bundle(UiBundle::<String, String, NoCustomUi>::new())?;

    let mut game = CoreApplication::<GameData, CustomStateEvent, CustomStateEventReader>::new(
        resources,
        LoadingState::default(),
        game_data,
    )?;
    game.run();
    Ok(())
}
