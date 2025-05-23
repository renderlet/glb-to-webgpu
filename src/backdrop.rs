use std::borrow::Cow;

pub struct Backdrop {
    render_pipeline: wgpu::RenderPipeline,
}

impl Backdrop {
    pub fn new(device: &wgpu::Device, swapchain_format: wgpu::TextureFormat) -> Self {
        let backdrop_src = Cow::Borrowed(include_str!("backdrop.wgsl"));

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(backdrop_src),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main".into(),
                buffers: &[],
                // compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main".into(),
                targets: &[Some(swapchain_format.into())],
                // compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Always,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            // cache: None,
        });

        Backdrop { render_pipeline }
    }

    pub fn draw(
        &self,
        frame: &wgpu::SurfaceTexture,
        depth_view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let view = frame.texture.create_view(&Default::default());
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(0.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        rpass.set_pipeline(&self.render_pipeline);
        rpass.draw(0..6, 0..1);
    }
}
