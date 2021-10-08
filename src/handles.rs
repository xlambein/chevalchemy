use bevy::{
    asset::{AssetServer, Assets, Handle},
    ecs::world::FromWorld,
    math::Vec2,
    render::texture::Texture,
    sprite::{self, ColorMaterial, TextureAtlas},
};

pub struct Handles {
    pub bg_texture: Handle<Texture>,
    pub bg_material: Handle<ColorMaterial>,
    pub leg_texture: Handle<Texture>,
    pub leg_material: Handle<ColorMaterial>,
    pub cauldron_texture: Handle<Texture>,
    pub cauldron_material: Handle<ColorMaterial>,
    pub controls_texture: Handle<Texture>,
    pub controls_atlas: Handle<TextureAtlas>,
    pub items_texture: Handle<Texture>,
    pub items_atlas: Handle<TextureAtlas>,
    pub smoke_texture: Handle<Texture>,
    pub smoke_atlas: Handle<TextureAtlas>,
}

impl FromWorld for Handles {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let mut texture_atlases = unsafe {
            world
                .get_resource_unchecked_mut::<Assets<TextureAtlas>>()
                .unwrap()
        };
        let mut color_materials = unsafe {
            world
                .get_resource_unchecked_mut::<Assets<ColorMaterial>>()
                .unwrap()
        };

        let bg = asset_server.load("main.png");

        let leg: Handle<Texture> = asset_server.load("leg.png");

        let controls = asset_server.load("controls.png");
        let mut controls_atlas = TextureAtlas::new_empty(controls.clone(), Vec2::new(180.0, 63.0));
        for [min, max] in CONTROLS_ATLAS_TEXTURES {
            controls_atlas.add_texture(sprite::Rect {
                min: Vec2::new(min[0] as f32, min[1] as f32),
                max: Vec2::new(max[0] as f32, max[1] as f32),
            });
        }
        let controls_atlas = texture_atlases.add(controls_atlas);

        let items = asset_server.load("items.png");
        let mut atlas = TextureAtlas::new_empty(items.clone(), Vec2::new(400.0, 300.0));
        for [min, max] in ITEMS_ATLAS_TEXTURES {
            atlas.add_texture(sprite::Rect {
                min: Vec2::new(min[0] as f32, min[1] as f32),
                max: Vec2::new(max[0] as f32, max[1] as f32),
            });
        }
        let items_atlas = texture_atlases.add(atlas);

        let smoke = asset_server.load("smoke.png");
        let mut smoke_atlas = TextureAtlas::new_empty(smoke.clone(), Vec2::new(155.0, 144.0));
        for [min, max] in SMOKE_ATLAS_TEXTURES {
            smoke_atlas.add_texture(sprite::Rect {
                min: Vec2::new(min[0] as f32, min[1] as f32),
                max: Vec2::new(max[0] as f32, max[1] as f32),
            });
        }
        let smoke_atlas = texture_atlases.add(smoke_atlas);

        let cauldron_texture = asset_server.load("cauldron.png");

        Handles {
            bg_texture: bg.clone(),
            bg_material: color_materials.add(bg.into()),
            leg_texture: leg.clone(),
            leg_material: color_materials.add(leg.into()),
            cauldron_texture: cauldron_texture.clone(),
            cauldron_material: color_materials.add(cauldron_texture.into()),
            controls_texture: controls,
            controls_atlas: controls_atlas,
            items_texture: items,
            items_atlas: items_atlas,
            smoke_texture: smoke,
            smoke_atlas: smoke_atlas,
        }
    }
}

const CONTROLS_ATLAS_TEXTURES: &[[[i32; 2]; 2]] = &[
    // 0. Exit
    [[4, 4], [4 + 56, 4 + 52]],
    // 1. Restart
    [[62, 5], [62 + 56, 5 + 52]],
    // 2. Checkmark
    [[119, 4], [119 + 60, 4 + 56]],
];

const ITEMS_ATLAS_TEXTURES: &[[[i32; 2]; 2]] = &[
    // 0. Eyed vial
    [[270, 137], [313, 189]],
    // 1. Support 1
    [[250, 175], [269, 195]],
    // 2. Support 2
    [[250, 197], [270, 219]],
    // 3. Radioactive vial
    [[22, 130], [54, 186]],
    // 4. Bone 1
    [[230, 100], [308, 125]],
    // 5. Bone 2 (left]
    [[222, 76], [266, 96]],
    // 6. Bone 3 (right]
    [[273, 74], [321, 98]],
    // 7. Mug
    [[331, 68], [377, 116]],
    // 8. Yorick
    [[14, 78], [69, 127]],
    // 9. Vial stand (back)
    [[63, 136], [140, 189]],
    // 10. Vial stand (front)
    [[63, 136 + 54], [140, 189 + 54]],
    // 11. Red vial
    [[141, 125], [160, 180]],
    // 12. Yellow vial
    [[160, 126], [180, 180]],
    // 13. Blue vial
    [[181, 124], [200, 181]],
    // 14--19. Cubes
    [[338, 133], [338 + 24, 133 + 24]],
    [[323, 163], [323 + 24, 163 + 24]],
    [[354, 163], [354 + 24, 163 + 24]],
    [[310, 194], [310 + 25, 194 + 26]],
    [[339, 193], [339 + 25, 193 + 26]],
    [[367, 191], [367 + 24, 191 + 29]],
    // 20--25. Gold nuggets
    [[142, 4], [142 + 22, 4 + 23]],
    [[167, 3], [167 + 22, 3 + 23]],
    [[141, 32], [141 + 23, 32 + 22]],
    [[166, 32], [166 + 22, 32 + 22]],
    [[142, 56], [142 + 22, 56 + 21]],
    [[165, 55], [165 + 22, 55 + 22]],
    // //
    // [[], []],
    // //
    // [[], []],
    // //
    // [[], []],
    // //
    // [[], []],
    // //
    // [[], []],
    // //
    // [[], []],
    // //
    // [[], []],
    // //
    // [[], []],
    // //
    // [[], []],
    // //
    // [[], []],
];

const SMOKE_ATLAS_TEXTURES: &[[[i32; 2]; 2]] = &[
    [[75, 114], [75 + 28, 114 + 26]],
    [[31, 105], [31 + 35, 105 + 27]],
    [[109, 89], [109 + 27, 89 + 28]],
    [[63, 70], [63 + 42, 70 + 36]],
];
