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
    pub max_movement_speed: f32,
}
impl Component for Movement {
    type Storage = DenseVecStorage<Self>;
}

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
            max_movement_speed: 5.0,
        })
        .with(Wander::new(1.0))
        .with(WanderBehaviorTag)
        .with(mesh.clone())
        .with(handle.clone())
        .with(local_transform)
        .build();
}
