use anyhow::Result;
use bevy::{
    asset::{AssetLoader, AssetPath, LoadedAsset},
    prelude::*,
    sprite,
    utils::BoxedFuture,
};
use serde::{Deserialize, Serialize};

use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
struct SpriteRect {
    min: Vec2,
    max: Vec2,
}

impl From<sprite::Rect> for SpriteRect {
    fn from(other: sprite::Rect) -> Self {
        Self {
            min: other.min,
            max: other.max,
        }
    }
}

impl Into<sprite::Rect> for SpriteRect {
    fn into(self) -> sprite::Rect {
        sprite::Rect {
            min: self.min,
            max: self.max,
        }
    }
}

fn sprite_rect_vec_ser<S: serde::Serializer>(
    vec: &Vec<sprite::Rect>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    // First convert the vector into a Vec<LocalColor>.
    let vec2: Vec<SpriteRect> = vec.iter().cloned().map(Into::into).collect();

    // Instead of serializing Vec<ExternalCrateColor>, we serialize Vec<LocalColor>.
    vec2.serialize(serializer)
}

fn sprite_rect_vec_deser<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<sprite::Rect>, D::Error> {
    // Deserialize as if it was a Vec<LocalColor>.
    let vec: Vec<SpriteRect> = serde::Deserialize::deserialize(deserializer)?;

    // Convert it into an Vec<ExternalCrateColor>
    Ok(vec.into_iter().map(Into::into).collect())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TextureAtlasFile {
    src: PathBuf,
    size: Vec2,
    #[serde(serialize_with = "sprite_rect_vec_ser")]
    #[serde(deserialize_with = "sprite_rect_vec_deser")]
    textures: Vec<sprite::Rect>,
}

impl TextureAtlasFile {
    pub fn from_file<'a>(path: impl AsRef<Path>) -> Result<Self, ron::Error> {
        let f = std::fs::File::open(path).map_err(|e| ron::Error {
            code: ron::error::ErrorCode::Io(e.to_string()),
            position: ron::error::Position { line: 1, col: 1 },
        })?;
        ron::de::from_reader(f)
    }

    pub fn into_asset(self, asset_server: &AssetServer) -> TextureAtlas {
        let texture = asset_server.load(self.src);
        let mut atlas = TextureAtlas::new_empty(texture, self.size);
        for rect in self.textures {
            atlas.add_texture(rect);
        }
        atlas
    }
}

#[derive(Clone, Default)]
pub struct TextureAtlasLoader;

const FILE_EXTENSIONS: &[&str] = &["atlas"];

impl AssetLoader for TextureAtlasLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext<'_>,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            let atlas: TextureAtlasFile = ron::de::from_bytes(bytes)?;

            let texture: Handle<Texture> = Handle::weak(AssetPath::from(atlas.src).get_id().into());

            let mut asset = TextureAtlas::new_empty(texture, atlas.size);
            for rect in atlas.textures {
                asset.add_texture(rect);
            }

            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        FILE_EXTENSIONS
    }
}
