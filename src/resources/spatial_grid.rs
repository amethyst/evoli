use amethyst::{
    core::{nalgebra::Vector4, transform::GlobalTransform},
    ecs::Entity,
};

use std::collections::HashMap;
use std::f32;

pub struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<i32, HashMap<i32, Vec<Entity>>>,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        SpatialGrid {
            cell_size,
            cells: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.cells = HashMap::new();
    }

    pub fn insert(&mut self, entity: Entity, transform: &GlobalTransform) {
        let pos = Vector4::from(transform.as_ref()[3]);
        let x_cell = (pos[0] / self.cell_size).floor() as i32;
        let y_cell = (pos[1] / self.cell_size).floor() as i32;
        let row_entry = self.cells.entry(x_cell).or_insert(HashMap::new());
        let col_entry = row_entry.entry(y_cell).or_insert(Vec::new());
        col_entry.push(entity);
    }

    pub fn query(&self, transform: &GlobalTransform, range: f32) -> Vec<Entity> {
        let pos = Vector4::from(transform.as_ref()[3]);
        let x_cell = (pos[0] / self.cell_size).floor() as i32;
        let y_cell = (pos[1] / self.cell_size).floor() as i32;

        let integer_range = 1 + (range / self.cell_size).ceil() as i32;
        //        let sq_range = range * range;

        let mut entities = Vec::new();
        for x in -integer_range..integer_range {
            for y in -integer_range..integer_range {
                //                if (x * x + y * y) as f32 > sq_range {
                //                    continue;
                //                }
                match self.cells.get(&(x_cell + x)) {
                    Some(col) => match col.get(&(y_cell + y)) {
                        Some(cell) => entities.extend_from_slice(cell.as_slice()),
                        None => (),
                    },
                    None => (),
                }
            }
        }
        entities
    }
}
