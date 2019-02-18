struct SimulationTime {
  step_size: i32,
  dt: f32
}

impl SimulationTime {
  pub fn new(step_size: i32) {
    SimulationTime {
      step_size: step_size,
      dt: 1.0 / self.step_size as f32
    }
  }

  pub fn dt(&self) -> f32 {
    self.dt
  }
}
