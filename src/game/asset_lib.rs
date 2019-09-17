
use std::{ collections::HashMap, rc::Rc };
use crate::{ Gl, Mesh, Texture, Program, Flip };

pub enum AssetType
{
    MESH,
    TEXTURE,
    PROGRAM
}

pub struct AssetLib
{
    gl: Gl,
    meshes: HashMap<String, Rc<Mesh>>,
    textures: HashMap<String, Rc<Texture>>,
    programs: HashMap<String, Rc<Program>>
}

impl AssetLib
{
    pub fn new(gl: &Gl) -> AssetLib
    {
        AssetLib { gl: gl.clone(), meshes: HashMap::new(), textures: HashMap::new(), programs: HashMap::new() }
    }

    pub fn get_mesh(self: &mut AssetLib, filename: &str) -> Result<Rc<Mesh>, String>
    {
        if !self.meshes.contains_key(filename)
        {
            let mesh = Mesh::new(&self.gl, filename)?;
            self.meshes.insert(String::from(filename), Rc::new(mesh));
        }

        Ok(Rc::clone(&self.meshes[filename]))
    }

    pub fn get_texture(self: &mut AssetLib, filename: &str, flip: Flip) -> Result<Rc<Texture>, String>
    {
        if !self.textures.contains_key(filename)
        {
            let texture = Texture::new(&self.gl, filename, flip)?;
            self.textures.insert(String::from(filename), Rc::new(texture));
        }

        Ok(Rc::clone(&self.textures[filename]))
    }

    pub fn get_program(self: &mut AssetLib, program_name: &str, vert_source: &str, frag_source: &str) -> Result<Rc<Program>, String>
    {
        if !self.programs.contains_key(program_name)
        {
            let program = Program::new(&self.gl, program_name, vert_source, frag_source)?;
            self.programs.insert(String::from(program_name), Rc::new(program));
        }

        Ok(Rc::clone(&self.programs[program_name]))
    }
}