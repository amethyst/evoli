use amethyst::{core::transform::Transform, ecs::Entity};

use std::collections::HashMap;
use std::f32;

// The SpatialGrid is a spatial hashing structure used to accelerate neighbor searches for entities.
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

    // Insert an entity in the grid based on its GlobalTransform component.
    // This might have to change when upgrading Amethyst to 0.11 as the GlobalTransform component was removed.
    pub fn insert(&mut self, entity: Entity, transform: &Transform) {
        let global_matrix = transform.global_matrix();
        let x_cell = (global_matrix[(3, 0)] / self.cell_size).floor() as i32;
        let y_cell = (global_matrix[(3, 1)] / self.cell_size).floor() as i32;
        let row_entry = self.cells.entry(x_cell).or_insert(HashMap::new());
        let col_entry = row_entry.entry(y_cell).or_insert(Vec::new());
        col_entry.push(entity);
    }

    // Query the entities close to a certain position.
    // The range of the query is defined by the range input.
    pub fn query(&self, transform: &Transform, range: f32) -> Vec<Entity> {
        let global_matrix = transform.global_matrix();
        let x_cell = (global_matrix[(3, 0)] / self.cell_size).floor() as i32;
        let y_cell = (global_matrix[(3, 1)] / self.cell_size).floor() as i32;
        let integer_range = 1 + (range / self.cell_size).ceil() as i32;
        let mut entities = Vec::new();
        for x in -integer_range..integer_range {
            for y in -integer_range..integer_range {
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

#[cfg(test)]
mod tests {
    use super::*;
    use amethyst::{
        core::transform::Transform,
        ecs::{Builder, World},
    };

    #[test]
    fn grid_creation_insertion_and_query() {
        let mut world = World::new();
        let mut spatial_grid = SpatialGrid::new(1.0f32);
        let transform = Transform::default();
        spatial_grid.insert(world.create_entity().build(), &transform);
        assert!(spatial_grid.query(&transform, 1.0f32).len() == 1);
    }
}
