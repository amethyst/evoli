use crate::{
    resources::{
        audio::initialise_audio,
        prefabs::{initialize_prefabs, update_prefabs},
        world_bounds::WorldBounds,
    },
    states::{main_game::MainGameState, menu::MenuState, CustomStateEvent},
};

use crate::components::combat::load_factions;
use amethyst::{
    assets::ProgressCounter,
    prelude::*,
    renderer::{DebugLines, DebugLinesParams},
};

const SKIP_MENU_ARG: &str = "no_menu";

pub struct LoadingState {
    prefab_loading_progress: Option<ProgressCounter>,
}

impl Default for LoadingState {
    fn default() -> Self {
        LoadingState {
            prefab_loading_progress: None,
        }
    }
}

impl<'a> State<GameData<'a, 'a>, CustomStateEvent> for LoadingState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'a, 'a>>) {
        load_factions(data.world);
        self.prefab_loading_progress = Some(initialize_prefabs(&mut data.world));
        initialise_audio(data.world);
        data.world.add_resource(DebugLinesParams {
            line_width: 1.0 / 20.0,
        });

        data.world
            .add_resource(DebugLines::new().with_capacity(100));
        data.world
            .add_resource(WorldBounds::new(-12.75, 12.75, -11.0, 11.0));
    }

    fn update(
        &mut self,
        mut data: StateData<'_, GameData<'a, 'a>>,
    ) -> Trans<GameData<'a, 'a>, CustomStateEvent> {
        data.data.update(&data.world);
        if let Some(ref counter) = self.prefab_loading_progress.as_ref() {
            if counter.is_complete() {
                self.prefab_loading_progress = None;
                update_prefabs(&mut data.world);
                use std::env;

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
