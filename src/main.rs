use crate::winit::{
    event::{ElementState, Event, MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};
use gltf::Gltf;
use my::renderlet::plugin_runtime::camera_position::{Camera, Quaternion, Vec3};
use nalgebra_glm::Vec2;
use std::sync::Arc;

pub(crate) mod app;
pub(crate) mod backdrop;
pub(crate) mod camera;
pub(crate) mod model;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use winit;
#[cfg(target_arch = "wasm32")]
pub(crate) mod winit;

#[cfg(not(target_arch = "wasm32"))]
extern crate wgpu_native as wgpu;
#[cfg(target_arch = "wasm32")]
extern crate wgpu_wasi as wgpu;

pub mod my {
    wit_bindgen::generate!({
        path: "wit",
        world: "example:example/example",
        with: {
            // "wasi:io/poll@0.2.0": ::wasi::io::poll,
            "wasi:io/poll@0.2.0": wgpu::backend::wasi_webgpu::wasi::io::poll,
            "wasi:webgpu/surface": wgpu::backend::wasi_webgpu::wasi::webgpu::surface,
            "wasi:webgpu/webgpu": wgpu::backend::wasi_webgpu::wasi::webgpu::webgpu,
        },
    });
}

use crate::app::App;

use wgpu::backend::wasi_webgpu::wasi::webgpu::surface::PointerEvent;

async fn run(event_loop: EventLoop<()>, window: Arc<Window>, gltf: Gltf) {
    let size = window.inner_size();
    let (surface, adapter) = {
        let instance = wgpu::Instance::new(Default::default());
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .expect("Failed to find an appropriate adapter");
        (surface, adapter)
    };

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(&Default::default(), None)
        .await
        .expect("Failed to create device");

    surface.configure(
        &device,
        &surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap(),
    );

    let mut app = App::new(size, adapter, surface, device, gltf);
    use crate::my::renderlet::plugin_runtime::camera_position;
    let ccc: Option<camera_position::Camera> = None;















    let pointer_up_pollable = window.surface.subscribe_pointer_up();
    let pointer_down_pollable = window.surface.subscribe_pointer_down();
    let pointer_move_pollable = window.surface.subscribe_pointer_move();
    let key_up_pollable = window.surface.subscribe_key_up();
    let key_down_pollable = window.surface.subscribe_key_down();
    let resize_pollable = window.surface.subscribe_resize();
    let frame_pollable = window.surface.subscribe_frame();
    let camera_pollable = camera_position::on_camera_position_change_subscribe(&window.surface);
    let pollables = vec![
        &pointer_up_pollable,
        &pointer_down_pollable,
        &pointer_move_pollable,
        &key_up_pollable,
        &key_down_pollable,
        &resize_pollable,
        &frame_pollable,
        &camera_pollable,
    ];
    let mut green = false;
    // let mut point_down = false;
    let mut global_camera: Option<Camera> = None;
    let mut pointer_pos: Option<PointerEvent> = None;
    let mut i = 0;
    loop {
        let pollables_res = wgpu::backend::wasi_webgpu::wasi::io::poll::poll(&pollables[..]);

        // print("loop");

        if pollables_res.contains(&0) {
            let event = window.surface.get_pointer_up();
            print(&format!("pointer_up: {:?}", event));
            pointer_pos = None;
            green = !green;
        }
        if pollables_res.contains(&1) {
            let event = window.surface.get_pointer_down();
            if let Some(event) = event {
                pointer_pos = Some(event);
            }

            print(&format!("pointer_down: {:?}", event));
        }
        if pollables_res.contains(&2) {
            let new_pointer_pos = window.surface.get_pointer_move();
            i+= 1;
            // skip every second event
            if i % 2 == 0 {
                continue;
            }
            if let Some(new_pointer_pos) = new_pointer_pos {
                if let Some(old_pointer_pos) = pointer_pos {
                    let dx = old_pointer_pos.x - new_pointer_pos.x;
                    let dy = old_pointer_pos.y - new_pointer_pos.y;
                    pointer_pos = Some(new_pointer_pos);

                    if let Some(camera) = global_camera {
                        let new_camera = Camera {
                            orientation: Quaternion {
                                x: camera.orientation.x,
                                y: camera.orientation.y,
                                z: camera.orientation.z,
                                w: camera.orientation.w,
                            },
                            position: Vec3 {
                                x: camera.position.x + (0.1 * dx as f32),
                                y: camera.position.y + (0.1 * dy as f32),
                                z: camera.position.z,
                            },
                        };
                        global_camera = Some(new_camera);
                        camera_position::set_camera_position(new_camera);
                    }
                }
            }
            print(&format!("pointer_move: {:?}", new_pointer_pos));
        }
        if pollables_res.contains(&3) {
            let event = window.surface.get_key_up();
            print(&format!("key_up: {:?}", event));
        }
        if pollables_res.contains(&4) {
            let event = window.surface.get_key_down();
            print(&format!("key_down: {:?}", event));
        }
        if pollables_res.contains(&5) {
            let event = window.surface.get_resize();
            print(&format!("resize: {:?}", event));
        }
        if pollables_res.contains(&6) {
            let event = window.surface.get_frame();
            // print(&format!("frame: {:?}", event));
        }
        if pollables_res.contains(&7) {
            let event = camera_position::on_camera_position_change_get(&window.surface).unwrap();
            global_camera = Some(event);
            print(&format!("camera: {:?}", event));
            let euler_angles = quaternion_to_euler_angles(event.orientation);
            app.camera.yaw = euler_angles.yaw;
            app.camera.pitch = euler_angles.pitch;
            app.redraw(&queue);
        }
    }

    // event_loop
    //     .run(move |event, event_loop| match event {
    //         Event::WindowEvent { event, .. } => match event {
    //             WindowEvent::CloseRequested => {
    //                 event_loop.exit();
    //             }
    //             WindowEvent::RedrawRequested => {
    //                 app.redraw(&queue);
    //             }
    //             WindowEvent::Resized(size) => {
    //                 app.resize(size);
    //                 window.request_redraw();
    //             }
    //             WindowEvent::MouseInput { button, state, .. } => {
    //                 match state {
    //                     ElementState::Pressed => app.camera.mouse_pressed(button),
    //                     ElementState::Released => app.camera.mouse_released(button),
    //                 }
    //                 window.request_redraw();
    //             }
    //             WindowEvent::CursorMoved { position, .. } => {
    //                 app.camera
    //                     .mouse_move(Vec2::new(position.x as f32, position.y as f32));
    //                 window.request_redraw();
    //             }
    //             WindowEvent::MouseWheel { delta, .. } => {
    //                 if let MouseScrollDelta::LineDelta(_, verti) = delta {
    //                     app.camera.mouse_scroll(verti * 10.0);
    //                 }
    //                 window.request_redraw();
    //             }
    //             _ => {}
    //         },
    //         Event::DeviceEvent { event, .. } => app.device_event(event),
    //         _ => (),
    //     })
    //     .unwrap();
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let gltf = gltf::Gltf::from_slice(include_bytes!("../axis.glb")).unwrap();

    let event_loop = EventLoop::<()>::new().unwrap();
    #[cfg(not(target_arch = "wasm32"))]
    let window = event_loop.create_window(Default::default()).unwrap();
    #[cfg(target_arch = "wasm32")]
    let window = winit::window::WindowBuilder::new()
        .build(&event_loop)
        .unwrap();
    let window = Arc::new(window);
    pollster::block_on(run(event_loop, window, gltf));
}

#[cfg(target_arch = "wasm32")]
struct MyCliRunner;
#[cfg(target_arch = "wasm32")]
impl ::wasi::exports::cli::run::Guest for MyCliRunner {
    fn run() -> Result<(), ()> {
        main();
        Ok(())
    }
}
#[cfg(target_arch = "wasm32")]
::wasi::cli::command::export!(MyCliRunner);

#[derive(Clone, Copy, Debug)]
struct EulerAngles {
    yaw: f32,
    pitch: f32,
    roll: f32,
}
fn quaternion_to_euler_angles(
    q: my::renderlet::plugin_runtime::camera_position::Quaternion,
) -> EulerAngles {
    EulerAngles {
        yaw: f32::atan2(
            2.0 * (q.y * q.z + q.w * q.x),
            q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z,
        ),
        pitch: f32::asin(-2.0 * (q.x * q.z - q.w * q.y)),
        roll: f32::atan2(
            2.0 * (q.x * q.y + q.w * q.z),
            q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z,
        ),
    }
}

fn print(s: &str) {
    let stdout = wasi::cli::stdout::get_stdout();
    stdout.blocking_write_and_flush(s.as_bytes()).unwrap();
    stdout.blocking_write_and_flush(b"\n").unwrap();
}
