use bevy::{
    asset::{AssetServer, Assets, Handle},
    ecs::world::FromWorld,
    render::texture::Texture,
    sprite::{ColorMaterial, TextureAtlas},
};

use crate::{
    items::{Item, ItemBundle},
    levels::Level,
};

pub struct Handles {
    pub bg_material: Handle<ColorMaterial>,
    pub leg_texture: Handle<Texture>,
    pub leg_material: Handle<ColorMaterial>,
    pub cauldron_material: Handle<ColorMaterial>,
    pub controls_atlas: Handle<TextureAtlas>,
    pub items_atlas: Handle<TextureAtlas>,
    pub smoke_atlas: Handle<TextureAtlas>,
    pub items: Handle<Item>,
    pub item_bundles: Handle<ItemBundle>,
    pub levels: Vec<Handle<Level>>,
}

impl FromWorld for Handles {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let mut color_materials = unsafe {
            world
                .get_resource_unchecked_mut::<Assets<ColorMaterial>>()
                .unwrap()
        };

        let controls_atlas = asset_server.load("controls.atlas");
        let items_atlas = asset_server.load("items.atlas");
        let smoke_atlas = asset_server.load("smoke.atlas");

        let items = asset_server.load("items.items");
        let item_bundles = asset_server.load("bundles.bundles");

        let levels = vec![
            asset_server.load("levels.levels#level1"),
            asset_server.load("levels.levels#level2"),
            asset_server.load("levels.levels#level3"),
            asset_server.load("levels.levels#level4"),
            asset_server.load("levels.levels#level5"),
        ];

        asset_server.load_folder("").unwrap();

        Handles {
            bg_material: color_materials.add(asset_server.load("main.png").into()),
            leg_texture: asset_server.load("leg.png"),
            leg_material: color_materials.add(asset_server.load("leg.png").into()),
            cauldron_material: color_materials.add(asset_server.load("cauldron.png").into()),
            controls_atlas: controls_atlas,
            items_atlas: items_atlas,
            smoke_atlas: smoke_atlas,
            items,
            item_bundles,
            levels,
        }
    }
}
