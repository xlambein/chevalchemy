use anyhow::Result;
use bevy::{
    asset::{AssetLoader, AssetPath, LoadedAsset},
    ecs::system::{Command, CommandQueue, EntityCommands},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use bevy_rapier2d::{na, prelude::*};
use serde::Deserialize;

fn is_clockwise(vertices: &[Vec2]) -> bool {
    (vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .map(|(u0, u1)| u0.x * u1.y - u1.x * u0.y)
        .sum::<f32>())
        < 0.0
}

#[derive(Debug, Clone, Deserialize)]
enum Shape {
    Ball(f32),
    Cuboid(Vec2),
    RoundCuboid(Vec2, f32),
    ConvexPolygon(Vec<Vec2>),
}

impl Into<ColliderShape> for Shape {
    fn into(self) -> ColliderShape {
        match self {
            Shape::Ball(radius) => ColliderShape::ball(radius),
            Shape::Cuboid(half_extents) => ColliderShape::cuboid(half_extents.x, half_extents.y),
            Shape::RoundCuboid(half_extents, radius) => {
                ColliderShape::round_cuboid(half_extents.x, half_extents.y, radius)
            }
            Shape::ConvexPolygon(vertices) => {
                ColliderShape::convex_polyline(if is_clockwise(&vertices) {
                    vertices
                        .iter()
                        .rev()
                        .map(|v| na::Point2::new(v.x, v.y))
                        .collect::<Vec<_>>()
                } else {
                    vertices
                        .iter()
                        .map(|v| na::Point2::new(v.x, v.y))
                        .collect::<Vec<_>>()
                })
                .unwrap()
            }
        }
    }
}

#[derive(Deserialize)]
struct ItemFile {
    label: String,
    #[serde(rename = "type")]
    ty: String,
    texture_atlas: String,
    texture_index: u32,
    colliders: Vec<(Vec2, Shape)>,
}

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "3226f39f-1918-421a-b3a0-e0b2ad20932e"]
pub struct Item {
    label: String,
    ty: String,
    texture_atlas: Handle<TextureAtlas>,
    texture_index: u32,
    colliders: Vec<(Vec2, Shape)>,
}

impl Item {
    pub fn spawn(&self, commands: &mut EntityCommands, position: Vec2) {
        let shape = ColliderShape::compound(
            self.colliders
                .iter()
                .cloned()
                .map(|(pos, shape)| (pos.into(), shape.into()))
                .collect(),
        );
        commands
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(self.texture_index),
                texture_atlas: self.texture_atlas.clone(),
                transform: Transform::from_xyz(0., 0., 5.),
                ..Default::default()
            })
            .insert_bundle(RigidBodyBundle {
                position: position.into(),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                shape,
                ..Default::default()
            })
            .insert(RigidBodyPositionSync::Discrete)
            .insert(crate::Item(self.ty.clone()));
    }
}

#[derive(Deserialize)]
struct ItemBundleFile {
    label: String,
    items: Vec<(Vec2, String)>,
}

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "6c68c7cc-59f1-4686-927b-8ebb53f06df1"]
pub struct ItemBundle {
    label: String,
    items: Vec<(Vec2, Handle<Item>)>,
}

impl ItemBundle {
    pub fn spawn(&self, commands: &mut Commands, items: &Assets<Item>, position: Vec2) {
        for (offset, item) in &self.items {
            items
                .get(item)
                .unwrap()
                .spawn(&mut commands.spawn(), position + *offset);
        }
    }
}

#[derive(Clone, Default)]
pub struct ItemLoader;

const FILE_EXTENSIONS: &[&str] = &["items"];

impl AssetLoader for ItemLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext<'_>,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            // TODO single-item file?
            let items: Vec<ItemFile> = ron::de::from_bytes(bytes)?;
            items.into_iter().for_each(|item| {
                let ItemFile {
                    label,
                    ty,
                    texture_atlas,
                    texture_index,
                    colliders,
                } = item;

                let texture_atlas: Handle<TextureAtlas> =
                    load_context.get_handle(AssetPath::from(texture_atlas.as_str()).get_id());

                let asset = Item {
                    label: label.clone(),
                    ty,
                    texture_atlas,
                    texture_index,
                    colliders,
                };
                load_context.set_labeled_asset(&label, LoadedAsset::new(asset));
            });

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        FILE_EXTENSIONS
    }
}

#[derive(Clone, Default)]
pub struct ItemBundleLoader;

const BUNDLE_FILE_EXTENSIONS: &[&str] = &["bundles"];

impl AssetLoader for ItemBundleLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext<'_>,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            // TODO single-bundle file?
            let bundles: Vec<ItemBundleFile> = ron::de::from_bytes(bytes)?;
            bundles.into_iter().for_each(|bundle| {
                let ItemBundleFile { label, items } = bundle;

                let items = items
                    .into_iter()
                    .map(|(pos, path)| {
                        (
                            pos,
                            load_context.get_handle(AssetPath::from(path.as_str()).get_id()),
                        )
                    })
                    .collect();

                let asset = ItemBundle {
                    label: label.clone(),
                    items,
                };
                load_context.set_labeled_asset(&label, LoadedAsset::new(asset));
            });

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        BUNDLE_FILE_EXTENSIONS
    }
}

pub struct SpawnItemBundle {
    pub bundle: Handle<ItemBundle>,
    pub position: Vec2,
}

impl Command for SpawnItemBundle {
    fn write(self: Box<Self>, world: &mut World) {
        let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, world);
        let bundles = world.get_resource::<Assets<ItemBundle>>().unwrap();
        let items = world.get_resource::<Assets<Item>>().unwrap();

        if bundles.get(self.bundle.clone()).is_none() {
            eprintln!("error: could not find bundle '{:?}'", &self.bundle);
            return;
        }

        for (offset, item) in &bundles.get(self.bundle).unwrap().items {
            if items.get(item).is_none() {
                eprintln!("error: could not find item '{:?}'", item);
                continue;
            }
            let item = items.get(item).unwrap();
            let shape = ColliderShape::compound(
                item.colliders
                    .iter()
                    .cloned()
                    .map(|(pos, shape)| (pos.into(), shape.into()))
                    .collect(),
            );
            commands
                .spawn()
                .insert_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(item.texture_index),
                    texture_atlas: item.texture_atlas.clone(),
                    transform: Transform::from_xyz(0., 0., 5.),
                    ..Default::default()
                })
                .insert_bundle(RigidBodyBundle {
                    position: (self.position + *offset).into(),
                    ..Default::default()
                })
                .insert_bundle(ColliderBundle {
                    shape,
                    ..Default::default()
                })
                .insert(RigidBodyPositionSync::Discrete)
                .insert(crate::Item(item.ty.clone()));
        }

        command_queue.apply(world);
    }
}

pub trait SpawnItemBundleExt {
    fn spawn_item_bundle(&mut self, bundle: Handle<ItemBundle>, position: Vec2) -> &mut Self;
}

impl SpawnItemBundleExt for Commands<'_> {
    fn spawn_item_bundle(&mut self, bundle: Handle<ItemBundle>, position: Vec2) -> &mut Self {
        self.add(SpawnItemBundle { bundle, position });
        self
    }
}
