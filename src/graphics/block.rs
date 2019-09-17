
extern crate tobj;
extern crate nalgebra_glm as glm;


use crate::utils::mat4_to_array;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

#[derive(Copy, Clone, Debug)]
struct Vertex 
{
    position: [f32; 3],
    tex_coord: [f32; 2],
    normal: [f32; 3]
}

#[derive(Debug)]
pub struct Block
{
    vb: glium::VertexBuffer<Vertex>,
    indices: glium::IndexBuffer<u32>,
    program: Option<glium::Program>,
    pub perspective: glm::Mat4,
}

impl Block
{
    pub fn new(gl: &glium::Display) -> Block
    {
        implement_vertex!(Vertex, position, tex_coord, normal);

        let obj_file = tobj::load_obj(&Path::new("assets/Cube/Cube.obj"));
        assert!(obj_file.is_ok());
        let (models, _) = obj_file.unwrap();

        let mesh = &models[0].mesh;

        let mut vertices: Vec<Vertex> = Vec::new();

        assert!(mesh.positions.len() % 3 == 0);
        for v in 0..mesh.positions.len() / 3 
        {
            vertices.push(Vertex 
            {
                position: [mesh.positions[3 * v], mesh.positions[3 * v + 1], mesh.positions[3 * v + 2]],
                tex_coord: [mesh.texcoords[2 * v], mesh.texcoords[2 * v + 1]],
                normal: [mesh.normals[3 * v], mesh.normals[3 * v + 1], mesh.normals[3 * v + 2]]
            });
        }
        let vb = glium::VertexBuffer::new(gl, &vertices).unwrap();
        let indices = glium::IndexBuffer::new(gl, glium::index::PrimitiveType::TrianglesList,
                                      &mesh.indices).unwrap();

        let mut file = File::open("assets/shaders/block.vert").unwrap();
        let mut vertex_shader_src = String::new();
        file.read_to_string(&mut vertex_shader_src).unwrap();

        let mut file = File::open("assets/shaders/block.frag").unwrap();
        let mut fragment_shader_src = String::new();
        file.read_to_string(&mut fragment_shader_src).unwrap();

        
        let result = glium::Program::from_source(gl, &vertex_shader_src, &fragment_shader_src, None);

        match result
        {
            Ok(program) => Block { vb, indices, program: Some(program), perspective: glm::Mat4::identity() },
            Err(error) => 
            {
                handle_program_error(&error, "block");
                Block { vb, indices, program: None, perspective: glm::Mat4::identity() }
            }
        }
    }

    pub fn draw(self: &mut Block, target: &mut glium::Frame, model: &glm::Mat4, 
            view: &glm::Mat4, texture: &glium::texture::Texture2d) 
    {
        use glium::Surface;
        let program = match &self.program
        {
            Some(p) => p,
            None => panic!("ERROR! Attempting to draw a block with an invalid program!")
        };

        let light = [-1.0, 0.4, 0.9f32];
        let uniforms = &uniform! 
        { 
            model: mat4_to_array(model), 
            view: mat4_to_array(view),
            perspective: mat4_to_array(&self.perspective),
            u_light: light,
            tex: texture
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        
        target.draw(&self.vb, &self.indices, &program, uniforms, &params).unwrap();
    }
}

//////////////////////////////////////////////////////////
// Helper METHOD TO PRINT SHADER PROGRAM ERRORS
fn handle_program_error(program_error: &glium::program::ProgramCreationError, program_name: &str)
{
    use glium::program::ProgramCreationError::{ CompilationError, LinkingError};

    match program_error
    {
        CompilationError(msg) => panic!("Could not compile shader program ({}): {}", program_name, msg),
        LinkingError(msg) => panic!("Could not link shader program ({}): {}", program_name, msg),
        _ => panic!("Error creating shader program ({}): {:?}", program_name, program_error)
    }
}