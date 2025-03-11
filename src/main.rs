use gltf::Gltf;
use nalgebra_glm::Vec2;
use std::sync::Arc;
use winit::{
    event::{ElementState, Event, MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

pub(crate) mod app;
pub(crate) mod backdrop;
pub(crate) mod camera;
pub(crate) mod model;

use crate::app::App;

async fn run(event_loop: EventLoop<()>, window: Arc<Window>, gltf: Gltf) {
    let size = window.inner_size();
    let (surface, adapter) = {
        let instance = wgpu::Instance::new(&Default::default());
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

    event_loop
        .run(move |event, event_loop| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    app.redraw(&queue);
                }
                WindowEvent::Resized(size) => {
                    app.resize(size);
                    window.request_redraw();
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    match state {
                        ElementState::Pressed => app.camera.mouse_pressed(button),
                        ElementState::Released => app.camera.mouse_released(button),
                    }
                    window.request_redraw();
                }
                WindowEvent::CursorMoved { position, .. } => {
                    app.camera
                        .mouse_move(Vec2::new(position.x as f32, position.y as f32));
                    window.request_redraw();
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    if let MouseScrollDelta::LineDelta(_, verti) = delta {
                        app.camera.mouse_scroll(verti * 10.0);
                    }
                    window.request_redraw();
                }
                _ => {}
            },
            Event::DeviceEvent { event, .. } => app.device_event(event),
            _ => (),
        })
        .unwrap();
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let gltf = gltf::Gltf::from_slice(include_bytes!("../axis.glb")).unwrap();

    let event_loop = EventLoop::new().unwrap();
    let window = event_loop.create_window(Default::default()).unwrap();
    let window = Arc::new(window);
    pollster::block_on(run(event_loop, window, gltf));
}
