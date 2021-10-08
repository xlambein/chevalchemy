use bevy::prelude::*;

use crate::handles::Handles;

use crate::{
    bone1, bone2, cubes, eyed_vial, golden_nuggets, mug, radioactive_vial, vial_stand, yorick,
};

pub fn spawn_level0(commands: &mut Commands, handles: &Res<Handles>) {
    bone1(commands, handles.items_atlas.clone(), Vec2::new(192., 89.));
    bone2(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(192., 89. + 40.),
    );
    cubes(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-300., -50.),
    );
    eyed_vial(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-190., 236.),
    );
    yorick(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-320., 230.),
    );
}

pub fn spawn_level1(commands: &mut Commands, handles: &Res<Handles>) {
    eyed_vial(commands, handles.items_atlas.clone(), Vec2::new(180., -20.));
    bone1(commands, handles.items_atlas.clone(), Vec2::new(200., 200.));
    bone2(commands, handles.items_atlas.clone(), Vec2::new(200., 250.));
    mug(commands, handles.items_atlas.clone(), Vec2::new(310., 104.));
    cubes(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-200., -53.),
    );
    yorick(commands, handles.items_atlas.clone(), Vec2::new(-200., 93.));
    golden_nuggets(commands, handles.items_atlas.clone(), Vec2::new(-318., 70.));
}

pub fn spawn_level2(commands: &mut Commands, handles: &Res<Handles>) {
    eyed_vial(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-189., 110.),
    );
    radioactive_vial(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-300., -10.),
    );
    bone1(commands, handles.items_atlas.clone(), Vec2::new(200., 200.));
    bone2(commands, handles.items_atlas.clone(), Vec2::new(200., 250.));
    mug(commands, handles.items_atlas.clone(), Vec2::new(310., 104.));
    cubes(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-200., -53.),
    );
    yorick(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-270., 225.),
    );
    golden_nuggets(commands, handles.items_atlas.clone(), Vec2::new(186., 70.));
}

pub fn spawn_level3(commands: &mut Commands, handles: &Res<Handles>) {
    eyed_vial(commands, handles.items_atlas.clone(), Vec2::new(200., -20.));
    radioactive_vial(commands, handles.items_atlas.clone(), Vec2::new(-200., 0.));
    bone1(commands, handles.items_atlas.clone(), Vec2::new(200., 200.));
    bone2(commands, handles.items_atlas.clone(), Vec2::new(200., 250.));
    mug(commands, handles.items_atlas.clone(), Vec2::new(320., 240.));
    yorick(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-310., 110.),
    );
    vial_stand(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-200., 120.),
    );
    cubes(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-300., -43.),
    );
    golden_nuggets(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-200., 200.),
    );
}

pub fn spawn_level4(commands: &mut Commands, handles: &Res<Handles>) {
    eyed_vial(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-319., 238.),
    );
    radioactive_vial(commands, handles.items_atlas.clone(), Vec2::new(327., -16.));
    mug(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-307., 100.),
    );
    yorick(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-312., -26.),
    );
    vial_stand(commands, handles.items_atlas.clone(), Vec2::new(200., 110.));
    golden_nuggets(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-200., -44.),
    );
    bone1(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-200., 200.),
    );
    bone2(
        commands,
        handles.items_atlas.clone(),
        Vec2::new(-200., 250.),
    );
    cubes(commands, handles.items_atlas.clone(), Vec2::new(223., 210.));
}
