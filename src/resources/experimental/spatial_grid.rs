use amethyst::{
    core::{math::Vector2, transform::Transform},
    ecs::{BitSet, Entity},
};

use std::collections::HashMap;
use std::f32;

use crate::utils::spatial_hash::SpatialBuildHasher;

// The SpatialGrid is a spatial hashing structure used to accelerate neighbor searches for entities.
pub struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<Vector2<i32>, BitSet, SpatialBuildHasher>,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        SpatialGrid {
            cell_size,
            cells: HashMap::with_hasher(SpatialBuildHasher::default()),
        }
    }

    pub fn reset(&mut self) {
        self.cells = HashMap::with_hasher(SpatialBuildHasher::default());
    }

    // Insert an entity in the grid based on its GlobalTransform component.
    // This might have to change when upgrading Amethyst to 0.11 as the GlobalTransform component was removed.
    pub fn insert(&mut self, entity: Entity, transform: &Transform) {
        let global_matrix = transform.global_matrix();
        let x_cell = (global_matrix[(0, 3)] / self.cell_size).floor() as i32;
        let y_cell = (global_matrix[(1, 3)] / self.cell_size).floor() as i32;

        let cell_entry = self
            .cells
            .entry(Vector2::new(x_cell, y_cell))
            .or_insert(BitSet::new());
        cell_entry.add(entity.id());
    }

    // Query the entities close to a certain position.
    // The range of the query is defined by the range input.
    pub fn query(&self, transform: &Transform, range: f32) -> BitSet {
        let global_matrix = transform.global_matrix();
        let x_cell = (global_matrix[(0, 3)] / self.cell_size).floor() as i32;
        let y_cell = (global_matrix[(1, 3)] / self.cell_size).floor() as i32;
        let integer_range = (range / self.cell_size).ceil() as i32;
        let mut entities = BitSet::new();
        for x in -integer_range..(integer_range + 1) {
            for y in -integer_range..(integer_range + 1) {
                match self.cells.get(&Vector2::new(x_cell + x, y_cell + y)) {
                    Some(cell) => {
                        entities |= cell;
                    }
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
        ecs::{Builder, World, WorldExt},
    };

    #[test]
    fn grid_creation_insertion_and_query() {
        let mut world = World::new();
        let mut spatial_grid = SpatialGrid::new(1.0f32);
        let transform = Transform::default();
        spatial_grid.insert(world.create_entity().build(), &transform);
        spatial_grid.insert(world.create_entity().build(), &transform);
        spatial_grid.insert(world.create_entity().build(), &transform);

        let mut transform2 = Transform::default();
        transform2.set_translation_xyz(10.0, 10.0, 10.0);
        transform2.copy_local_to_global();
        spatial_grid.insert(world.create_entity().build(), &transform2);

        transform2.set_translation_xyz(10.5, 12.5, 10.0);
        transform2.copy_local_to_global();
        spatial_grid.insert(world.create_entity().build(), &transform2);

        assert!(
            (&spatial_grid.query(&transform2, 1.0f32))
                .into_iter()
                .count()
                == 1
        );
        assert!(
            (&spatial_grid.query(&transform, 1.0f32))
                .into_iter()
                .count()
                == 3
        );
    }
}
