use bevy::{app::AppExit, prelude::*};
use bevy_rapier2d::{na, prelude::*};
use rand::prelude::*;
use wasm_bindgen::prelude::*;

mod editor;
mod handles;
mod items;
mod levels;
mod texture_atlas;
#[cfg(target_arch = "wasm32")]
mod wasm;

use editor::EditorPlugin;
use handles::Handles;
use levels::{Level, SpawnLevelExt};

#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(wasm::WasmPlugin);

    app.insert_resource(WindowDescriptor {
        title: "Chevalchemy: a Hoof of Concept".to_string(),
        width: 800.,
        height: 600.,
        resizable: false,
        vsync: true,
        // cursor_visible: false,
        ..Default::default()
    })
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierRenderPlugin)
    .add_plugin(EditorPlugin)
    .add_system(bevy::input::system::exit_on_esc_system.system())
    // Assets
    .add_asset::<items::Item>()
    .add_asset::<items::ItemBundle>()
    .add_asset::<levels::Level>()
    // Asset loaders
    .init_asset_loader::<texture_atlas::TextureAtlasLoader>()
    .init_asset_loader::<items::ItemLoader>()
    .init_asset_loader::<items::ItemBundleLoader>()
    .init_asset_loader::<levels::LevelLoader>()
    // Events
    .add_event::<UpdateRecipeEvent>()
    .add_event::<ResetLevelEvent>()
    .add_event::<NextLevelEvent>()
    .add_event::<ItemInCauldronEvent>()
    .init_resource::<Handles>()
    .insert_resource(MousePositionWorld::default())
    .add_startup_system(setup.system().label("setup"))
    .add_startup_system(setup_base.system().after("setup"))
    // Main menu
    .add_state(AppState::InGame)
    .insert_resource(CurrentLevel::default())
    .insert_resource(CurrentRecipe::default())
    // .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_base.system()))
    // .add_system_set_to_stage(
    //     CoreStage::PreUpdate,
    //     SystemSet::on_update(AppState::MainMenu).with_system(mouse_position.system()),
    // )
    // In-game state
    // .add_system_set_to_stage(
    //     CoreStage::PreUpdate,
    //     SystemSet::on_update(AppState::InGame).with_system(mouse_position.system()),
    // )
    .add_system(mouse_position.system())
    .add_system(probe.system())
    .add_system(cauldron_detector.system())
    .add_system(despawn_when_oob.system())
    .add_system(level_inputs.system())
    .add_system(update_recipe_events.system())
    .add_system(reset_level_events.system())
    .add_system(next_level_events.system())
    .add_system(item_in_cauldron_events.system())
    // .add_system_set(
    //     SystemSet::on_update(AppState::InGame)
    //         .with_system(mouse_position.system())
    //         .with_system(probe.system())
    //         .with_system(cauldron_detector.system())
    //         .with_system(despawn_when_oob.system())
    //         .with_system(rules.system()),
    // )
    .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
}

struct CurrentLevel(usize);

impl Default for CurrentLevel {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Clone, Default)]
struct CurrentRecipe {
    items: Vec<ItemType>,
    next_index: usize,
}

impl CurrentRecipe {
    fn new(items: Vec<ItemType>) -> Self {
        Self {
            items,
            next_index: 0,
        }
    }

    fn has_next_item(&self) -> bool {
        self.next_index < self.items.len()
    }

    fn next_item(&self) -> Option<&ItemType> {
        self.items.get(self.next_index)
    }

    fn check_item(&mut self) {
        self.next_index += 1;
    }
}

type ItemType = String;

fn item_type_to_atlas_index(item_type: &str) -> u32 {
    match item_type {
        "eyed_vial" => 0,
        "support" => 1,
        "radioactive_vial" => 3,
        "bone" => 5,
        "mug" => 7,
        "yorick" => 8,
        "vial_stand" => 9,
        "red_vial" => 11,
        "yellow_vial" => 12,
        "blue_vial" => 13,
        "cube" => 14,
        "gold_nugget" => 20,
        _ => panic!("unknown item type '{}'", item_type),
    }
}

struct UpdateRecipeEvent;
struct ResetLevelEvent;
struct NextLevelEvent;
struct ItemInCauldronEvent(ItemType);

struct IsItem(ItemType);
struct Mouse;
struct MainCamera;
struct CauldronSensor;
struct RecipeDisplay;

#[derive(Default)]
struct MousePositionWorld(Vec2);

fn spawn_cuboid(commands: &mut Commands, pos: Vec2, size: Vec2) {
    commands
        .spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: RigidBodyPosition {
                position: na::Isometry2::translation(pos.x, pos.y),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(size.x / 2., size.y / 2.),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete);
}

fn spawn_cupboard(commands: &mut Commands) {
    let board_size = Vec2::new(125., 5.);

    spawn_cuboid(commands, Vec2::new(-125., 87.), board_size);
    spawn_cuboid(commands, Vec2::new(-125., 25.), board_size);
    spawn_cuboid(commands, Vec2::new(-125., -37.), board_size);

    spawn_cuboid(commands, Vec2::new(125., 87.), board_size);
    spawn_cuboid(commands, Vec2::new(125., 25.), board_size);
    spawn_cuboid(commands, Vec2::new(125., -37.), board_size);
}

fn is_clockwise(vertices: &[[f32; 2]]) -> bool {
    (vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .map(|([x0, y0], [x1, y1])| x0 * y1 - x1 * y0)
        .sum::<f32>())
        < 0.0
}

fn make_convex_poly(shape: &[[f32; 2]]) -> ColliderShape {
    let shape = if is_clockwise(shape) {
        shape
            .iter()
            .rev()
            .cloned()
            .map(Into::into)
            .collect::<Vec<_>>()
    } else {
        shape.iter().cloned().map(Into::into).collect::<Vec<_>>()
    };
    ColliderShape::convex_polyline(shape).unwrap()
}

fn make_compound_shape(shapes: &[Vec<[f32; 2]>]) -> ColliderShape {
    ColliderShape::compound(
        shapes
            .iter()
            .map(|shape| ([0., 0.].into(), make_convex_poly(shape)))
            .collect(),
    )
}

fn cauldron(commands: &mut Commands, handles: &Res<Handles>) {
    let shapes = vec![
        vec![[-53.5, 40.0], [-49.5, 38.0], [-58.5, 0.0], [-60.5, 8.0]],
        vec![[-60.5, 7.5], [-42.0, -15.0], [-54.5, -9.5], [-60.0, -1.0]],
        vec![[-46.0, -12.5], [48.5, -12.5], [26.0, -18.5], [-22.0, -18.0]],
        vec![[48.5, -14.0], [55.5, -9.0], [62.0, 3.0], [60.5, 16.0]],
        vec![[61.0, 12.0], [52.0, 40.0], [51.0, 35.0], [60.0, 3.0]],
    ];
    let sensor = vec![
        // [-54.5, -84.5],
        // [59.5, -83.5],
        // [46.5, -100.5],
        // [-44.0, -99.5],
        [-55.5, 4.5],
        [58.5, 5.5],
        [45.5, -11.5],
        [-45.0, -10.5],
    ];

    // for shape in &shapes {
    //     eprintln!("vec![");
    // for [x, y] in &sensor {
    //     eprintln!("[{:.0}.0, {:.0}.0],", x - 1., y + 150. - 61.);
    // }
    //     eprintln!("],");
    // }

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0., 0., 100.),
            material: handles.cauldron_material.clone(),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: [1., -150. + 61.].into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: make_compound_shape(&shapes),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .with_children(|parent| {
            parent
                .spawn_bundle(ColliderBundle {
                    collider_type: ColliderType::Sensor,
                    shape: make_convex_poly(&sensor),
                    ..Default::default()
                })
                .insert(CauldronSensor);
        });
}

fn smoke(commands: &mut Commands, handles: &Res<Handles>, color: Color) {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let index = rng.gen_range(0..4);
        let pos = Vec2::new(rng.gen_range(-50.0..50.0), rng.gen_range(-62.5..-37.5));
        let speed = Vec2::new(rng.gen_range(-50.0..50.0), rng.gen_range(25.0..50.0));

        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index,
                    color,
                    ..TextureAtlasSprite::default()
                },
                texture_atlas: handles.smoke_atlas.clone(),
                transform: Transform::from_xyz(0., 0., 5.),
                ..Default::default()
            })
            .insert_bundle(RigidBodyBundle {
                position: pos.into(),
                velocity: RigidBodyVelocity {
                    linvel: speed.into(),
                    ..Default::default()
                },
                mass_properties: MassProperties::from_ball(1.0, 10.0).into(),
                forces: RigidBodyForces {
                    gravity_scale: -1.,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RigidBodyPositionSync::Discrete);
    }
}

fn setup(mut commands: Commands, mut reset_level_events: EventWriter<ResetLevelEvent>) {
    commands.insert_resource(RapierConfiguration {
        gravity: Vec2::new(0., -98.1 * 2.).into(),
        scale: 1.0,
        ..Default::default()
    });

    reset_level_events.send(ResetLevelEvent);
}

fn setup_base(mut commands: Commands, handles: Res<Handles>) {
    // Camera
    let far = 1000.0;
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(
            Transform::from_scale(Vec2::splat(0.5).extend(1.0))
                * Transform::from_xyz(0.0, 0.0, far - 0.1),
        )
        .insert(MainCamera);

    // Background
    commands.spawn_bundle(SpriteBundle {
        material: handles.bg_material.clone(),
        ..Default::default()
    });

    // Exit button
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(0),
        texture_atlas: handles.controls_atlas.clone(),
        transform: Transform::from_xyz(-167.5, -117.5, 0.),
        ..Default::default()
    });
    // Restart button
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(1),
        texture_atlas: handles.controls_atlas.clone(),
        transform: Transform::from_xyz(167.5, -117.5, 0.),
        ..Default::default()
    });

    // Cauldron
    cauldron(&mut commands, &handles);

    // Cupboard colliders
    spawn_cupboard(&mut commands);

    // Mouse
    let hoof_shape = vec![
        [4.5, 17.0],
        [-16.0, -4.5],
        [-6.0, -16.5],
        [17.0, -7.5],
        [10.5, 12.0],
    ];
    commands
        .spawn_bundle(SpriteBundle {
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::KinematicPositionBased,
            // ccd: RigidBodyCcd {
            //     ccd_enabled: true,
            //     ..Default::default()
            // },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::convex_polyline(
                hoof_shape
                    .iter()
                    .cloned()
                    .map(Into::into)
                    .collect::<Vec<_>>(),
            )
            .unwrap(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Mouse)
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(200. - 19., -162. + 19., 500.),
                material: handles.leg_material.clone(),
                ..Default::default()
            });
        });
}

fn start_level(commands: &mut Commands, handles: &Res<Handles>) {
    // Probe
    commands
        .spawn_bundle(SpriteBundle::default())
        // .spawn_bundle(SpriteSheetBundle {
        //     sprite: TextureAtlasSprite::new(1),
        //     texture_atlas: atlas,
        //     ..Default::default()
        // })
        // .insert_bundle(RigidBodyBundle {
        //     position: [-200., -20.].into(),
        //     ..Default::default()
        // })
        .with_children(|parent| {
            // parent.spawn_bundle(SpriteBundle {
            //     // sprite: Sprite::new(Vec2::new(345. * 2., 122. * 2.)),
            //     transform: Transform::from_xyz(400. - 38., -324. + 38., 500.),
            //     material: handles.leg_material.clone(),
            //     ..Default::default()
            // });
            parent.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(19),
                texture_atlas: handles.items_atlas.clone(),
                ..Default::default()
            });
            // parent
            //     .spawn()
            //     .insert_bundle(ColliderBundle {
            //         shape: ColliderShape::ball(36.),
            //         position: [0., -8.0].into(),
            //         ..Default::default()
            //     })
            //     // .insert(ColliderDebugRender::with_id(0))
            //     .insert(ColliderPositionSync::Discrete);
            // parent
            //     .spawn_bundle(ColliderBundle {
            //         shape: ColliderShape::cuboid(10.0, 15.0),
            //         position: [0., 36.0].into(),
            //         ..Default::default()
            //     })
            //     .insert(ColliderDebugRender::with_id(0))
            //     .insert(ColliderPositionSync::Discrete);
        });
}

fn mouse_position(
    wnds: Res<Windows>,
    mut mouse_position_world: ResMut<MousePositionWorld>,
    mut q: QuerySet<(
        Query<&Transform, With<MainCamera>>,
        Query<&mut RigidBodyPosition, With<Mouse>>,
    )>,
) {
    let wnd = wnds.get_primary().unwrap();

    if let Some(pos) = wnd.cursor_position() {
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        let p = pos - size / 2.0;

        let camera_transform = q.q0().single().unwrap();

        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        let pos_wld = pos_wld.truncate();

        mouse_position_world.0 = Vec2::new(pos_wld.x, pos_wld.y);

        let mut mouse = q.q1_mut().single_mut().unwrap();
        mouse.next_position.translation.vector = na::Vector2::new(pos_wld.x, pos_wld.y);
    }
}

fn probe(
    wnds: Res<Windows>,
    camera: Query<&Transform, With<MainCamera>>,
    buttons: Res<Input<MouseButton>>,
) {
    let wnd = wnds.get_primary().unwrap();

    if let Some(pos) = wnd.cursor_position() {
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        let p = pos - size / 2.0;

        let camera_transform = camera.single().unwrap();

        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        let pos_wld = pos_wld.truncate();

        if buttons.just_pressed(MouseButton::Left) {
            eprintln!("[{:.0}.0, {:.0}.0],", pos_wld.x, pos_wld.y);
            // eprintln!("Vec2::new({:.0}.0, {:.0}.0),", pos_wld.x, pos_wld.y);
        }
    }
}

fn cauldron_detector(
    mut commands: Commands,
    cauldron: Query<Entity, With<CauldronSensor>>,
    items: Query<&IsItem>,
    narrow_phase: Res<NarrowPhase>,
    mut item_in_cauldron_events: EventWriter<ItemInCauldronEvent>,
) {
    let cauldron = cauldron.single().unwrap();
    let handle = cauldron.handle();
    for (h1, h2, _b) in narrow_phase.intersections_with(handle) {
        let other = if h1 == handle { h2 } else { h1 };
        let other = other.entity();
        if let Ok(item) = items.get(other) {
            commands.entity(other).despawn_recursive();

            item_in_cauldron_events.send(ItemInCauldronEvent(item.0.clone()));
        }
    }
}

fn despawn_when_oob(
    mut commands: Commands,
    wnds: Res<Windows>,
    query: Query<(Entity, &Transform)>,
) {
    let wnd = wnds.get_primary().unwrap();
    let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

    for (entity, transform) in query.iter() {
        let pos = &transform.translation;
        if pos.x < -1000. || pos.y < -1000. || pos.x > size.x + 1000. || pos.y > size.y + 1000. {
            commands.entity(entity).despawn();
        }
    }
}

fn update_recipe_events(
    mut update_recipe_events: EventReader<UpdateRecipeEvent>,
    mut commands: Commands,
    current_recipe: Res<CurrentRecipe>,
    handles: Res<Handles>,
    recipe_display: Query<Entity, With<RecipeDisplay>>,
) {
    if let Some(_) = update_recipe_events.iter().last() {
        // Remove previous display, if any
        for entity in recipe_display.iter() {
            commands.entity(entity).despawn_recursive();
        }
        // Make a new one
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                transform: Transform::from_xyz(0., 150., 1.)
                    * Transform::from_scale(Vec3::splat(0.75)),
                ..Default::default()
            })
            .insert(RecipeDisplay)
            .with_children(|parent| {
                let mut x = -35.;
                let mut y = -60.;
                for (i, item) in current_recipe.items.iter().enumerate() {
                    let index = item_type_to_atlas_index(&item);
                    parent.spawn_bundle(SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(index),
                        texture_atlas: handles.items_atlas.clone(),
                        transform: Transform::from_xyz(x, y, 0.),
                        ..Default::default()
                    });

                    if i < current_recipe.next_index {
                        parent.spawn_bundle(SpriteSheetBundle {
                            sprite: TextureAtlasSprite::new(2),
                            texture_atlas: handles.controls_atlas.clone(),
                            transform: Transform::from_xyz(x, y, 0.),
                            ..Default::default()
                        });
                    }

                    x = -x;
                    y -= 30.;
                }
            });
    }
}

fn item_in_cauldron_events(
    mut item_in_cauldron_events: EventReader<ItemInCauldronEvent>,
    mut next_level_events: EventWriter<NextLevelEvent>,
    mut update_recipe_events: EventWriter<UpdateRecipeEvent>,
    mut reset_level_events: EventWriter<ResetLevelEvent>,
    mut commands: Commands,
    mut current_recipe: ResMut<CurrentRecipe>,
    handles: Res<Handles>,
) {
    for ItemInCauldronEvent(item_type) in item_in_cauldron_events.iter() {
        if let Some(next_item_type) = current_recipe.next_item() {
            // If it's the correct item
            if next_item_type == item_type {
                current_recipe.check_item();
                update_recipe_events.send(UpdateRecipeEvent);
                smoke(&mut commands, &handles, Color::DARK_GREEN);
                if !current_recipe.has_next_item() {
                    // TODO success screen
                    next_level_events.send(NextLevelEvent);
                }
            } else {
                // Otherwise
                // TODO failure
                smoke(&mut commands, &handles, Color::CRIMSON);
                reset_level_events.send(ResetLevelEvent);
            }
        }
    }
}

fn reset_level_events(
    mut reset_level_events: EventReader<ResetLevelEvent>,
    current_level: Res<CurrentLevel>,
    mut current_recipe: ResMut<CurrentRecipe>,
    mut update_recipe_events: EventWriter<UpdateRecipeEvent>,

    mut commands: Commands,
    levels: Res<Assets<Level>>,
    handles: Res<Handles>,
    items: Query<Entity, With<IsItem>>,
) {
    if let Some(_) = reset_level_events.iter().last() {
        let level_handle = handles.levels[current_level.0].clone();
        let level = levels.get(level_handle.clone_weak()).unwrap();
        *current_recipe = CurrentRecipe::new(level.recipe.clone());
        items.for_each(|e| commands.entity(e).despawn_recursive());
        commands.spawn_level(level_handle);
        update_recipe_events.send(UpdateRecipeEvent);
    }
}

fn next_level_events(
    mut next_level_events: EventReader<NextLevelEvent>,
    mut reset_level_events: EventWriter<ResetLevelEvent>,
    mut current_level: ResMut<CurrentLevel>,
    handles: Res<Handles>,
) {
    if let Some(_) = next_level_events.iter().last() {
        if current_level.0 < handles.levels.len() - 1 {
            current_level.0 += 1;
            // Reset level if there's a next one
            reset_level_events.send(ResetLevelEvent);
        } else {
            // TODO game end!
        }
    }
}

fn level_inputs(
    mouse_position_world: Res<MousePositionWorld>,
    buttons: Res<Input<MouseButton>>,
    mut reset_level_events: EventWriter<ResetLevelEvent>,
    mut exit: EventWriter<AppExit>,
) {
    if buttons.just_released(MouseButton::Left) {
        // Exit
        if (mouse_position_world.0 - Vec2::new(-167.5, -117.5)).length() < 22.5 {
            exit.send(AppExit);
        }
        // Restart
        if (mouse_position_world.0 - Vec2::new(167.5, -117.5)).length() < 22.5 {
            reset_level_events.send(ResetLevelEvent);
        }
    }
}
