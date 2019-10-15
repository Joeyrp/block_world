
use std::{fmt, rc::Rc};

// use crate::graphics::Mesh;

#[derive(Clone, Debug)]
pub struct Voxel
{
    pub id: u16,
    pub visible: bool
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
        Layer { layer: vec![vec![Voxel { id: 0, visible: true }; width]; depth] }
    }

    pub fn fill_with(self: &mut Layer, value: u16)
    {
        for row in self.layer.iter_mut()
        {
            for i in 0..row.len()
            {
                row[i] = Voxel { id: value, visible: true };
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
    pub instance_buff: Option<Rc<glium::VertexBuffer<Attr>>>,
    pub total_blocks: u32,
    pub hidden_blocks: u32,
    pub rendered_blocks: u32,
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

        WorldChunk { width, height, depth, layers, instance_buff: None, total_blocks: 0, hidden_blocks: 0, rendered_blocks: 0 }
    }

    pub fn make_empty(self: &mut WorldChunk)
    {
        for l in 0..self.layers.len()
        {
            self.layers[l].fill_with(0);
        }
    }

    pub fn get_instance_buffer(self: &mut WorldChunk, display: &glium::Display, force_regen: bool) -> Rc<glium::VertexBuffer<Attr>>
    {
        if self.instance_buff.is_none() || force_regen
        {
            self.gen_instance_buffer(display, false);
        }

        match &self.instance_buff
        {
            Some(ib) => ib.clone(),
            None => panic!("ERROR RETURNING INSTANCE BUFFER!!!")
        }
    }

    fn gen_instance_buffer(self: &mut WorldChunk, display: &glium::Display, debug_output: bool)
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
                        //let block = &mut self.layers[l].layer[r][c];
                        
                        if self.layers[l].layer[r][c].id < 1
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
                            skipped_blocks += 1;
                            continue;
                        }

                        self.layers[l].layer[r][c].visible = !skip;

                        let x = (c as f32) * cube_size;
                        let y = (l as f32) * cube_size;
                        let z = (r as f32) * cube_size;
                        let texture = self.layers[l].layer[r][c].id as u32;

                        data.push(Attr { offset: (x, y, z), texture: texture });
                    }
                }
            }

            if debug_output
            {
                println!("Chunk Dimensions ({}x{}x{})\ntotal visible blocks: {}\nskipped blocks: {}\nrendering {} blocks", 
                        self.width, self.height, self.depth, total_blocks, skipped_blocks, data.len());
            }

            self.total_blocks = total_blocks;
            self.hidden_blocks = skipped_blocks;
            self.rendered_blocks = data.len() as u32;

            Some(Rc::new(glium::vertex::VertexBuffer::dynamic(display, &data).unwrap()))
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

        // above
        if self.layers[y + 1].layer[x][z].id < 1
        {
            return true;
        }

        // below
        if self.layers[y - 1].layer[x][z].id < 1
        {
            return true;
        }

        // The rest of the comments are from the perspective
        // of a face-on view of the layer.

        // left
        if self.layers[y].layer[x - 1][z].id < 1
        {
            return true;
        }
        
        // left top
        if self.layers[y].layer[x - 1][z - 1].id < 1
        {
            return true;
        }

        // top
        if self.layers[y].layer[x][z - 1].id < 1
        {
            return true;
        }

        // top right
        if self.layers[y].layer[x + 1][z - 1].id < 1
        {
            return true;
        }

        // right
        if self.layers[y].layer[x + 1][z].id < 1
        {
            return true;
        }

        // bottom right
        if self.layers[y].layer[x + 1][z + 1].id < 1
        {
            return true;
        }
        
        // bottom
        if self.layers[y].layer[x][z + 1].id < 1
        {
            return true;
        }

        // bottom left
        if self.layers[y].layer[x - 1][z + 1].id < 1
        {
            return true;
        }

        return false;
    }
}