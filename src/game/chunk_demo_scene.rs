
use crate::{ utils::mat4_to_array, GridPlane, AssetLib, Flip, WorldChunk };

#[derive(Copy, Clone)]
struct Attr 
{
    offset: (f32, f32, f32),
    texture: u32,
}

implement_vertex!(Attr, offset, texture);

pub struct ChunkDemoScene
{
    grid: GridPlane,
    chunk: WorldChunk,
    perspective: glm::Mat4,
    instance_buff: Option<glium::VertexBuffer<Attr>>
}

impl ChunkDemoScene
{
    pub fn new(assets: &mut AssetLib, display: &glium::Display, perspective: &glm::Mat4) 
        -> Result<ChunkDemoScene, String>
    {
        // Pre Load assets
        assets.get_mesh("assets/Cube/BasicCube.obj")?;
        assets.get_texture("assets/textures/Grass.png", Flip::NONE)?;
        assets.get_texture("assets/textures/Dirt.png", Flip::NONE)?;
        assets.get_texture("assets/textures/Stone.png", Flip::NONE)?;
        assets.get_program("Blocks_instanced", "assets/shaders/block_instanced.vert", "assets/shaders/block.frag")?;

        let mut grid = GridPlane::new(&display, [0.75, 0.75, 0.75], 1.0, 20, 20).unwrap();
        grid.projection = *perspective;

        Ok( ChunkDemoScene { grid, chunk: WorldChunk::new(16, 16, 16), perspective: *perspective, instance_buff: None })
    }

    pub fn make_chunk_single_layer(self: &mut ChunkDemoScene)
    {
        self.chunk.layers[8].fill_with(1);
    }

    pub fn make_chunk_single_layer_with_hole(self: &mut ChunkDemoScene)
    {
        self.chunk.layers[5].fill_with(1);

        self.chunk.layers[5].layer[8][8].id = 0;
    }

    pub fn make_test_one(self: &mut ChunkDemoScene, display: &glium::Display)
    {
        /*
            LAYERS: 16x16x16 = 4,096
            INTERIOR: 14x14x14 = 2,744

            SHOULD RENDER: 1,352
        */

       // self.chunk.layers[15].fill_with(1);
      //  self.chunk.layers[14].fill_with(2);
      //  self.chunk.layers[13].fill_with(2);

        for i in 0..16
        {
            self.chunk.layers[i].fill_with((i % 3 + 1) as u16);
        }

        // self.chunk.layers[12].layer[7][7].id = 0;
        // self.chunk.layers[11].layer[7][7].id = 0;
        // self.chunk.layers[10].layer[7][7].id = 0;
        // self.chunk.layers[9].layer[7][7].id = 0;

        // building the vertex buffer with the attributes per instance
        let mut total_blocks = 0;
        let mut skipped_blocks = 0;
        self.instance_buff = {
            let mut data: Vec<Attr> = vec![];
            let cube_size = 1.0;
            for l in 0..self.chunk.layers.len()
            {
                for r in 0..self.chunk.layers[l].layer.len()
                {
                    for c in 0..self.chunk.layers[l].layer[r].len()
                    {
                        let block = &self.chunk.layers[l].layer[r][c];
                        
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
                            self.chunk.layers[l].layer[r][c].visiable = false;
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

            println!("total blocks: {}\nskipped blocks: {}\nrendering {} cubes", total_blocks, skipped_blocks, data.len());
            Some(glium::vertex::VertexBuffer::dynamic(display, &data).unwrap())
        };
    }

    pub fn render_scene(self: &mut ChunkDemoScene, assets: &mut AssetLib, 
                            target: &mut glium::Frame, view: &glm::Mat4)
    {
        if self.instance_buff.is_none()
        {
            return;
        }

        use glium::Surface;
        let block_mesh = assets.get_mesh("assets/Cube/BasicCube.obj").unwrap();
        let grass_tex = assets.get_texture("assets/textures/Grass.png", Flip::NONE).unwrap();
        let dirt_tex = assets.get_texture("assets/textures/Dirt.png", Flip::NONE).unwrap();
        let stone_tex = assets.get_texture("assets/textures/Stone.png", Flip::NONE).unwrap();
        let program = assets.get_program("Blocks_instanced", "assets/shaders/block_instanced.vert", "assets/shaders/block.frag").unwrap();

        
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        // uniform
        let light = [-1.0, 0.4, 0.9f32];
        
        self.grid.view = *view;
        self.grid.draw(target);

        let uniforms = &uniform! 
        { 
            model: mat4_to_array(&glm::Mat4::identity()), 
            view: mat4_to_array(view),
            perspective: mat4_to_array(&self.perspective),
            u_light: light,
            tex1: grass_tex.get_texture(),
            tex2: dirt_tex.get_texture(),
            tex3: stone_tex.get_texture()
        };

        let instance_buff = match &self.instance_buff
        {
            Some(b) => b,
            None => panic!("NO INSTANCE BUFFER")
        };

        target.draw((&block_mesh.vb, instance_buff.per_instance().unwrap()),
                    &block_mesh.indices, &program.program, uniforms,
                    &params).unwrap();
    }

    fn has_neighbor_gap(self: &ChunkDemoScene, x: usize, y: usize, z: usize) -> bool
    {
        let width = self.chunk.layers[y].width;
        let height = self.chunk.height;
        let depth = self.chunk.layers[y].depth;

        if x == 0 || y == 0 || z == 0
            || x == width -1
            || y == height -1
            || z == depth -1
            {
                return true;
            }
        
        // if include_self && self.chunk.layers[y].layer[x][z].id < 1
        // {
        //     return true;
        // }

        // // left
        // if self.chunk.layers[y].layer[x - 1][z].id < 1
        // {
        //     return true;
        // }
        
        // // left top
        // if self.chunk.layers[y].layer[x - 1][z - 1].id < 1
        // {
        //     return true;
        // }

        // // top
        // if self.chunk.layers[y].layer[x][z - 1].id < 1
        // {
        //     return true;
        // }

        // // top right
        // if self.chunk.layers[y].layer[x + 1][z - 1].id < 1
        // {
        //     return true;
        // }

        // // right
        // if self.chunk.layers[y].layer[x + 1][z].id < 1
        // {
        //     return true;
        // }

        // // bottom right
        // if self.chunk.layers[y].layer[x + 1][z + 1].id < 1
        // {
        //     return true;
        // }
        
        // // bottom
        // if self.chunk.layers[y].layer[x][z + 1].id < 1
        // {
        //     return true;
        // }

        // // bottom left
        // if self.chunk.layers[y].layer[x - 1][z + 1].id < 1
        // {
        //     return true;
        // }

        return false;
    }

}