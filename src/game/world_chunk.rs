
use std::fmt;

// use crate::graphics::Mesh;

#[derive(Clone, Debug)]
pub struct Voxel
{
    pub id: u16,
    pub visiable: bool
}

#[derive(Clone)]
pub struct Layer
{
    width: usize,
    depth: usize,
    pub layer: Vec<Vec<Voxel>>,
   // combined: Mesh
}

impl fmt::Display for Layer
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        let mut output: String = format!("");
        for row in self.layer.iter()
        {
            output = format!("{}\n{:?}", output, row);
        }

        write!(f, "{}", output)
    }
}

impl Layer
{
    pub fn new(width: usize, depth: usize) -> Layer
    {
        Layer { width, depth, layer: vec![vec![Voxel { id: 0, visiable: true }; width]; depth] }
    }

    pub fn fill_with(self: &mut Layer, value: u16)
    {
        for row in self.layer.iter_mut()
        {
            for i in 0..row.len()
            {
                row[i] = Voxel { id: value, visiable: true };
            } 
        }
    }
}

pub struct WorldChunk
{
    height: usize,
    pub layers: Vec<Layer>
}

impl fmt::Display for WorldChunk
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        let mut output: String = format!("WorldChunk ({}, {}, {})\n____________________________________________\n", 
                                            self.layers.len(), self.layers[0].layer.len(), self.height);

        for i in 0..self.layers.len()
        {
            output = format!("{}\nLayer #{}{}\n", output, i, self.layers[i]);
        }

        write!(f, "{}", output)
    }
}

impl WorldChunk
{
    pub fn new(width: usize, depth: usize, height: usize) -> WorldChunk
    {
        let layers = vec![Layer::new(width, depth); height];

        WorldChunk { height, layers }
    }
}