
extern crate tobj;
use std::path::Path;

use crate::graphics::vertex::Vertex;

#[derive(Debug)]
pub struct Mesh
{
    pub vb: glium::VertexBuffer<Vertex>,
    pub indices: glium::IndexBuffer<u32>,
}

impl Mesh
{
    pub fn new(gl: &glium::Display, filename: &str) -> Result<Mesh, String>
    {
        let obj_file = tobj::load_obj(&Path::new(filename));

        //assert!(obj_file.is_ok());
        if obj_file.is_err()
        {
            return Err(String::from("Unable"));
        }

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

        Ok(Mesh { vb, indices })
    }

    pub fn new_from_verts(gl: &glium::Display, verts: &Vec<Vertex>, indices: &Vec<u32>) -> Mesh
    {
        let vb = glium::VertexBuffer::new(gl, verts).unwrap();
        let indices = glium::IndexBuffer::new(gl, glium::index::PrimitiveType::TrianglesList,
                                   indices).unwrap();

        Mesh { vb, indices }
    }
}