use bevy::{
    asset::{AssetServer, Assets, Handle},
    ecs::world::FromWorld,
    sprite::{ColorMaterial, TextureAtlas},
};

use crate::items::{Item, ItemBundle, ItemBundleLoader, ItemLoader};

pub struct Handles {
    pub bg_material: Handle<ColorMaterial>,
    pub leg_material: Handle<ColorMaterial>,
    pub cauldron_material: Handle<ColorMaterial>,
    pub controls_atlas: Handle<TextureAtlas>,
    pub items_atlas: Handle<TextureAtlas>,
    pub smoke_atlas: Handle<TextureAtlas>,
    pub items: Handle<Item>,
    pub item_bundles: Handle<ItemBundle>,
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
        asset_server.add_loader(ItemLoader);
        asset_server.add_loader(ItemBundleLoader);

        let controls_atlas = asset_server.load("controls.atlas");
        let items_atlas = asset_server.load("items.atlas");
        let smoke_atlas = asset_server.load("smoke.atlas");

        let items = asset_server.load("items.items");
        let item_bundles = asset_server.load("bundles.bundles");

        asset_server.load_folder("").unwrap();

        Handles {
            bg_material: color_materials.add(asset_server.load("main.png").into()),
            leg_material: color_materials.add(asset_server.load("leg.png").into()),
            cauldron_material: color_materials.add(asset_server.load("cauldron.png").into()),
            controls_atlas: controls_atlas,
            items_atlas: items_atlas,
            smoke_atlas: smoke_atlas,
            items,
            item_bundles,
        }
    }
}

pub struct Bundles {
    pub eyed_vial: Handle<ItemBundle>,
    pub radioactive_vial: Handle<ItemBundle>,
    pub bone1: Handle<ItemBundle>,
    pub bone2: Handle<ItemBundle>,
    pub mug: Handle<ItemBundle>,
    pub yorick: Handle<ItemBundle>,
    pub vial_stand: Handle<ItemBundle>,
    pub cubes: Handle<ItemBundle>,
    pub golden_nuggets: Handle<ItemBundle>,
}

impl FromWorld for Bundles {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        Bundles {
            eyed_vial: asset_server.load("bundles.bundles#eyed_vial"),
            radioactive_vial: asset_server.load("bundles.bundles#radioactive_vial"),
            bone1: asset_server.load("bundles.bundles#bone1"),
            bone2: asset_server.load("bundles.bundles#bone2"),
            mug: asset_server.load("bundles.bundles#mug"),
            yorick: asset_server.load("bundles.bundles#yorick"),
            vial_stand: asset_server.load("bundles.bundles#vial_stand"),
            cubes: asset_server.load("bundles.bundles#cubes"),
            golden_nuggets: asset_server.load("bundles.bundles#golden_nuggets"),
        }
    }
}
