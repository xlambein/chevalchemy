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
        transform: Transform::from_xyz(0., 0., 5.) * Transform::from_scale(Vec3::splat(2.0)),
        ..Default::default()
    });
    entity
        .insert_bundle(RigidBodyBundle {
            position: position.into(),
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..Default::default()
            },
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
            ([0., -8.].into(), ColliderShape::ball(36.)),
            ([0., 36.].into(), ColliderShape::cuboid(10.0, 15.0)),
        ]),
    );

    // Support 1
    let shape = vec![[-5.0, 12.0], [-12.0, -14.0], [13.0, -10.0]];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        1,
        ItemType::Support,
        position + Vec2::new(-36., -40.),
        make_convex_hull(&shape),
    );

    // Support 2
    let shape = vec![[1.0, 12.0], [-13.0, -13.0], [13.0, -14.0]];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        2,
        ItemType::Support,
        position + Vec2::new(36., -40.),
        make_convex_hull(&shape),
    );
}

pub fn radioactive_vial(commands: &mut Commands, atlas: Handle<TextureAtlas>, position: Vec2) {
    let shapes = vec![
        vec![[-23.0, -49.0], [-11.0, 4.0], [10.0, 6.0], [25.0, -46.0]],
        vec![[-11.0, 4.0], [-13.0, 41.0], [12.0, 48.0], [10.0, 6.0]],
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
        vec![[-39.0, 3.0], [-54.0, 7.0], [-67.0, -16.0], [-39.0, -14.0]],
        vec![[-39.0, 3.0], [-39.0, -14.0], [49.0, -13.0], [40.0, 6.0]],
        vec![[49.0, -13.0], [67.0, -16.0], [65.0, 15.0], [40.0, 6.0]],
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
        vec![[-34.0, 11.0], [-35.0, -12.0], [-14.0, -8.0], [-19.0, 8.0]],
        vec![[-14.0, -8.0], [31.0, -5.0], [30.0, 7.0], [-19.0, 8.0]],
    ];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        5,
        ItemType::Bone,
        position + Vec2::new(-20., 0.),
        make_compound_shape(&shapes),
    );

    let shapes = vec![
        vec![[-32.0, 8.0], [-34.0, -5.0], [8.0, -3.0], [29.0, 7.0]],
        vec![[8.0, -3.0], [37.0, -15.0], [40.0, 11.0], [29.0, 7.0]],
    ];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        6,
        ItemType::Bone,
        position + Vec2::new(20., 0.),
        make_compound_shape(&shapes),
    );
}

pub fn mug(commands: &mut Commands, atlas: Handle<TextureAtlas>, position: Vec2) {
    let shapes = vec![
        vec![[38.0, 35.0], [-13.0, 35.0], [-21.0, -33.0], [31.0, -37.0]],
        vec![
            [-17.0, 21.0],
            [-29.0, 22.0],
            [-38.0, 8.0],
            [-38.0, -15.0],
            [-30.0, -25.0],
            [-20.0, -26.0],
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
        [-31.0, -6.0],
        [-48.0, -26.0],
        [-25.0, -43.0],
        [-8.0, -42.0],
        [-0.0, -32.0],
    ];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        8,
        ItemType::Yorick,
        position,
        ColliderShape::compound(vec![
            ([7., -1.].into(), ColliderShape::ball(33.)),
            ([0., 0.].into(), make_convex_hull(&jaw)),
        ]),
    );
}

pub fn vial_stand(commands: &mut Commands, atlas: Handle<TextureAtlas>, position: Vec2) {
    let shapes = vec![
        // Bottom
        vec![[66.0, -22.0], [66.0, -44.0], [-59.0, -42.0], [-66.0, -25.0]],
        // Top left
        vec![[-68.0, 20.0], [-68.0, 34.0], [-55.0, 40.0], [-52.0, 21.0]],
        // Top center-left
        vec![[-11.0, 22.0], [-21.0, 21.0], [-20.0, 40.0], [-13.0, 40.0]],
        // Top center-right
        vec![[17.0, 25.0], [29.0, 24.0], [28.0, 40.0], [19.0, 41.0]],
        // Top right
        vec![[54.0, 21.0], [53.0, 41.0], [68.0, 41.0], [69.0, 20.0]],
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
    let shape = vec![[-10.0, 39.0], [-8.0, -44.0], [9.0, -41.0], [10.0, 43.0]];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        11,
        ItemType::RedVial,
        position + Vec2::new(-36., 15.),
        make_convex_hull(&shape),
    );
    // Yellow vial
    let shape = vec![
        [-10.0, 40.0],
        [-6.0, -42.0],
        [4.0, -47.0],
        [11.0, -37.0],
        [11.0, 46.0],
    ];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        12,
        ItemType::YellowVial,
        position + Vec2::new(-2., 15.),
        make_convex_hull(&shape),
    );
    // Blue vial
    let shape = vec![
        [-12.0, 45.0],
        [-11.0, -44.0],
        [-6.0, -50.0],
        [4.0, -49.0],
        [10.0, -36.0],
        [8.0, 48.0],
    ];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        13,
        ItemType::BlueVial,
        position + Vec2::new(39., 15.),
        make_convex_hull(&shape),
    );
}

pub fn cubes(commands: &mut Commands, atlas: Handle<TextureAtlas>, position: Vec2) {
    let shape = vec![[-12.0, 12.0], [-13.0, -15.0], [14.0, -12.0], [8.0, 15.0]];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        14 + 0,
        ItemType::Cube,
        position + Vec2::new(0., 64.),
        make_convex_hull(&shape),
    );
    let shape = vec![[-13.0, 9.0], [9.0, 16.0], [12.0, -13.0], [-10.0, -16.0]];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        14 + 1,
        ItemType::Cube,
        position + Vec2::new(-16., 32.),
        make_convex_hull(&shape),
    );
    let shape = vec![[12.0, 16.0], [-13.0, 13.0], [-11.0, -14.0], [12.0, -13.0]];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        14 + 2,
        ItemType::Cube,
        position + Vec2::new(16., 32.),
        make_convex_hull(&shape),
    );
    let shape = vec![[-16.0, 9.0], [12.0, 18.0], [16.0, -15.0], [-9.0, -19.0]];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        14 + 3,
        ItemType::Cube,
        position + Vec2::new(-32., 0.),
        make_convex_hull(&shape),
    );
    let shape = vec![[-13.0, 13.0], [15.0, 18.0], [15.0, -19.0], [-12.0, -16.0]];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        14 + 4,
        ItemType::Cube,
        position + Vec2::new(0., 0.),
        make_convex_hull(&shape),
    );
    let shape = vec![[14.0, 18.0], [-14.0, 17.0], [-11.0, -22.0], [14.0, -21.0]];
    spawn_from_atlas(
        commands,
        atlas.clone(),
        14 + 5,
        ItemType::Cube,
        position + Vec2::new(32., 0.),
        make_convex_hull(&shape),
    );
}

pub fn golden_nuggets(commands: &mut Commands, atlas: Handle<TextureAtlas>, position: Vec2) {
    let shape = ColliderShape::round_cuboid(10., 10., 4.);
    spawn_from_atlas(
        commands,
        atlas.clone(),
        20 + 0,
        ItemType::Gold,
        position + Vec2::new(0., 64.),
        shape.clone(),
    );
    spawn_from_atlas(
        commands,
        atlas.clone(),
        20 + 1,
        ItemType::Gold,
        position + Vec2::new(-15., 32.),
        shape.clone(),
    );
    spawn_from_atlas(
        commands,
        atlas.clone(),
        20 + 2,
        ItemType::Gold,
        position + Vec2::new(15., 32.),
        shape.clone(),
    );
    spawn_from_atlas(
        commands,
        atlas.clone(),
        20 + 3,
        ItemType::Gold,
        position + Vec2::new(-30., 0.),
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
        position + Vec2::new(30., 0.),
        shape.clone(),
    );
}
