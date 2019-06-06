#[derive(Default)]
pub struct WorldBounds {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

impl WorldBounds {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32) -> WorldBounds {
        WorldBounds {
            left: left,
            right: right,
            bottom: bottom,
            top: top,
        }
    }
}


