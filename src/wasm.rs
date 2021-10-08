use bevy::prelude::*;
use wasm_bindgen::prelude::*;

pub struct WasmPlugin;

impl Plugin for WasmPlugin {
    fn build(&self, app: &mut AppBuilder) {
        console_error_panic_hook::set_once();

        app.add_plugin(bevy_webgl2::WebGL2Plugin)
            // "resizer" hack to ensure the canvas size is correct,
            // ruthlessly stolen from https://github.com/horup/some-tank-game-rs
            .add_system(resizer.system());
    }
}

#[wasm_bindgen]
extern "C" {
    fn resize_canvas(width: f32, height: f32);
}

pub fn resizer(
    window_descriptor: Res<WindowDescriptor>,
    mut windows: ResMut<Windows>,
    mut window_resized_events: EventWriter<WindowResized>,
) {
    if let Some(window) = windows.get_primary_mut() {
        let WindowDescriptor { width, height, .. } = *window_descriptor;
        if window.width() != width || window.height() != height {
            info!(
                "Current window size: {:?},{:?} with scale {:?}",
                window.width(),
                window.height(),
                window.scale_factor()
            );
            info!("Resizing to {:?},{:?}", width, height);
            let p_width = width * window.scale_factor() as f32;
            let p_height = height * window.scale_factor() as f32;
            window.update_actual_size_from_backend(p_width as u32, p_height as u32);
            window_resized_events.send(WindowResized {
                id: window.id(),
                height: height,
                width: width,
            });
            resize_canvas(width, height);
        }
    }
}
