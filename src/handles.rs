use bevy::{
    asset::{AssetServer, Assets, Handle},
    ecs::world::FromWorld,
    sprite::{ColorMaterial, TextureAtlas},
};

use crate::texture_atlas::TextureAtlasFile;

pub struct Handles {
    pub bg_material: Handle<ColorMaterial>,
    pub leg_material: Handle<ColorMaterial>,
    pub cauldron_material: Handle<ColorMaterial>,
    pub controls_atlas: Handle<TextureAtlas>,
    pub items_atlas: Handle<TextureAtlas>,
    pub smoke_atlas: Handle<TextureAtlas>,
}

impl FromWorld for Handles {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let mut texture_atlases = unsafe {
            world
                .get_resource_unchecked_mut::<Assets<TextureAtlas>>()
                .unwrap()
        };
        let mut color_materials = unsafe {
            world
                .get_resource_unchecked_mut::<Assets<ColorMaterial>>()
                .unwrap()
        };

        // asset_server.add_loader(crate::texture_atlas::TextureAtlasLoader);
        // TODO replace this with the atlas loader when it actually works
        let controls_atlas = texture_atlases.add(
            TextureAtlasFile::from_file("assets/controls.atlas")
                .unwrap()
                .into_asset(&asset_server),
        );
        // let controls_atlas = asset_server.load("controls.atlas");
        let items_atlas = texture_atlases.add(
            TextureAtlasFile::from_file("assets/items.atlas")
                .unwrap()
                .into_asset(&asset_server),
        );
        // let items_atlas = asset_server.load("items.atlas");
        let smoke_atlas = texture_atlases.add(
            TextureAtlasFile::from_file("assets/smoke.atlas")
                .unwrap()
                .into_asset(&asset_server),
        );
        // let smoke_atlas = asset_server.load("smoke.atlas");

        Handles {
            bg_material: color_materials.add(asset_server.load("main.png").into()),
            leg_material: color_materials.add(asset_server.load("leg.png").into()),
            cauldron_material: color_materials.add(asset_server.load("cauldron.png").into()),
            controls_atlas: controls_atlas,
            items_atlas: items_atlas,
            smoke_atlas: smoke_atlas,
        }
    }
}
