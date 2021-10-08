use bevy::{app::AppExit, prelude::*};
use bevy_rapier2d::{na, prelude::*};
use rand::prelude::*;
use wasm_bindgen::prelude::*;

mod handles;
mod items;
mod levels;
#[cfg(target_arch = "wasm32")]
mod wasm;

use handles::Handles;
use items::*;
use levels::*;

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
    .add_system(bevy::input::system::exit_on_esc_system.system())
    // Events
    .add_event::<StartLevelEvent>()
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
    .insert_resource(CurrentLevel::new())
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

const N_LEVELS: u32 = 5;

struct CurrentLevel(u32);

impl CurrentLevel {
    fn new() -> Self {
        Self(0)
    }

    fn next_level(&mut self) -> bool {
        if self.0 < N_LEVELS - 1 {
            self.0 += 1;
            true
        } else {
            false
        }
    }

    fn recipe(&self) -> Vec<ItemType> {
        use ItemType::*;
        match self.0 {
            0 => vec![Cube, Bone, Cube],
            1 => vec![Gold, EyedVial, Bone],
            2 => vec![EyedVial, RadioactiveVial, Bone, Cube],
            3 => vec![EyedVial, Mug, YellowVial, Bone],
            4 => vec![RedVial, Gold, EyedVial, BlueVial],
            _ => unreachable!(),
        }
    }

    fn spawn(&self, commands: &mut Commands, handles: &Res<Handles>) {
        match self.0 {
            0 => spawn_level0(commands, handles),
            1 => spawn_level1(commands, handles),
            2 => spawn_level2(commands, handles),
            3 => spawn_level3(commands, handles),
            4 => spawn_level4(commands, handles),
            _ => unreachable!(),
        }
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

struct StartLevelEvent(u32);
struct UpdateRecipeEvent;
struct ResetLevelEvent;
struct NextLevelEvent;
struct ItemInCauldronEvent(ItemType);

struct Item(ItemType);
struct Mouse;
struct MainCamera;
struct CauldronSensor;
struct RecipeDisplay;

#[derive(Default)]
struct MousePositionWorld(Vec2);

#[derive(Clone, Copy, PartialEq)]
#[repr(u32)]
enum ItemType {
    EyedVial = 0,
    Support = 1,
    RadioactiveVial = 3,
    Bone = 5,
    Mug = 7,
    Yorick = 8,
    VialStand = 9,
    RedVial = 11,
    YellowVial = 12,
    BlueVial = 13,
    Cube = 14,
    Gold = 20,
}

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
    let board_size = Vec2::new(250., 10.);

    spawn_cuboid(commands, Vec2::new(-250., 175.), board_size);
    spawn_cuboid(commands, Vec2::new(-250., 50.), board_size);
    spawn_cuboid(commands, Vec2::new(-250., -75.), board_size);

    spawn_cuboid(commands, Vec2::new(250., 175.), board_size);
    spawn_cuboid(commands, Vec2::new(250., 50.), board_size);
    spawn_cuboid(commands, Vec2::new(250., -75.), board_size);
}

fn make_convex_hull(shape: &[[f32; 2]]) -> ColliderShape {
    ColliderShape::convex_hull(&shape.iter().cloned().map(Into::into).collect::<Vec<_>>()).unwrap()
}

fn make_compound_shape(shapes: &[Vec<[f32; 2]>]) -> ColliderShape {
    ColliderShape::compound(
        shapes
            .iter()
            .map(|shape| ([0., 0.].into(), make_convex_hull(shape)))
            .collect(),
    )
}

fn cauldron(commands: &mut Commands, handles: &Res<Handles>) {
    let shapes = vec![
        vec![[-107.0, 80.0], [-99.0, 76.0], [-117.0, 0.0], [-121.0, 16.0]],
        vec![
            [-121.0, 15.0],
            [-84.0, -30.0],
            [-109.0, -19.0],
            [-120.0, -2.0],
        ],
        vec![[-92.0, -25.0], [97.0, -25.0], [52.0, -37.0], [-44.0, -36.0]],
        vec![[97.0, -28.0], [111.0, -18.0], [124.0, 6.0], [121.0, 32.0]],
        vec![[122.0, 24.0], [104.0, 80.0], [102.0, 70.0], [120.0, 6.0]],
    ];
    let sensor = vec![
        // [-109.0, -169.0],
        // [119.0, -167.0],
        // [93.0, -201.0],
        // [-88.0, -199.0],
        [-111.0, 9.0],
        [117.0, 11.0],
        [91.0, -23.0],
        [-90.0, -21.0],
    ];

    // for shape in &shapes {
    //     eprintln!("vec![");
    // for [x, y] in &sensor {
    //     eprintln!("[{:.0}.0, {:.0}.0],", x - 2., y + 300. - 122.);
    // }
    //     eprintln!("],");
    // }

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0., 0., 100.) * Transform::from_scale(Vec3::splat(2.)),
            material: handles.cauldron_material.clone(),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: [2., -300. + 122.].into(),
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
                    shape: make_convex_hull(&sensor),
                    ..Default::default()
                })
                .insert(CauldronSensor);
        });
}

fn smoke(commands: &mut Commands, handles: &Res<Handles>, color: Color) {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let index = rng.gen_range(0..4);
        let pos = Vec2::new(rng.gen_range(-100.0..100.0), rng.gen_range(-125.0..-75.0));
        let speed = Vec2::new(rng.gen_range(-100.0..100.0), rng.gen_range(50.0..100.0));

        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index,
                    color,
                    ..TextureAtlasSprite::default()
                },
                texture_atlas: handles.smoke_atlas.clone(),
                transform: Transform::from_xyz(0., 0., 5.)
                    * Transform::from_scale(Vec3::splat(2.0)),
                ..Default::default()
            })
            .insert_bundle(RigidBodyBundle {
                position: pos.into(),
                velocity: RigidBodyVelocity {
                    linvel: speed.into(),
                    ..Default::default()
                },
                mass_properties: MassProperties::from_ball(1.0, 20.0).into(),
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
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    // Background
    commands.spawn_bundle(SpriteBundle {
        material: handles.bg_material.clone(),
        transform: Transform::from_scale(Vec3::splat(2.)),
        ..Default::default()
    });

    // Exit button
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(0),
        texture_atlas: handles.controls_atlas.clone(),
        transform: Transform::from_xyz(-335., -235., 0.) * Transform::from_scale(Vec3::splat(2.0)),
        ..Default::default()
    });
    // Restart button
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(1),
        texture_atlas: handles.controls_atlas.clone(),
        transform: Transform::from_xyz(335., -235., 0.) * Transform::from_scale(Vec3::splat(2.0)),
        ..Default::default()
    });

    // Cauldron
    cauldron(&mut commands, &handles);

    // Cupboard colliders
    spawn_cupboard(&mut commands);

    // Mouse
    let hoof_shape = vec![
        [9.0, 34.0],
        [-32.0, -9.0],
        [-12.0, -33.0],
        [34.0, -15.0],
        [21.0, 24.0],
    ];
    commands
        .spawn_bundle(SpriteBundle {
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::KinematicPositionBased,
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            // shape: ColliderShape::ball(10.0),
            shape: ColliderShape::convex_hull(
                &hoof_shape
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
                // sprite: Sprite::new(Vec2::new(345. * 2., 122. * 2.)),
                transform: Transform::from_xyz(400. - 38., -324. + 38., 500.)
                    * Transform::from_scale(Vec3::splat(2.)),
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
        //     transform: Transform::from_scale(Vec3::splat(2.0)),
        //     ..Default::default()
        // })
        // .insert_bundle(RigidBodyBundle {
        //     position: [-200., -20.].into(),
        //     ..Default::default()
        // })
        .with_children(|parent| {
            // parent.spawn_bundle(SpriteBundle {
            //     // sprite: Sprite::new(Vec2::new(345. * 2., 122. * 2.)),
            //     transform: Transform::from_xyz(400. - 38., -324. + 38., 500.)
            //         * Transform::from_scale(Vec3::splat(2.)),
            //     material: handles.leg_material.clone(),
            //     ..Default::default()
            // });
            parent.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(19),
                texture_atlas: handles.items_atlas.clone(),
                transform: Transform::from_scale(Vec3::splat(2.0)),
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
    items: Query<&Item>,
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

            item_in_cauldron_events.send(ItemInCauldronEvent(item.0));
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
                transform: Transform::from_xyz(0., 300., 1.)
                    * Transform::from_scale(Vec3::splat(1.5)),
                ..Default::default()
            })
            .insert(RecipeDisplay)
            .with_children(|parent| {
                let mut x = -35.;
                let mut y = -60.;
                for (i, item) in current_recipe.items.iter().enumerate() {
                    let index = *item as u32;
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
    handles: Res<Handles>,
    items: Query<Entity, With<Item>>,
) {
    if let Some(_) = reset_level_events.iter().last() {
        *current_recipe = CurrentRecipe::new(current_level.recipe());
        items.for_each(|e| commands.entity(e).despawn_recursive());
        current_level.spawn(&mut commands, &handles);
        update_recipe_events.send(UpdateRecipeEvent);
    }
}

fn next_level_events(
    mut next_level_events: EventReader<NextLevelEvent>,
    mut reset_level_events: EventWriter<ResetLevelEvent>,
    mut current_level: ResMut<CurrentLevel>,
) {
    if let Some(_) = next_level_events.iter().last() {
        if current_level.next_level() {
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
        if (mouse_position_world.0 - Vec2::new(-335., -235.)).length() < 45.0 {
            exit.send(AppExit);
        }
        // Restart
        if (mouse_position_world.0 - Vec2::new(335., -235.)).length() < 45.0 {
            reset_level_events.send(ResetLevelEvent);
        }
    }
}
