use bevy::{math::Vec4Swizzles, prelude::*, render::camera::Camera};

pub struct MousePositionWorldPlugin;

impl Plugin for MousePositionWorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(MousePositionWorld::default())
            .add_system(mouse_position.system());
    }
}

#[derive(Default)]
pub struct MousePositionWorld(pub Vec2);

fn mouse_position(
    wnds: Res<Windows>,
    mut mouse_position_world: ResMut<MousePositionWorld>,
    camera: Query<&Transform, With<Camera>>,
) {
    let wnd = wnds.get_primary().unwrap();

    if let Some(pos) = wnd.cursor_position() {
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        let p = pos - size / 2.0;

        let camera_transform = camera.single().unwrap();

        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

        mouse_position_world.0 = pos_wld.xy();
    }
}
