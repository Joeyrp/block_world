
use std::fmt;

// use crate::graphics::Mesh;

#[derive(Clone, Debug)]
pub struct Voxel
{
    pub id: u16,
    pub visiable: bool
}

#[derive(Copy, Clone)]
pub struct Attr 
{
    offset: (f32, f32, f32),
    texture: u32,
}

implement_vertex!(Attr, offset, texture);

#[derive(Clone)]
pub struct Layer
{
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
        Layer { layer: vec![vec![Voxel { id: 0, visiable: true }; width]; depth] }
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
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub layers: Vec<Layer>,
    pub instance_buff: Option<glium::VertexBuffer<Attr>>
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
    pub fn new(width: usize, height: usize, depth: usize,) -> WorldChunk
    {
        let layers = vec![Layer::new(width, depth); height];

        WorldChunk { width, height, depth, layers, instance_buff: None }
    }

    pub fn gen_instance_buffer(self: &mut WorldChunk, display: &glium::Display, debug_output: bool)
    {
        let mut total_blocks = 0;
        let mut skipped_blocks = 0;
        self.instance_buff = {
            let mut data: Vec<Attr> = vec![];
            let cube_size = 1.0;
            for l in 0..self.layers.len()
            {
                for r in 0..self.layers[l].layer.len()
                {
                    for c in 0..self.layers[l].layer[r].len()
                    {
                        let block = &self.layers[l].layer[r][c];
                        
                        if block.id < 1
                        {
                            continue;
                        }

                        total_blocks += 1;
                        
                        let mut skip = true;

                        if self.has_neighbor_gap(r, l, c)
                        {
                            skip = false;
                        }

                        if skip
                        {
                            self.layers[l].layer[r][c].visiable = false;
                            skipped_blocks += 1;
                            continue;
                        }

                        let x = (c as f32) * cube_size;
                        let y = (l as f32) * cube_size;
                        let z = (r as f32) * cube_size;

                        let texture = block.id as u32;

                        data.push(Attr { offset: (x, y, z), texture: texture });
                    }
                }
            }

            if debug_output
            {
                println!("Chunk Dimensions (16x16x16)\ntotal blocks: {}\nskipped blocks: {}\nrendering {} cubes", 
                        total_blocks, skipped_blocks, data.len());
            }

            Some(glium::vertex::VertexBuffer::dynamic(display, &data).unwrap())
        };
    }

    fn has_neighbor_gap(self: &WorldChunk, x: usize, y: usize, z: usize) -> bool
    {

        if x == 0 || y == 0 || z == 0
            || x == self.width -1
            || y == self.height -1
            || z == self.depth -1
            {
                return true;
            }
        
        for offset in -1..2 as i32
        {
            let idx = (y as i32 + offset) as usize;
            if self.layers[idx].layer[x][z].id < 1
            {
                return true;
            }

            // left
            if self.layers[idx].layer[x - 1][z].id < 1
            {
                return true;
            }
            
            // left top
            if self.layers[idx].layer[x - 1][z - 1].id < 1
            {
                return true;
            }

            // top
            if self.layers[idx].layer[x][z - 1].id < 1
            {
                return true;
            }

            // top right
            if self.layers[idx].layer[x + 1][z - 1].id < 1
            {
                return true;
            }

            // right
            if self.layers[idx].layer[x + 1][z].id < 1
            {
                return true;
            }

            // bottom right
            if self.layers[idx].layer[x + 1][z + 1].id < 1
            {
                return true;
            }
            
            // bottom
            if self.layers[idx].layer[x][z + 1].id < 1
            {
                return true;
            }

            // bottom left
            if self.layers[idx].layer[x - 1][z + 1].id < 1
            {
                return true;
            }

        }

        return false;
    }
}