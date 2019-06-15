use amethyst::{
    core::{Time},
    ecs::*,
    audio::{SourceHandle},
};

use crate::Music;

pub struct DayNightCycle{
    pub cycle_time: f32,
    pub elapsed_cycle_time: f32,
    pub current_cycle: CycleState,
    pub cycle_data: CycleData,
}

impl DayNightCycle {
    pub fn new() -> Self {
        DayNightCycle{
            cycle_time: 6.0, // FIXME 1 second for testing
            current_cycle : CycleState::Day,
            elapsed_cycle_time: 0.0,
            cycle_data: CycleData {
                day_music_name: "day".into(),
                night_music_name: "night".into(),
            }
        }
    }
}

#[derive(Copy,Clone,Debug)]
pub enum CycleState{
    Day,
    Night,
}

pub struct CycleData{
    pub day_music_name: String,
    pub night_music_name: String,
}

impl<'s> System<'s> for DayNightCycle{
    type SystemData = ( Read<'s, Time>, Write<'s, Music> );


    fn run(&mut self, (time, mut music): Self::SystemData) {
        self.elapsed_cycle_time += time.delta_real_seconds();
        if self.elapsed_cycle_time >= self.cycle_time {
            dbg!(&self.current_cycle);
            match self.current_cycle {
                CycleState::Day => {
                    self.current_cycle = CycleState::Night;
                    music.set_current(Some(self.cycle_data.night_music_name.clone()));

                },
                CycleState::Night => {
                    self.current_cycle = CycleState::Day;
                    music.set_current(Some(self.cycle_data.day_music_name.clone()));
                },
            }
            self.elapsed_cycle_time = 0.0;
        }
    }

}