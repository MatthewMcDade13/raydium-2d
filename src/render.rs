use std::sync::Arc;

use anyhow::bail;
use sdl2::video::Window;

use crate::shader::Shader;

pub struct DeviceSurface {
    pub device: wgpu::Device,
    pub surface: wgpu::Surface,
    pub config: wgpu::SurfaceConfiguration,
    pub queue: wgpu::Queue,
    pub window: Arc<Window>,
}

pub struct QuadRenderer {
    ds: DeviceSurface,
    shader: Shader,
}

impl QuadRenderer {
    pub async fn new(window: Window) -> anyhow::Result<Self> {
        env_logger::init();
        let (width, height) = window.size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            dx12_shader_compiler: Default::default(),
            ..Default::default()
        });
        let surface = unsafe { instance.create_surface(&window)? };
        let adapter_opt = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await;
        let Some(adapter) = adapter_opt else {
            bail!("No Adapter Found.")
        };

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    limits: wgpu::Limits::default(),
                    label: Some("device"),
                    features: wgpu::Features::empty(),
                },
                None,
            )
            .await?;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                "shader.wgsl"
            ))),
        });

        // (2)
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 0, //std::mem::size_of::<AppState>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // (3)
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniform_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            // (4)
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        let ds = DeviceSurface {
            device,
            surface,
            config: surface_config,
            queue,
            window: Arc::new(window),
        };
        let shader = Shader {
            pipeline,
            bind_group,
            uniform_buf: uniform_buffer,
        };
        let s = Self { ds, shader };
        Ok(s)
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        self.ds.config.width = w;
        self.ds.config.height = h;
        let (device, config) = (&self.ds.device, &self.ds.config);
        self.ds.surface.configure(device, config);
    }

    #[inline]
    pub fn has_window(&self, window_id: u32) -> bool {
        window_id == self.ds.window.id()
    }

    #[inline]
    pub fn window(&self) -> &Window {
        &self.ds.window
    }
}
