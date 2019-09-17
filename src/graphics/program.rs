

use std::fs::File;
use std::io::prelude::*;

pub struct Program
{
    pub name: String,
    pub program: glium::Program
}

impl Program
{
    pub fn new(gl: &glium::Display, program_name: &str, vert_source_file: &str, frag_source_file: &str) -> Result<Program, String>
    {
        let mut file = File::open(vert_source_file).unwrap();
        let mut vertex_shader_src = String::new();
        file.read_to_string(&mut vertex_shader_src).unwrap();

        let mut file = File::open(frag_source_file).unwrap();
        let mut fragment_shader_src = String::new();
        file.read_to_string(&mut fragment_shader_src).unwrap();
        
        let result = glium::Program::from_source(gl, &vertex_shader_src, &fragment_shader_src, None);

        match result
        {
            Ok(program) => Ok(Program { name: String::from(program_name), program }),
            Err(error) => Err(error_to_string(&error, program_name))
        }
    }
}

fn error_to_string(program_error: &glium::program::ProgramCreationError, program_name: &str) -> String
{
    use glium::program::ProgramCreationError::{ CompilationError, LinkingError};

    match program_error
    {
        CompilationError(msg) => format!("Could not compile shader program ({}): {}", program_name, msg),
        LinkingError(msg) => format!("Could not link shader program ({}): {}", program_name, msg),
        _ => format!("Error creating shader program ({}): {:?}", program_name, program_error)
    }
}