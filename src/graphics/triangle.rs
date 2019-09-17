
use std::fs::File;
use std::io::prelude::*;
use std::vec;

#[derive(Copy, Clone, Debug)]
struct Vertex 
{
    position: [f32; 2],
    color: [f32; 3],
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Triangle
{
    vb: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
    pub program: glium::Program,
}

#[allow(dead_code)]
impl Triangle
{
    pub fn new(gl: &glium::Display) -> Result<Triangle, String>
    {
        implement_vertex!(Vertex, position, color);

        let vertex1 = Vertex { position: [-0.5, -0.5], color: [1.0, 0.0, 0.0] };
        let vertex2 = Vertex { position: [ 0.0,  0.5], color: [0.0, 1.0, 0.0] };
        let vertex3 = Vertex { position: [ 0.5, -0.5], color: [0.0, 0.0, 1.0] };

        let vertex_buffer = glium::VertexBuffer::new(gl, &vec![vertex1, vertex2, vertex3]).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let mut file = File::open("assets/shaders/triangle.vert").unwrap();
        let mut vertex_shader_src = String::new();
        file.read_to_string(&mut vertex_shader_src).unwrap();

        let mut file = File::open("assets/shaders/triangle.frag").unwrap();
        let mut fragment_shader_src = String::new();
        file.read_to_string(&mut fragment_shader_src).unwrap();

        
        let program = glium::Program::from_source(gl, &vertex_shader_src, &fragment_shader_src, None).unwrap();

        Ok( Triangle{ vb: vertex_buffer, indices, program } )
    }

    pub fn draw<U>(self: &Triangle, target: &mut glium::Frame, uniform: &U)
    where U: glium::uniforms::Uniforms,
    {
        use glium::Surface;
        target.draw(&self.vb, &self.indices, &self.program, uniform,
                        &Default::default()).unwrap();
    }
}