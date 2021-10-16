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
        let mut color_materials = unsafe {
            world
                .get_resource_unchecked_mut::<Assets<ColorMaterial>>()
                .unwrap()
        };

        asset_server.add_loader(crate::texture_atlas::TextureAtlasLoader);
        let controls_atlas = asset_server.load("controls.atlas");
        let items_atlas = asset_server.load("items.atlas");
        let smoke_atlas = asset_server.load("smoke.atlas");

        asset_server.load_folder("").unwrap();

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
