use crate::{
    components::combat::load_factions,
    resources::{
        prefabs::{initialize_prefabs, update_prefabs},
        world_bounds::WorldBounds,
    },
    states::{main_game::MainGameState, menu::MenuState},
};
use amethyst::{
    assets::ProgressCounter,
    prelude::*,
    renderer::debug_drawing::{DebugLines, DebugLinesParams},
};
use std::env;

const SKIP_MENU_ARG: &str = "no_menu";

#[derive(Default)]
pub struct LoadingState {}

impl SimpleState for LoadingState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        data.world
            .res
            .entry::<ProgressCounter>()
            .or_insert(ProgressCounter::default());
        load_factions(data.world);
        initialize_prefabs(&mut data.world);
        data.world
            .add_resource(DebugLinesParams { line_width: 2.0 });
        data.world.add_resource(DebugLines::new());
        data.world
            .add_resource(WorldBounds::new(-10.0, 10.0, -10.0, 10.0));
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);
        if data.world.read_resource::<ProgressCounter>().is_complete() {
            info!("loading complete");
            update_prefabs(&mut data.world);
            // TODO how to reset the ProgressCounter? data.world.write_resource::<Option<ProgressCounter>>().as_mut() = Some(ProgressCounter::new());
            if env::args().any(|arg| arg == SKIP_MENU_ARG) {
                return Trans::Switch(Box::new(MainGameState::new(data.world)));
            } else {
                return Trans::Switch(Box::new(MenuState::default()));
            }
        }

        Trans::None
    }
}
