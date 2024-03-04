use crate::interface;
use glium::{implement_vertex, Surface};
extern crate glium;

mod shaders;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

const X_DIV: f32 = 2.0 / interface::NATIVE_SCREEN_WIDTH as f32;
const Y_DIV: f32 = 2.0 / interface::NATIVE_SCREEN_HEIGHT as f32;
const VERTICES_PER_PIXEL: usize = 6;
const VERTEX_COUNT: usize =
    interface::NATIVE_SCREEN_HEIGHT * interface::NATIVE_SCREEN_WIDTH * VERTICES_PER_PIXEL;

pub struct OpenGL {
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
}

impl OpenGL {
    pub fn new(display: &glium::backend::glutin::Display) -> Self {
        let mut position_vertices = [Vertex {
            position: [0.0, 0.0],
            color: [1.0, 1.0, 1.0],
        }; VERTEX_COUNT];

        let mut i = 0;
        let mut x = 0.0;
        let mut y = 0.0;
        while i < VERTEX_COUNT {
            position_vertices[i] = Vertex {
                position: [-1.0 + (X_DIV * x), 1.0 - (Y_DIV * y)],
                color: [1.0, 1.0, 1.0],
            };
            position_vertices[i + 1] = Vertex {
                position: [-1.0 + ((X_DIV * x) + X_DIV), 1.0 - (Y_DIV * y)],
                color: [1.0, 1.0, 1.0],
            };
            position_vertices[i + 2] = Vertex {
                position: [-1.0 + (X_DIV * x), 1.0 - ((Y_DIV * y) - Y_DIV)],
                color: [1.0, 1.0, 1.0],
            };
            position_vertices[i + 3] = Vertex {
                position: [-1.0 + ((X_DIV * x) + X_DIV), 1.0 - (Y_DIV * y)],
                color: [1.0, 1.0, 1.0],
            };
            position_vertices[i + 4] = Vertex {
                position: [-1.0 + (X_DIV * x), 1.0 - ((Y_DIV * y) - Y_DIV)],
                color: [1.0, 1.0, 1.0],
            };
            position_vertices[i + 5] = Vertex {
                position: [-1.0 + ((X_DIV * x) + X_DIV), 1.0 - ((Y_DIV * y) - Y_DIV)],
                color: [1.0, 1.0, 1.0],
            };

            x += 1.0;
            if x >= interface::NATIVE_SCREEN_WIDTH as f32 {
                x = 0.0;
                y += 1.0;
            }

            i += VERTICES_PER_PIXEL;
        }

        let vertex_buffer = glium::VertexBuffer::new(display, &position_vertices).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let program =
            glium::Program::from_source(display, shaders::VERTEX, shaders::FRAGMENT, None).unwrap();

        return Self {
            program,
            vertex_buffer,
            indices,
        };
    }

    pub fn update_frame(
        &mut self,
        display: &glium::backend::glutin::Display,
        frame_data: [[interface::Pixel; interface::NATIVE_SCREEN_WIDTH];
            interface::NATIVE_SCREEN_HEIGHT],
    ) {
        let mut i = 0;
        let mut current_vertices = self.vertex_buffer.read().unwrap();

        for scanline in frame_data {
            for pixel in scanline {
                let (r, g, b) = pixel.to_rgb();
                for j in 0..VERTICES_PER_PIXEL {
                    current_vertices[i + j].color = [r, g, b];
                }
                i += VERTICES_PER_PIXEL;
            }
        }

        self.vertex_buffer = glium::VertexBuffer::new(display, &current_vertices).unwrap();
    }

    pub fn render(&self, frame: &mut glium::Frame) {
        frame
            .draw(
                &self.vertex_buffer,
                &self.indices,
                &self.program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
    }
}
