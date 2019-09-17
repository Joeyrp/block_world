
extern crate nalgebra_glm as glm;

use crate::utils;

use std::fs::File;
use std::io::prelude::*;
use std::vec;

#[derive(Copy, Clone, Debug)]
struct Vertex
{
    pos: [f32; 3],
    color: [f32; 3],
}

#[derive(Debug)]
pub struct GridPlane
{
    width: i32,
    length: i32,
    vb: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
    pub program: glium::Program,
    pub view: glm::Mat4,
    pub projection: glm::Mat4,
}

impl GridPlane
{
    pub fn new(gl: &glium::Display, color: [f32; 3], cell_size: f32, width: i32, length: i32) -> Result<GridPlane, String>
    {
        implement_vertex!(Vertex, pos, color);

        // Generate Vertices
        let mut vertices: Vec<Vertex> = Vec::new();
        let half_width = width / 2;
        let start = half_width * -1;

        // Loop from -half_width to half_width
        for x in start..half_width + 1
        {
            let mut final_color = color;
            let mut final_color2 = color;

            if x == 0
            {
                final_color = [0.0, 0.0, 1.0];
                final_color2 = [0.0, 0.0, 0.0];
            }

            let xp = (x as f32) * cell_size;
            let z1 = ((length / 2) as f32) * cell_size;
            let z2 = ((length / 2) as f32) * -1.0 * cell_size; 
            vertices.push(Vertex { 
                    pos: [xp, 0.0, z1],
                    color: final_color,
                });

            vertices.push(Vertex { 
                pos: [xp, 0.0, z2],
                color: final_color2,
            });
        }

      let half_length = length / 2;
      let start = half_length * -1;
        for z in start..half_length + 1
        {
            let mut final_color = color;
            let mut final_color2 = color;

            if z == 0
            {
                final_color = [1.0, 0.0, 0.0];
                final_color2 = [0.0, 0.0, 0.0];
            }

            let zp = (z as f32) * cell_size;
            let x1 = ((width / 2) as f32) * cell_size;
            let x2 = ((width / 2) as f32) * -1.0 * cell_size; 
            vertices.push(Vertex { 
                    pos: [x1, 0.0, zp],
                    color: final_color,
                });

            vertices.push(Vertex { 
                pos: [x2, 0.0, zp],
                color: final_color2,
            });
        }

          // Create and Fill buffers
        let vertex_buffer = glium::VertexBuffer::new(gl, &vertices).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

        // load shaders
        let mut file = File::open("assets/shaders/grid_plane.vert").unwrap();
        let mut vertex_shader_src = String::new();
        file.read_to_string(&mut vertex_shader_src).unwrap();

        let mut file = File::open("assets/shaders/grid_plane.frag").unwrap();
        let mut fragment_shader_src = String::new();
        file.read_to_string(&mut fragment_shader_src).unwrap();
        
        let program = glium::Program::from_source(gl, &vertex_shader_src, &fragment_shader_src, None).unwrap();

        Ok (GridPlane { width, length, program, vb: vertex_buffer, indices,
                    view: glm::Mat4::identity(), projection: glm::Mat4::identity() })
    }

    pub fn draw(self: &GridPlane, target: &mut glium::Frame)
    {
        let model = glm::identity();
        let uniforms = uniform! 
        { 
            model: utils::mat4_to_array(&model), 
            view: utils::mat4_to_array(&self.view), 
            projection: utils::mat4_to_array(&self.projection)
        };

        use glium::Surface;
        target.draw(&self.vb, &self.indices, &self.program, &uniforms,
                        &Default::default()).unwrap();
    }
}

