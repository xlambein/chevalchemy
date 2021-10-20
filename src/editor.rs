use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use std::collections::HashMap;

use crate::{
    items::{Item, ItemBundle, SpawnItemBundleExt},
    mouse_position_world::MousePositionWorld,
};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(EguiPlugin)
            .insert_resource(EguiTextures::default())
            .add_system(ui_bundle_selector.system());
    }
}

#[derive(Default)]
struct EguiTextures {
    map: HashMap<Handle<Texture>, u64>,
    next: u64,
    new: Vec<(Handle<Texture>, u64)>,
}

impl EguiTextures {
    fn get(&mut self, handle: Handle<Texture>) -> u64 {
        let Self { map, next, new } = self;
        *map.entry(handle.clone_weak()).or_insert_with(|| {
            *next += 1;
            new.push((handle, *next));
            *next
        })
    }

    fn register_new_textures(&mut self, egui_context: &mut EguiContext) {
        while let Some((handle, id)) = self.new.pop() {
            egui_context.set_egui_texture(id, handle);
        }
    }
}

fn show_bundle(
    ui: &mut egui::Ui,
    egui_textures: &mut ResMut<EguiTextures>,
    items: &Assets<Item>,
    texture_atlases: &Assets<TextureAtlas>,
    bundle: &ItemBundle,
    size: egui::Vec2,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());
    let center = rect.center();

    let bounds = {
        let mut min = Vec2::splat(f32::INFINITY);
        let mut max = Vec2::splat(-f32::INFINITY);
        for (offset, item) in &bundle.items {
            let offset = *offset * Vec2::new(1.0, -1.0);
            let item = items.get(item).unwrap();
            let texture_atlas = texture_atlases.get(&item.texture_atlas).unwrap();
            let tex_rect = &texture_atlas.textures[item.texture_index as usize];
            let half_size = Vec2::new(tex_rect.width(), tex_rect.height()) / 2.;

            min = min.min(offset - half_size);
            max = max.max(offset + half_size);
        }
        egui::Rect {
            min: egui::pos2(min.x, min.y),
            max: egui::pos2(max.x, max.y),
        }
    };

    let center = center - bounds.center().to_vec2();

    for (offset, item) in &bundle.items {
        let image_center = center + egui::Vec2::new(offset.x, -offset.y);

        let item = items.get(item).unwrap();
        let texture_atlas = texture_atlases.get(&item.texture_atlas).unwrap();

        let tex_rect = &texture_atlas.textures[item.texture_index as usize];
        let tex_uv = egui::Rect {
            min: <[f32; 2]>::from(tex_rect.min / texture_atlas.size).into(),
            max: <[f32; 2]>::from(tex_rect.max / texture_atlas.size).into(),
        };
        let tex_rect = egui::Rect {
            min: <[f32; 2]>::from(tex_rect.min).into(),
            max: <[f32; 2]>::from(tex_rect.max).into(),
        };
        let image_rect = egui::Rect::from_center_size(image_center, tex_rect.size());

        let tex = egui_textures.get(texture_atlas.texture.clone_weak());

        ui.put(
            image_rect,
            egui::widgets::Image::new(egui::TextureId::User(tex), image_rect.size()).uv(tex_uv),
        );
    }

    response
}

fn ui_bundle_selector(
    mut egui_context: ResMut<EguiContext>,
    items: Res<Assets<Item>>,
    bundles: Res<Assets<ItemBundle>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut egui_textures: ResMut<EguiTextures>,
    mut commands: Commands,
    mouse_position_world: Res<MousePositionWorld>,
) {
    egui_textures.register_new_textures(&mut egui_context);

    let ctx = egui_context.ctx();
    egui::Window::new("Editor").show(ctx, |ui| {
        ui.set_max_width(100.);
        egui::ScrollArea::auto_sized().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                for (id, bundle) in bundles.iter() {
                    let response = show_bundle(
                        ui,
                        &mut egui_textures,
                        &items,
                        &texture_atlases,
                        bundle,
                        egui::vec2(100., 100.),
                    )
                    .interact(egui::Sense::drag());

                    if response.hovered() {
                        ui.output().cursor_icon = egui::CursorIcon::Grab;
                    }
                    if response.dragged() {
                        ui.output().cursor_icon = egui::CursorIcon::Grabbing;
                    }
                    if response.drag_released() && !ctx.is_pointer_over_area() {
                        commands.spawn_item_bundle(Handle::weak(id), mouse_position_world.0);
                    }
                }
            });
        });
    });
}
