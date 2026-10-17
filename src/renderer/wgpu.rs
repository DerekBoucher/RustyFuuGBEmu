use wgpu::util::DeviceExt;

use super::shaders;
use super::{FrameData, Renderer};
use crate::gpu::GpuContext;
use crate::ppu;
use crate::ui;

/// Horizontal size of one Game Boy pixel in normalized device coordinates.
const X_DIV: f32 = 2.0 / ppu::NATIVE_SCREEN_WIDTH as f32;
/// Vertical size of one Game Boy pixel in normalized device coordinates.
const Y_DIV: f32 = 2.0 / ppu::NATIVE_SCREEN_HEIGHT as f32;
/// NDC height the original OpenGL renderer reserved for the top menubar.
const TOP_MENUBAR_COMPENSATION: f32 = 2.0 / (ui::TOP_MENUBAR_HEIGHT * ui::SCALE_FACTOR as f32);

/// Number of vertices of the screen quad, drawn as two triangles.
const VERTEX_COUNT: u32 = 6;

// The OpenGL renderer drew one quad per Game Boy pixel. The screen is now a
// single textured quad covering exactly the same area: from the top edge of
// pixel row 0 down to the bottom edge of the last pixel row, across the full
// window width.
const SCREEN_LEFT: f32 = -1.0;
const SCREEN_RIGHT: f32 = -1.0 + (X_DIV * ppu::NATIVE_SCREEN_WIDTH as f32);
const SCREEN_TOP: f32 = 1.0 - TOP_MENUBAR_COMPENSATION + Y_DIV;
const SCREEN_BOTTOM: f32 =
    1.0 - TOP_MENUBAR_COMPENSATION - (Y_DIV * (ppu::NATIVE_SCREEN_HEIGHT - 1) as f32);

/// A vertex of the screen quad: a clip space position and the texture
/// coordinate sampled there.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

/// Draws the Game Boy screen with wgpu.
///
/// The newest [`FrameData`] is uploaded into a 160x144 texture and drawn as a
/// single quad, sampled with nearest filtering so pixels stay sharp when the
/// screen is scaled up.
pub struct WgpuRenderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    screen_texture: wgpu::Texture,
    screen_bind_group: wgpu::BindGroup,
    /// Staging buffer holding the screen pixels as RGBA8, reused every frame
    /// to avoid reallocating for each upload.
    pixels: Vec<u8>,
}

impl WgpuRenderer {
    pub fn new(gpu: &GpuContext) -> Self {
        let device = &gpu.device;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("game boy screen shader"),
            source: wgpu::ShaderSource::Wgsl(shaders::SCREEN.into()),
        });

        let screen_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("game boy screen texture"),
            size: wgpu::Extent3d {
                width: ppu::NATIVE_SCREEN_WIDTH as u32,
                height: ppu::NATIVE_SCREEN_HEIGHT as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let screen_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("game boy screen sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let screen_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("game boy screen bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let screen_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("game boy screen bind group"),
            layout: &screen_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(
                        &screen_texture.create_view(&wgpu::TextureViewDescriptor::default()),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&screen_sampler),
                },
            ],
        });

        let vertices = [
            Vertex {
                position: [SCREEN_LEFT, SCREEN_BOTTOM],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [SCREEN_RIGHT, SCREEN_BOTTOM],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [SCREEN_LEFT, SCREEN_TOP],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [SCREEN_RIGHT, SCREEN_BOTTOM],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [SCREEN_LEFT, SCREEN_TOP],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [SCREEN_RIGHT, SCREEN_TOP],
                tex_coords: [1.0, 0.0],
            },
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("game boy screen vertices"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("game boy screen pipeline layout"),
            bind_group_layouts: &[&screen_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("game boy screen pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2],
                }],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: gpu.config.format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            vertex_buffer,
            screen_texture,
            screen_bind_group,
            pixels: vec![0; ppu::NATIVE_SCREEN_WIDTH * ppu::NATIVE_SCREEN_HEIGHT * 4],
        }
    }
}

impl Renderer for WgpuRenderer {
    fn update_frame(&mut self, queue: &wgpu::Queue, frame_data: FrameData) {
        for (row_index, scanline) in frame_data.iter().enumerate() {
            for (column_index, pixel) in scanline.iter().enumerate() {
                let (red, green, blue) = pixel.to_rgb_u8();
                let offset = (row_index * ppu::NATIVE_SCREEN_WIDTH + column_index) * 4;

                self.pixels[offset] = red;
                self.pixels[offset + 1] = green;
                self.pixels[offset + 2] = blue;
                self.pixels[offset + 3] = u8::MAX;
            }
        }

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.screen_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &self.pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some((ppu::NATIVE_SCREEN_WIDTH * 4) as u32),
                rows_per_image: Some(ppu::NATIVE_SCREEN_HEIGHT as u32),
            },
            wgpu::Extent3d {
                width: ppu::NATIVE_SCREEN_WIDTH as u32,
                height: ppu::NATIVE_SCREEN_HEIGHT as u32,
                depth_or_array_layers: 1,
            },
        );
    }

    fn render(&mut self, pass: &mut wgpu::RenderPass<'static>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.screen_bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..VERTEX_COUNT, 0..1);
    }
}
