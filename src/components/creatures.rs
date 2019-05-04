use amethyst::{
    assets::{
        AssetLoaderSystemData, Handle, Prefab, PrefabData, PrefabError, PrefabLoader,
        ProgressCounter, RonFormat,
    },
    core::{nalgebra::Vector3, transform::Transform, Named},
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, NullStorage, WriteStorage},
    prelude::*,
    renderer::{GraphicsPrefab, Mesh, ObjFormat, PosNormTex, PosTex, Shape, TextureFormat},
};
use amethyst_inspector::Inspect;

use serde::{Deserialize, Serialize};

use crate::{
    components::{collider::Circle, combat::CombatPrefabData, digestion::DigestionPrefabData},
    resources::prefabs::CreaturePrefabs,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum CreatureType {
    Carnivore,
    Herbivore,
    Plant,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData, Inspect)]
#[prefab(Component)]
pub struct CarnivoreTag;
impl Component for CarnivoreTag {
    type Storage = NullStorage<Self>;
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData, Inspect)]
#[prefab(Component)]
pub struct HerbivoreTag;
impl Component for HerbivoreTag {
    type Storage = NullStorage<Self>;
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData, Inspect)]
#[prefab(Component)]
pub struct PlantTag;
impl Component for PlantTag {
    type Storage = NullStorage<Self>;
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData, Inspect)]
#[prefab(Component)]
pub struct IntelligenceTag;
impl Component for IntelligenceTag {
    type Storage = NullStorage<Self>;
}

///
///
///
#[derive(Clone, smart_default::SmartDefault, Inspect, Debug, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
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
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData, Inspect)]
#[prefab(Component)]
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

#[derive(Default, Deserialize, Serialize, PrefabData)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct CreaturePrefabData {
    name: Option<Named>,
    graphics: Option<GraphicsPrefab<Vec<PosNormTex>, ObjFormat, TextureFormat>>,
    movement: Option<Movement>,
    wander: Option<Wander>,
    collider: Option<Circle>,
    digestion: Option<DigestionPrefabData>,
    combat: Option<CombatPrefabData>,

    // Tags for carnivores and herbivores
    // Should probably be reworked to avoid having too many fields here.
    carnivore_tag: Option<CarnivoreTag>,
    herbivore_tag: Option<HerbivoreTag>,
    plant_tag: Option<PlantTag>,
    intelligence_tag: Option<IntelligenceTag>,
}

pub fn initialize_prefabs(world: &mut World) {
    let mut creature_prefabs = CreaturePrefabs::default();
    let carnivore_prefab = world.exec(|loader: PrefabLoader<'_, CreaturePrefabData>| {
        loader.load("prefabs/carnivore.ron", RonFormat, (), ())
    });
    creature_prefabs.insert(CreatureType::Carnivore, carnivore_prefab);

    let herbivore_prefab = world.exec(|loader: PrefabLoader<'_, CreaturePrefabData>| {
        loader.load("prefabs/herbivore.ron", RonFormat, (), ())
    });
    creature_prefabs.insert(CreatureType::Herbivore, herbivore_prefab);

    let plant_prefab = world.exec(|loader: PrefabLoader<'_, CreaturePrefabData>| {
        loader.load("prefabs/plant.ron", RonFormat, (), ())
    });
    creature_prefabs.insert(CreatureType::Plant, plant_prefab);

    world.add_resource(creature_prefabs);
}

pub fn create_plant(
    world: &mut World,
    x: f32,
    y: f32,
    handle: &Handle<Prefab<CreaturePrefabData>>,
    _faction: Entity,
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
        .with(Circle::new(0.8))
        //        .with(Health::new(20.0))
        //        .with(combat::HasFaction::new(faction))
        .with(mesh.clone())
        .with(handle.clone())
        .with(transform)
        .build();
}
