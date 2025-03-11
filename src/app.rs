use gltf;
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, MouseScrollDelta},
};

use crate::{backdrop::Backdrop, camera::Camera, model::Model};

pub struct App<'a> {
    surface: wgpu::Surface<'a>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    swapchain_format: wgpu::TextureFormat,
    model: Model,
    backdrop: Backdrop,
    pub camera: Camera,
    depth: (wgpu::Texture, wgpu::TextureView),
    size: PhysicalSize<u32>,
}

impl<'a> App<'a> {
    pub fn new(
        size: PhysicalSize<u32>,
        adapter: wgpu::Adapter,
        surface: wgpu::Surface<'a>,
        device: wgpu::Device,
        gltf: gltf::Gltf,
    ) -> Self {
        let swapchain_format = surface.get_capabilities(&adapter).formats[0];

        let depth = Self::rebuild_depth_(size, &device, swapchain_format);
        let backdrop = Backdrop::new(&device, swapchain_format);

        let (model, vertices) = Model::new(&device, swapchain_format, &gltf);
        let mut camera = Camera::new(size.width as f32, size.height as f32);
        camera.fit_verts(&vertices);

        Self {
            depth,
            backdrop,
            swapchain_format,
            model,
            camera,
            surface,
            device,
            adapter,
            size,
        }
    }

    pub fn device_event(&mut self, e: DeviceEvent) {
        if let DeviceEvent::MouseWheel { delta } = e {
            if let MouseScrollDelta::PixelDelta(p) = delta {
                self.camera.mouse_scroll(p.y as f32);
            }
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        self.surface.configure(
            &self.device,
            &self
                .surface
                .get_default_config(&self.adapter, size.width, size.height)
                .unwrap(),
        );

        self.camera.set_size(size.width as f32, size.height as f32);
        self.depth = Self::rebuild_depth_(size, &self.device, self.swapchain_format);
    }

    pub fn rebuild_depth_(
        size: PhysicalSize<u32>,
        device: &wgpu::Device,
        view_format: wgpu::TextureFormat,
    ) -> (wgpu::Texture, wgpu::TextureView) {
        let size = wgpu::Extent3d {
            width: size.width,
            height: size.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("depth tex"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            // format: view_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // | wgpu::TextureUsages::SAMPLED,
            // view_formats: &[view_format],
            view_formats: &[wgpu::TextureFormat::Depth32Float],
        };
        let tex = device.create_texture(&desc);
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
        (tex, view)
    }

    pub fn redraw(&mut self, queue: &wgpu::Queue) {
        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        self.backdrop.draw(&frame, &self.depth.1, &mut encoder);
        self.model
            .draw(&self.camera, &queue, &frame, &self.depth.1, &mut encoder);
        queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
