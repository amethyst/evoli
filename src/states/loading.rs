use crate::{
    resources::{
        audio::initialise_audio,
        prefabs::{initialize_prefabs, update_prefabs},
        wind::*,
        world_bounds::WorldBounds,
    },
    states::{main_game::MainGameState, menu::MenuState},
};
use std::env;

use crate::components::combat::load_factions;
use amethyst::{
    assets::ProgressCounter,
    prelude::*,
    renderer::debug_drawing::{DebugLines, DebugLinesParams},
};

const SKIP_MENU_ARG: &str = "no_menu";

pub struct LoadingState {
    config_path: String,
    prefab_loading_progress: Option<ProgressCounter>,
}

impl Default for LoadingState {
    fn default() -> Self {
        LoadingState {
            config_path: "".to_string(),
            prefab_loading_progress: None,
        }
    }
}

impl LoadingState {
    pub fn new(config_path: String) -> Self {
        LoadingState {
            config_path,
            prefab_loading_progress: None,
        }
    }
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        load_factions(data.world);
        self.prefab_loading_progress = Some(initialize_prefabs(&mut data.world));
        initialise_audio(data.world);
        data.world.insert(DebugLinesParams { line_width: 1.0 });

        data.world.insert(DebugLines::new());
        data.world
            .insert(WorldBounds::new(-10.0, 10.0, -10.0, 10.0));
        let wind_config_path = self.config_path.clone() + "/wind.ron";
        let wind_config = Wind::load(wind_config_path).unwrap_or_else(|error| {
            error!("Failed to load wind resource from config file. Using Wind::default() instead. Error: {:?}", error);
            Wind::default()
        });
        data.world.insert(wind_config);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);
        if let Some(ref counter) = self.prefab_loading_progress.as_ref() {
            println!(
                "Loading: {}, Failed: {}, Finished: {}",
                counter.num_loading(),
                counter.num_failed(),
                counter.num_finished()
            );
            if counter.is_complete() {
                self.prefab_loading_progress = None;
                update_prefabs(&mut data.world);
                if env::args().any(|arg| arg == SKIP_MENU_ARG) {
                    return Trans::Switch(Box::new(MainGameState::new(data.world)));
                } else {
                    return Trans::Switch(Box::new(MenuState::default()));
                }
            }
        }

        Trans::None
    }
}
