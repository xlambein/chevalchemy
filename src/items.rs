use crate::{make_compound_shape, make_convex_hull};
use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{Item, ItemType};

fn spawn_from_atlas<'a, 'b>(
    commands: &'b mut Commands<'a>,
    atlas: Handle<TextureAtlas>,
    index: u32,
    item_type: ItemType,
    position: Vec2,
    shape: ColliderShape,
) -> EntityCommands<'a, 'b> {
    let mut entity = commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(index),
        texture_atlas: atlas.clone(),
        transform: Transform::from_xyz(0., 0., 5.),
        ..Default::default()
    });
    entity
        .insert_bundle(RigidBodyBundle {
            position: position.into(),
            // ccd: RigidBodyCcd {
            //     ccd_enabled: true,
            //     ..Default::default()
            // },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape,
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Item(item_type));
    entity
}

pub fn eyed_vial(commands: &mut Commands, atlas: Handle<TextureAtlas>, position: Vec2) {
    // Vial
    spawn_from_atlas(
        commands,
        atlas.clone(),
        0,
        ItemType::EyedVial,
        position,
        ColliderShape::compound(vec![
            ([0., -4.].into(), ColliderShape::ball(18.)),
            ([0., 18.].into(), ColliderShape::cuboid(5.0, 7.5)),
        ]),
    );

    // Support 1
    let shape = vec![[-2.5, 6.0], [-6.0, -7.0], [6.5, -5.0]];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        1,
        ItemType::Support,
        position + Vec2::new(-18., -20.),
        make_convex_hull(&shape),
    );

    // Support 2
    let shape = vec![[0.5, 6.0], [-6.5, -6.5], [6.5, -7.0]];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        2,
        ItemType::Support,
        position + Vec2::new(18., -20.),
        make_convex_hull(&shape),
    );
}

pub fn radioactive_vial(commands: &mut Commands, atlas: Handle<TextureAtlas>, position: Vec2) {
    let shapes = vec![
        vec![[-11.5, -24.5], [-5.5, 2.0], [5.0, 3.0], [12.5, -23.0]],
        vec![[-5.5, 2.0], [-6.5, 20.5], [6.0, 24.0], [5.0, 3.0]],
    ];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        3,
        ItemType::RadioactiveVial,
        position,
        make_compound_shape(&shapes),
    );
}

pub fn bone1(commands: &mut Commands, atlas: Handle<TextureAtlas>, position: Vec2) {
    let shapes = vec![
        vec![[-19.5, 1.5], [-27.0, 3.5], [-33.5, -8.0], [-19.5, -7.0]],
        vec![[-19.5, 1.5], [-19.5, -7.0], [24.5, -6.5], [20.0, 3.0]],
        vec![[24.5, -6.5], [33.5, -8.0], [32.5, 7.5], [20.0, 3.0]],
    ];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        4,
        ItemType::Bone,
        position,
        make_compound_shape(&shapes),
    );
}

pub fn bone2(commands: &mut Commands, atlas: Handle<TextureAtlas>, position: Vec2) {
    let shapes = vec![
        vec![[-17.0, 5.5], [-17.5, -6.0], [-7.0, -4.0], [-9.5, 4.0]],
        vec![[-7.0, -4.0], [15.5, -2.5], [15.0, 3.5], [-9.5, 4.0]],
    ];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        5,
        ItemType::Bone,
        position + Vec2::new(-10.0, 0.0),
        make_compound_shape(&shapes),
    );

    let shapes = vec![
        vec![[-16.0, 4.0], [-17.0, -2.5], [4.0, -1.5], [14.5, 3.5]],
        vec![[4.0, -1.5], [18.5, -7.5], [20.0, 5.5], [14.5, 3.5]],
    ];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        6,
        ItemType::Bone,
        position + Vec2::new(10.0, 0.0),
        make_compound_shape(&shapes),
    );
}

pub fn mug(commands: &mut Commands, atlas: Handle<TextureAtlas>, position: Vec2) {
    let shapes = vec![
        vec![[19.0, 17.5], [-6.5, 17.5], [-10.5, -16.5], [15.5, -18.5]],
        vec![
            [-8.5, 10.5],
            [-14.5, 11.0],
            [-19.0, 4.0],
            [-19.0, -7.5],
            [-15.0, -12.5],
            [-10.0, -13.0],
        ],
    ];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        7,
        ItemType::Mug,
        position,
        make_compound_shape(&shapes),
    );
}

pub fn yorick(commands: &mut Commands, atlas: Handle<TextureAtlas>, position: Vec2) {
    let jaw = vec![
        [-15.5, -3.0],
        [-24.0, -13.0],
        [-12.5, -21.5],
        [-4.0, -21.0],
        [-0.0, -16.0],
    ];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        8,
        ItemType::Yorick,
        position,
        ColliderShape::compound(vec![
            ([3.5, -0.5].into(), ColliderShape::ball(16.5)),
            ([0., 0.].into(), make_convex_hull(&jaw)),
        ]),
    );
}

pub fn vial_stand(commands: &mut Commands, atlas: Handle<TextureAtlas>, position: Vec2) {
    let shapes = vec![
        // Bottom
        vec![[33.0, -11.0], [33.0, -22.0], [-29.5, -21.0], [-33.0, -12.5]],
        // Top left
        vec![[-34.0, 10.0], [-34.0, 17.0], [-27.5, 20.0], [-26.0, 10.5]],
        // Top center-left
        vec![[-5.5, 11.0], [-10.5, 10.5], [-10.0, 20.0], [-6.5, 20.0]],
        // Top center-right
        vec![[8.5, 12.5], [14.5, 12.0], [14.0, 20.0], [9.5, 20.5]],
        // Top right
        vec![[27.0, 10.5], [26.5, 20.5], [34.0, 20.5], [34.5, 10.0]],
    ];
    let mut entity = spawn_from_atlas(
        commands,
        atlas.clone(),
        9,
        ItemType::VialStand,
        position,
        make_compound_shape(&shapes),
    );
    // Front
    entity.with_children(|parent| {
        parent.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(10),
            texture_atlas: atlas.clone(),
            transform: Transform::from_xyz(0., 0., 10.),
            ..Default::default()
        });
    });

    // Red vial
    let shape = vec![[-5.0, 19.5], [-4.0, -22.0], [4.5, -20.5], [5.0, 21.5]];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        11,
        ItemType::RedVial,
        position + Vec2::new(-18., 7.),
        make_convex_hull(&shape),
    );
    // Yellow vial
    let shape = vec![
        [-5.0, 20.0],
        [-3.0, -21.0],
        [2.0, -23.5],
        [5.5, -18.5],
        [5.5, 23.0],
    ];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        12,
        ItemType::YellowVial,
        position + Vec2::new(-1., 7.),
        make_convex_hull(&shape),
    );
    // Blue vial
    let shape = vec![
        [-6.0, 22.5],
        [-5.5, -22.0],
        [-3.0, -25.0],
        [2.0, -24.5],
        [5.0, -18.0],
        [4.0, 24.0],
    ];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        13,
        ItemType::BlueVial,
        position + Vec2::new(19., 7.),
        make_convex_hull(&shape),
    );
}

pub fn cubes(commands: &mut Commands, atlas: Handle<TextureAtlas>, position: Vec2) {
    let shape = vec![[-6.0, 6.0], [-6.5, -7.5], [7.0, -6.0], [4.0, 7.5]];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        14 + 0,
        ItemType::Cube,
        position + Vec2::new(0., 32.),
        make_convex_hull(&shape),
    );
    let shape = vec![[-7.5, 4.5], [4.5, 8.0], [6.0, -7.5], [-5.0, -8.0]];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        14 + 1,
        ItemType::Cube,
        position + Vec2::new(-8., 16.),
        make_convex_hull(&shape),
    );
    let shape = vec![[6.0, 8.0], [-7.5, 7.5], [-5.5, -7.5], [6.0, -7.5]];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        14 + 2,
        ItemType::Cube,
        position + Vec2::new(8., 16.),
        make_convex_hull(&shape),
    );
    let shape = vec![[-8.0, 4.5], [6.0, 9.0], [8.0, -7.5], [-4.5, -9.5]];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        14 + 3,
        ItemType::Cube,
        position + Vec2::new(-16., 0.),
        make_convex_hull(&shape),
    );
    let shape = vec![[-7.5, 7.5], [7.5, 9.0], [7.5, -9.5], [-6.0, -8.0]];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        14 + 4,
        ItemType::Cube,
        position + Vec2::new(0., 0.),
        make_convex_hull(&shape),
    );
    let shape = vec![[7.0, 9.0], [-7.0, 8.5], [-5.5, -11.0], [7.0, -10.5]];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        14 + 5,
        ItemType::Cube,
        position + Vec2::new(16., 0.),
        make_convex_hull(&shape),
    );
}

pub fn golden_nuggets(commands: &mut Commands, atlas: Handle<TextureAtlas>, position: Vec2) {
    let shape = ColliderShape::round_cuboid(5., 5., 2.);
    spawn_from_atlas(
        commands,
        atlas.clone(),
        20 + 0,
        ItemType::Gold,
        position + Vec2::new(0., 32.),
        shape.clone(),
    );
    spawn_from_atlas(
        commands,
        atlas.clone(),
        20 + 1,
        ItemType::Gold,
        position + Vec2::new(-7.5, 16.),
        shape.clone(),
    );
    spawn_from_atlas(
        commands,
        atlas.clone(),
        20 + 2,
        ItemType::Gold,
        position + Vec2::new(7.5, 16.),
        shape.clone(),
    );
    spawn_from_atlas(
        commands,
        atlas.clone(),
        20 + 3,
        ItemType::Gold,
        position + Vec2::new(-15., 0.),
        shape.clone(),
    );
    spawn_from_atlas(
        commands,
        atlas.clone(),
        20 + 4,
        ItemType::Gold,
        position + Vec2::new(0., 0.),
        shape.clone(),
    );
    spawn_from_atlas(
        commands,
        atlas.clone(),
        20 + 5,
        ItemType::Gold,
        position + Vec2::new(15., 0.),
        shape.clone(),
    );
}
