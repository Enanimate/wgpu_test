use std::{env::current_dir, fs};

use wgpu::{BindGroupLayout, BlendState, ColorTargetState, ColorWrites, Device, Face, FragmentState, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, TextureFormat, VertexBufferLayout, VertexState};

pub(crate) struct CreateRenderPipeline<'a> {
    device: &'a Device,
    bind_group_layouts: Vec<&'a BindGroupLayout>,
    shader_filename: String,
    vertex_entry: String,
    fragment_entry: String,
    vertex_buffer_layouts: Vec<VertexBufferLayout<'static>>,
    surface_format: TextureFormat,
}

impl<'a> CreateRenderPipeline<'a> {
    pub fn new(device: &'a Device) -> Self {
        CreateRenderPipeline {
            device,
            bind_group_layouts: Vec::new(),
            shader_filename: "dummy".to_string(),
            vertex_entry: "dummy".to_string(),
            fragment_entry: "dummy".to_string(),
            vertex_buffer_layouts: Vec::new(),
            surface_format: TextureFormat::Rgba8Unorm,
        }
    }

    pub fn set_shader_module(&mut self, shader_filename: &str, vertex_entry: &str, fragment_entry: &str) -> &mut Self {
        self.shader_filename = shader_filename.to_string();
        self.vertex_entry = vertex_entry.to_string();
        self.fragment_entry = fragment_entry.to_string();

        return self;
    }

    pub fn set_surface_format(&mut self, surface_format: TextureFormat) -> &mut Self {
        self.surface_format = surface_format;

        return self;
    }

    pub fn add_vertex_buffer(&mut self, layout: VertexBufferLayout<'static>) -> &mut Self {
        self.vertex_buffer_layouts.push(layout);

        return self;
    }

    pub fn add_bind_group_layout(&mut self, layout: &'a BindGroupLayout) {
        self.bind_group_layouts.push(layout);
    }

    pub fn build(&mut self, label: &str) -> RenderPipeline {
        let mut filepath = current_dir().unwrap();
        filepath.push("render_engine/src/shaders/");
        filepath.push(self.shader_filename.as_str());
        let filepath = filepath.into_os_string().into_string().unwrap();
        let source_code = fs::read_to_string(filepath).expect("Can't read source code!");

        let shader_module_descriptor = ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: ShaderSource::Wgsl(source_code.into()),
        };

        let shader_module = self.device.create_shader_module(shader_module_descriptor);

        let pipeline_layout_descriptor = PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &self.bind_group_layouts,
            push_constant_ranges: &[],
        };
        
        let pipeline_layout = self.device.create_pipeline_layout(&pipeline_layout_descriptor);

        let render_targets = [Some(ColorTargetState {
            format: self.surface_format,
            blend: Some(BlendState::REPLACE),
            write_mask: ColorWrites::ALL,
        })];

        self.device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(&pipeline_layout),

            vertex: VertexState {
                module: &shader_module,
                entry_point: Some(&self.vertex_entry),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &self.vertex_buffer_layouts,
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: Some(&self.fragment_entry),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &render_targets,
            }),

            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },

            multiview: None,
            cache: None,
        })
    }
}