use bevy::prelude::*;

use crate::handles::Bundles;

use crate::items::SpawnItemBundleExt;

pub fn spawn_level0(commands: &mut Commands, bundles: &Res<Bundles>) {
    commands
        .spawn_item_bundle(bundles.bone1.clone(), Vec2::new(96., 44.5))
        .spawn_item_bundle(bundles.bone2.clone(), Vec2::new(96., 44.5 + 20.))
        .spawn_item_bundle(bundles.cubes.clone(), Vec2::new(-150., -25.))
        .spawn_item_bundle(bundles.eyed_vial.clone(), Vec2::new(-95., 118.))
        .spawn_item_bundle(bundles.yorick.clone(), Vec2::new(-160., 115.));
}

pub fn spawn_level1(commands: &mut Commands, bundles: &Res<Bundles>) {
    commands
        .spawn_item_bundle(bundles.eyed_vial.clone(), Vec2::new(90., -10.))
        .spawn_item_bundle(bundles.bone1.clone(), Vec2::new(100., 100.))
        .spawn_item_bundle(bundles.bone2.clone(), Vec2::new(100., 100. + 25.))
        .spawn_item_bundle(bundles.mug.clone(), Vec2::new(155., 52.))
        .spawn_item_bundle(bundles.cubes.clone(), Vec2::new(-100., -26.))
        .spawn_item_bundle(bundles.yorick.clone(), Vec2::new(-100., 46.))
        .spawn_item_bundle(bundles.golden_nuggets.clone(), Vec2::new(-159., 35.));
}

pub fn spawn_level2(commands: &mut Commands, bundles: &Res<Bundles>) {
    commands
        .spawn_item_bundle(bundles.eyed_vial.clone(), Vec2::new(-94., 55.))
        .spawn_item_bundle(bundles.radioactive_vial.clone(), Vec2::new(-150., -5.))
        .spawn_item_bundle(bundles.bone1.clone(), Vec2::new(100., 100.))
        .spawn_item_bundle(bundles.bone2.clone(), Vec2::new(100., 100. + 25.))
        .spawn_item_bundle(bundles.mug.clone(), Vec2::new(155., 52.))
        .spawn_item_bundle(bundles.cubes.clone(), Vec2::new(-100., -26.))
        .spawn_item_bundle(bundles.yorick.clone(), Vec2::new(-135., 112.))
        .spawn_item_bundle(bundles.golden_nuggets.clone(), Vec2::new(93., 35.));
}

pub fn spawn_level3(commands: &mut Commands, bundles: &Res<Bundles>) {
    commands
        .spawn_item_bundle(bundles.eyed_vial.clone(), Vec2::new(100., -10.))
        .spawn_item_bundle(bundles.radioactive_vial.clone(), Vec2::new(-100., 0.))
        .spawn_item_bundle(bundles.bone1.clone(), Vec2::new(100., 100.))
        .spawn_item_bundle(bundles.bone2.clone(), Vec2::new(100., 100. + 25.))
        .spawn_item_bundle(bundles.mug.clone(), Vec2::new(160., 120.))
        .spawn_item_bundle(bundles.yorick.clone(), Vec2::new(-166., 55.))
        .spawn_item_bundle(bundles.vial_stand.clone(), Vec2::new(-100., 60.))
        .spawn_item_bundle(bundles.cubes.clone(), Vec2::new(-150., -21.))
        .spawn_item_bundle(bundles.golden_nuggets.clone(), Vec2::new(-100., 100.));
}

pub fn spawn_level4(commands: &mut Commands, bundles: &Res<Bundles>) {
    commands
        .spawn_item_bundle(bundles.eyed_vial.clone(), Vec2::new(-159., 119.))
        .spawn_item_bundle(bundles.radioactive_vial.clone(), Vec2::new(163., -8.))
        .spawn_item_bundle(bundles.mug.clone(), Vec2::new(-153., 50.))
        .spawn_item_bundle(bundles.yorick.clone(), Vec2::new(-156., -13.))
        .spawn_item_bundle(bundles.vial_stand.clone(), Vec2::new(100., 55.))
        .spawn_item_bundle(bundles.golden_nuggets.clone(), Vec2::new(-100., -22.))
        .spawn_item_bundle(bundles.bone1.clone(), Vec2::new(-100., 100.))
        .spawn_item_bundle(bundles.bone2.clone(), Vec2::new(-100., 100. + 25.))
        .spawn_item_bundle(bundles.cubes.clone(), Vec2::new(111., 105.));
}
