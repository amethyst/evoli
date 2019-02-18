use amethyst::{
    assets::{AssetLoaderSystemData, Handle, Prefab},
    core::{nalgebra::Vector3, transform::Transform},
    ecs::{Component, DenseVecStorage, NullStorage},
    prelude::*,
    renderer::{Mesh, PosNormTex, PosTex, Shape},
    utils::scene::BasicScenePrefab,
};

#[derive(Default)]
pub struct CarnivoreTag;
#[derive(Default)]
pub struct HerbivoreTag;
#[derive(Default)]
pub struct PlantTag;

impl Component for CarnivoreTag {
    type Storage = NullStorage<Self>;
}
impl Component for HerbivoreTag {
    type Storage = NullStorage<Self>;
}
impl Component for PlantTag {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct WanderBehaviorTag;
#[derive(Default)]
pub struct EvadeBehaviorTag;
#[derive(Default)]
pub struct PursueBehaviorTag;

impl Component for WanderBehaviorTag {
    type Storage = NullStorage<Self>;
}
impl Component for EvadeBehaviorTag {
    type Storage = NullStorage<Self>;
}
impl Component for PursueBehaviorTag {
    type Storage = NullStorage<Self>;
}

pub struct Movement {
    pub velocity: Vector3<f32>,
    pub wandering_speed: f32,
    pub max_movement_speed: f32,
}

impl Component for Movement {
    type Storage = DenseVecStorage<Self>;
}

pub type CarnivorePrefabData = BasicScenePrefab<Vec<PosNormTex>>;

pub fn create_carnivore(
    world: &mut World,
    x: f32,
    y: f32,
    handle: &Handle<Prefab<CarnivorePrefabData>>,
) {
    let mut local_transform = Transform::default();
    local_transform.set_xyz(x, y, 0.0);

    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(Shape::Plane(None).generate::<Vec<PosTex>>(None), ())
    });

    world
        .create_entity()
        .with(Movement {
            velocity: [x, y, 0.0].into(),
            wandering_speed: 5.0,
            max_movement_speed: 10.0,
        })
        .with(WanderBehaviorTag)
        .with(mesh.clone())
        .with(handle.clone())
        .with(local_transform)
        .build();
}
