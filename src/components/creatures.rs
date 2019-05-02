use amethyst::{
    assets::{AssetLoaderSystemData, Handle, Prefab},
    core::{nalgebra::Vector3, transform::Transform},
    ecs::{Component, DenseVecStorage, Entity, NullStorage},
    prelude::*,
    renderer::{Mesh, PosNormTex, PosTex, Shape},
    utils::scene::BasicScenePrefab,
};
use amethyst_inspector::Inspect;

use crate::components::collider;
use crate::components::combat;
use crate::components::digestion;
use crate::components::health::Health;

#[derive(Default, Inspect)]
pub struct CarnivoreTag;
impl Component for CarnivoreTag {
    type Storage = NullStorage<Self>;
}

#[derive(Default, Inspect)]
pub struct HerbivoreTag;
impl Component for HerbivoreTag {
    type Storage = NullStorage<Self>;
}

#[derive(Default, Inspect)]
pub struct PlantTag;
impl Component for PlantTag {
    type Storage = NullStorage<Self>;
}

#[derive(Default, Inspect)]
pub struct IntelligenceTag;
impl Component for IntelligenceTag {
    type Storage = NullStorage<Self>;
}

///
///
///
#[derive(Clone, smart_default::SmartDefault, Inspect)]
pub struct Movement {
    #[default(Vector3::zeros())]
    pub velocity: Vector3<f32>,
    pub max_movement_speed: f32,
}
impl Component for Movement {
    type Storage = DenseVecStorage<Self>;
}

///
///
///
#[derive(Default, Clone, Inspect)]
pub struct Wander {
    pub angle: f32,
    pub radius: f32,
}
impl Component for Wander {
    type Storage = DenseVecStorage<Self>;
}

impl Wander {
    pub fn new(radius: f32) -> Wander {
        Wander {
            angle: 0.0,
            radius: radius,
        }
    }

    pub fn get_direction(&self) -> Vector3<f32> {
        Vector3::new(
            self.radius * self.angle.cos(),
            self.radius * self.angle.sin(),
            0.0,
        )
    }
}

///
///
///
pub type CreaturePrefabData = BasicScenePrefab<Vec<PosNormTex>>;

// TODO: Turn this into a generic `create` function
pub fn create_carnivore(
    world: &mut World,
    x: f32,
    y: f32,
    handle: &Handle<Prefab<CreaturePrefabData>>,
    faction: Entity,
) {
    let mut transform = Transform::default();
    transform.set_xyz(x, y, 1.0);

    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(Shape::Plane(None).generate::<Vec<PosTex>>(None), ())
    });

    world
        .create_entity()
        .named("Carnivore")
        .with(CarnivoreTag)
        .with(IntelligenceTag)
        .with(Wander::new(1.0))
        .with(Movement {
            velocity: [0.0, 0.0, 0.0].into(),
            max_movement_speed: 1.75,
        })
        .with(collider::Circle::new(0.45))
        .with(digestion::Fullness::new(100.0, 100.0))
        .with(digestion::Digestion::new(1.0))
        .with(Health::new(100.0))
        .with(combat::Speed::new(1.0))
        .with(combat::Damage::new(20.0))
        .with(combat::HasFaction::new(faction))
        .with(mesh.clone())
        .with(handle.clone())
        .with(transform)
        .build();
}

pub fn create_herbivore(
    world: &mut World,
    x: f32,
    y: f32,
    handle: &Handle<Prefab<CreaturePrefabData>>,
    faction: Entity,
) {
    let mut transform = Transform::default();
    transform.set_xyz(x, y, 1.0);

    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(Shape::Plane(None).generate::<Vec<PosTex>>(None), ())
    });

    world
        .create_entity()
        .named("Herbivore")
        .with(HerbivoreTag)
        .with(IntelligenceTag)
        .with(Wander::new(1.0))
        .with(Movement {
            velocity: [0.0, 0.0, 0.0].into(),
            max_movement_speed: 2.0,
        })
        .with(collider::Circle::new(0.45))
        .with(digestion::Fullness::new(100.0, 100.0))
        .with(digestion::Digestion::new(1.0))
        .with(Health::new(100.0))
        .with(combat::Speed::new(0.5))
        .with(combat::Damage::new(20.0))
        .with(combat::HasFaction::new(faction))
        .with(mesh.clone())
        .with(handle.clone())
        .with(transform)
        .build();
}

pub fn create_plant(
    world: &mut World,
    x: f32,
    y: f32,
    handle: &Handle<Prefab<CreaturePrefabData>>,
    faction: Entity,
) {
    let mut transform = Transform::default();
    transform.set_xyz(x, y, 0.0);

    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(Shape::Plane(None).generate::<Vec<PosTex>>(None), ())
    });

    world
        .create_entity()
        .named("Plant")
        .with(PlantTag)
        .with(collider::Circle::new(0.8))
        .with(Health::new(20.0))
        .with(combat::HasFaction::new(faction))
        .with(mesh.clone())
        .with(handle.clone())
        .with(transform)
        .build();
}
