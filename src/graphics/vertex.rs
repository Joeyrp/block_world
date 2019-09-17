

#[derive(Copy, Clone, Debug)]
pub struct Vertex 
{
    pub position: [f32; 3],
    pub tex_coord: [f32; 2],
    pub normal: [f32; 3]
}


implement_vertex!(Vertex, position, tex_coord, normal);