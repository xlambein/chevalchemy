use anyhow::Result;
use bevy::{
    asset::{AssetLoader, AssetPath, LoadedAsset},
    ecs::system::{Command, CommandQueue},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;

use crate::{
    items::{ItemBundle, SpawnItemBundleExt},
    ItemType,
};

#[derive(Deserialize)]
struct LevelFile {
    label: String,
    recipe: Vec<String>,
    bundles: Vec<(Vec2, String)>,
}

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "2184f3fa-2354-4d20-be9a-59cb16af498f"]
pub struct Level {
    pub label: String,
    pub recipe: Vec<ItemType>,
    bundles: Vec<(Vec2, Handle<ItemBundle>)>,
}

#[derive(Clone, Default)]
pub struct LevelLoader;

const FILE_EXTENSIONS: &[&str] = &["levels"];

impl AssetLoader for LevelLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext<'_>,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            // TODO single-item file?
            let levels: Vec<LevelFile> = ron::de::from_bytes(bytes)?;
            levels.into_iter().for_each(|level| {
                let LevelFile {
                    label,
                    recipe,
                    bundles,
                } = level;

                let bundles = bundles
                    .into_iter()
                    .map(|(pos, path)| {
                        (
                            pos,
                            load_context.get_handle(AssetPath::from(path.as_str()).get_id()),
                        )
                    })
                    .collect();

                let asset = Level {
                    label: label.clone(),
                    recipe,
                    bundles,
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

pub struct SpawnLevel(Handle<Level>);

impl Command for SpawnLevel {
    fn write(self: Box<Self>, world: &mut World) {
        let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, world);
        let levels = world.get_resource::<Assets<Level>>().unwrap();

        if levels.get(self.0.clone()).is_none() {
            eprintln!("error: could not find level '{:?}'", &self.0);
            return;
        }

        for (position, bundle) in &levels.get(self.0).unwrap().bundles {
            commands.spawn_item_bundle(bundle.clone(), *position);
        }

        command_queue.apply(world);
    }
}

pub trait SpawnLevelExt {
    fn spawn_level(&mut self, level: Handle<Level>) -> &mut Self;
}

impl SpawnLevelExt for Commands<'_> {
    fn spawn_level(&mut self, level: Handle<Level>) -> &mut Self {
        self.add(SpawnLevel(level));
        self
    }
}
