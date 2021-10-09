use bevy::prelude::*;

use crate::handles::Handles;

use crate::{
    bone1, bone2, cubes, eyed_vial, golden_nuggets, mug, radioactive_vial, vial_stand, yorick,
};

pub fn spawn_level0(commands: &mut Commands, handles: &Res<Handles>) {
    bone1(commands, handles.items_atlas.clone(), Vec2::new(96., 44.5));
    bone2(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(96., 44.5 + 20.),
    );
    cubes(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-150., -25.),
    );
    eyed_vial(commands, handles.items_atlas.clone(), Vec2::new(-95., 118.));
    yorick(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-160., 115.),
    );
}

pub fn spawn_level1(commands: &mut Commands, handles: &Res<Handles>) {
    eyed_vial(commands, handles.items_atlas.clone(), Vec2::new(90., -10.));
    bone1(commands, handles.items_atlas.clone(), Vec2::new(100., 100.));
    bone2(commands, handles.items_atlas.clone(), Vec2::new(100., 125.));
    mug(commands, handles.items_atlas.clone(), Vec2::new(155., 52.));
    cubes(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-100., -26.),
    );
    yorick(commands, handles.items_atlas.clone(), Vec2::new(-100., 46.));
    golden_nuggets(commands, handles.items_atlas.clone(), Vec2::new(-159., 35.));
}

pub fn spawn_level2(commands: &mut Commands, handles: &Res<Handles>) {
    eyed_vial(commands, handles.items_atlas.clone(), Vec2::new(-94., 55.));
    radioactive_vial(commands, handles.items_atlas.clone(), Vec2::new(-150., -5.));
    bone1(commands, handles.items_atlas.clone(), Vec2::new(100., 100.));
    bone2(commands, handles.items_atlas.clone(), Vec2::new(100., 125.));
    mug(commands, handles.items_atlas.clone(), Vec2::new(155., 52.));
    cubes(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-100., -26.),
    );
    yorick(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-135., 112.),
    );
    golden_nuggets(commands, handles.items_atlas.clone(), Vec2::new(93., 35.));
}

pub fn spawn_level3(commands: &mut Commands, handles: &Res<Handles>) {
    eyed_vial(commands, handles.items_atlas.clone(), Vec2::new(100., -10.));
    radioactive_vial(commands, handles.items_atlas.clone(), Vec2::new(-100., 0.));
    bone1(commands, handles.items_atlas.clone(), Vec2::new(100., 100.));
    bone2(commands, handles.items_atlas.clone(), Vec2::new(100., 125.));
    mug(commands, handles.items_atlas.clone(), Vec2::new(160., 120.));
    yorick(commands, handles.items_atlas.clone(), Vec2::new(-155., 55.));
    vial_stand(commands, handles.items_atlas.clone(), Vec2::new(-100., 60.));
    cubes(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-150., -21.),
    );
    golden_nuggets(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-100., 100.),
    );
}

pub fn spawn_level4(commands: &mut Commands, handles: &Res<Handles>) {
    eyed_vial(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-159., 119.),
    );
    radioactive_vial(commands, handles.items_atlas.clone(), Vec2::new(163., -8.));
    mug(commands, handles.items_atlas.clone(), Vec2::new(-153., 50.));
    yorick(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-156., -13.),
    );
    vial_stand(commands, handles.items_atlas.clone(), Vec2::new(100., 55.));
    golden_nuggets(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-100., -22.),
    );
    bone1(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-100., 100.),
    );
    bone2(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-100., 125.),
    );
    cubes(commands, handles.items_atlas.clone(), Vec2::new(111., 105.));
}
