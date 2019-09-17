
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

        Ok( ChunkDemoScene { grid, chunk: WorldChunk::new(32, 32, 16), perspective: *perspective, instance_buff: None })
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
        self.chunk.layers[12].fill_with(1);
        self.chunk.layers[11].fill_with(2);
        self.chunk.layers[10].fill_with(2);

        for i in 0..10
        {
            self.chunk.layers[i].fill_with(3);
        }

        self.chunk.layers[12].layer[7][7].id = 0;
        self.chunk.layers[11].layer[7][7].id = 0;
        self.chunk.layers[10].layer[7][7].id = 0;
        self.chunk.layers[9].layer[7][7].id = 0;

        // building the vertex buffer with the attributes per instance
        let mut count = 0;
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
                        
                        // Check if this block is fully surrounded by neighbors
                        let mut bottom_check = false;
                        if l > 0
                        {
                            bottom_check = self.has_neighbor_gap(r, l - 1, c, true);
                        }
                        if !self.has_neighbor_gap(r, l, c, false)
                            && !bottom_check
                            && !self.has_neighbor_gap(r, l + 1, c, true)
                            {
                                self.chunk.layers[l].layer[r][c].id = 0;
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

            println!("rendering {} cubes", data.len());
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

    fn has_neighbor_gap(self: &ChunkDemoScene, x: usize, y: usize, z: usize, include_self: bool) -> bool
    {
        if include_self && self.chunk.layers[y].layer[x][z].id < 1
        {
            return true;
        }

        // Surrounding 8 viewed from the side
        // left
        if x > 0 && self.chunk.layers[y].layer[x - 1][z].id < 1
        {
            return true;
        }
        
        // left top
        if x > 0 && z > 0 && self.chunk.layers[y].layer[x - 1][z - 1].id < 1
        {
            return true;
        }

        // top
        if z > 0 && self.chunk.layers[y].layer[x][z - 1].id < 1
        {
            return true;
        }

        // top right
        if x < self.chunk.layers[y].layer.len() && z > 0 && self.chunk.layers[y].layer[x + 1][z - 1].id < 1
        {
            return true;
        }

        // right
        if x < self.chunk.layers[y].layer.len() && self.chunk.layers[y].layer[x + 1][z].id < 1
        {
            return true;
        }

        // bottom right
        if x < self.chunk.layers[y].layer.len() 
            && z < self.chunk.layers[y].layer[x + 1].len() 
            && self.chunk.layers[y].layer[x + 1][z + 1].id < 1
        {
            return true;
        }
        
        // bottom
        if z < self.chunk.layers[y].layer[x + 1].len() && self.chunk.layers[y].layer[x][z + 1].id < 1
        {
            return true;
        }

        // bottom left
        if x > 0 && z < self.chunk.layers[y].layer[x + 1].len() && self.chunk.layers[y].layer[x - 1][z + 1].id < 1
        {
            return true;
        }

        return false;
    }

}