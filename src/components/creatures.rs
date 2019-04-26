use amethyst::{
    assets::{AssetLoaderSystemData, Handle, Prefab},
    core::{nalgebra::Vector3, transform::Transform},
    ecs::{Component, DenseVecStorage, LazyUpdate, NullStorage, Read, ReadStorage, WriteStorage, Entity},
    prelude::*,
    renderer::{Mesh, PosNormTex, PosTex, Shape},
    utils::scene::BasicScenePrefab,
};

use crate::components::collider;
use crate::components::combat;
use crate::components::digestion;
use crate::components::health::Health;
use amethyst_imgui::imgui;

#[derive(Default)]
pub struct CarnivoreTag;
impl Component for CarnivoreTag {
    type Storage = NullStorage<Self>;
}
amethyst_inspector::inspect_marker!(CarnivoreTag);

#[derive(Default)]
pub struct HerbivoreTag;
impl Component for HerbivoreTag {
    type Storage = NullStorage<Self>;
}
amethyst_inspector::inspect_marker!(HerbivoreTag);

#[derive(Default)]
pub struct PlantTag;
impl Component for PlantTag {
    type Storage = NullStorage<Self>;
}
amethyst_inspector::inspect_marker!(PlantTag);

#[derive(Default)]
pub struct IntelligenceTag;
impl Component for IntelligenceTag {
    type Storage = NullStorage<Self>;
}
amethyst_inspector::inspect_marker!(IntelligenceTag);

///
///
///
pub struct Movement {
    pub velocity: Vector3<f32>,
    pub max_movement_speed: f32,
}
impl Component for Movement {
    type Storage = DenseVecStorage<Self>;
}
impl<'a> amethyst_inspector::Inspect<'a> for Movement {
    type SystemData = (ReadStorage<'a, Self>, Read<'a, LazyUpdate>);
    const CAN_ADD: bool = true;

    fn inspect(
        (storage, lazy): &Self::SystemData,
        entity: amethyst::ecs::Entity,
        ui: &imgui::Ui<'_>,
    ) {
        let &Movement {
            velocity,
            mut max_movement_speed,
        } = if let Some(x) = storage.get(entity) {
            x
        } else {
            return;
        };
        let mut v: [f32; 3] = velocity.into();
        ui.drag_float3(imgui::im_str!("velocity##movement{:?}", entity,), &mut v)
            .build();
        ui.drag_float(
            imgui::im_str!("max speed##movement{:?}", entity.id(),),
            &mut max_movement_speed,
        )
        .build();
        lazy.insert(
            entity,
            Movement {
                velocity,
                max_movement_speed,
            },
        );
        ui.separator();
    }

    fn add((_storage, lazy): &Self::SystemData, entity: amethyst::ecs::Entity) {
        lazy.insert(
            entity,
            Movement {
                velocity: Vector3::zeros(),
                max_movement_speed: 0.,
            },
        );
    }
}

///
///
///
pub struct Wander {
    pub angle: f32,
    pub radius: f32,
}
impl Component for Wander {
    type Storage = DenseVecStorage<Self>;
}
impl<'a> amethyst_inspector::Inspect<'a> for Wander {
    type SystemData = (ReadStorage<'a, Self>, Read<'a, LazyUpdate>);
    const CAN_ADD: bool = true;

    fn inspect(
        (storage, lazy): &Self::SystemData,
        entity: amethyst::ecs::Entity,
        ui: &imgui::Ui<'_>,
    ) {
        let &Wander {
            mut angle,
            mut radius,
        } = if let Some(x) = storage.get(entity) {
            x
        } else {
            return;
        };
        ui.drag_float(imgui::im_str!("angle##wander{:?}", entity), &mut angle)
            .build();
        ui.drag_float(imgui::im_str!("radius##wander{:?}", entity,), &mut radius)
            .build();
        lazy.insert(entity, Wander { angle, radius });
        ui.separator();
    }

    fn add((_storage, lazy): &Self::SystemData, entity: amethyst::ecs::Entity) {
        lazy.insert(entity, Wander::new(0.));
    }
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
