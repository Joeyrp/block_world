
use std::rc::Rc;
use crate::{ graphics::Gl, utils::mat4_to_array, GridPlane, AssetLib, Flip, 
                WorldChunk, game::world_chunk::Voxel, game::world_chunk::Attr, utils::OlcNoise };


pub struct ChunkDemoScene
{
    gl: Gl,
    grid: GridPlane,
    chunk: WorldChunk,
    perspective: glm::Mat4,
    chunk_instance: Option<Rc<glium::VertexBuffer<Attr>>>,
    force_chunk_regen: bool
}

impl ChunkDemoScene
{
    pub fn new(assets: &mut AssetLib, display: Gl, perspective: &glm::Mat4) 
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

        Ok( ChunkDemoScene { gl: display.clone(), grid, chunk: WorldChunk::new(32, 32, 32), 
                            perspective: *perspective, chunk_instance: None, force_chunk_regen: false })
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

    pub fn make_test_one(self: &mut ChunkDemoScene)
    {
        /*
            LAYERS: 16x16x16 = 4,096
            INTERIOR: 14x14x14 = 2,744

            SHOULD RENDER: 1,352
        */

        for i in 0..16
        {
            self.chunk.layers[i].fill_with((i % 3 + 1) as u16);
        }

        self.chunk.layers[15].layer[7][7].id = 0;
    }

    #[allow(non_snake_case)]
    pub fn make_noise2D_test(self: &mut ChunkDemoScene, octaves: i32, bias: f32, use_random_seed: bool)
    {
        //println!("Generating chunk from 2D noise");
        //println!("Octaves: {}, Bias: {}", octaves, bias);
        // Sample noise to generate a random chunk
        // Limit height by requiring larger sample values for higher blocks

        self.chunk.make_empty();

        // Hard code the seed to all zeros
        let mut seed: Option<[u8; 32]> = Some([0; 32]);

        if use_random_seed
        {
            seed = None;
        }

        // The noise generator
        let noise_machine = OlcNoise::new(self.chunk.width as i32, self.chunk.depth as i32, seed);

        // Only testing 2D noise to start
        // In this test the chunk will be solid (no caves)
        // but will have variable height
        for x in 0..self.chunk.width
        {
            for z in 0..self.chunk.depth
            {
                // olc noise does not use x and y between 0 and 1
               // let fx: f32 = (x as f32) / (self.chunk.width as f32);
                //let fz: f32 = (z as f32) / (self.chunk.depth as f32);
                let height_scale = noise_machine.sample2D(x as i32, z as i32, octaves, bias);
                //println!("Noise sample at ({}, {}): {}", x, z, height_scale);
                
                // use height_scale to lerp between 1 and 32
                // a + x * (b - a)
                // Obviously can be simplified
                let final_height = (1.0 + height_scale * ((32 - 1) as f32)) as i32; 
                //println!("height_scale: {} -- final_height: {}", height_scale, final_height);

                // fill chunk column up to height
                for i in 0..(final_height + 1)
                {
                    let value = match i
                    {
                        0...7 => 3,
                        8...9 => 2,
                        _ => 1
                    };

                    // heigth - i is a hack to put the grass on the top and the stone on the bottom of the chunk
                    self.chunk.layers[i as usize].layer[x][z] = Voxel { id: value, visible: true };
                }
            }
        }

        let mut seed_out = String::from("Default - All Zeros");
        if use_random_seed
        {
            seed_out = String::from("Random");
        }

        println!("\nNew chunk generated with seed: {}, num octaves: {}, bias: {}", seed_out, octaves, bias);
        self.force_chunk_regen = true;
    }

    pub fn update(self: &mut ChunkDemoScene, _delta_time: f64)
    {
        // The instance buffer must be created before drawing begins
        // so this cannot happen in render_scene()
        self.chunk_instance = Some(self.chunk.get_instance_buffer(&self.gl, self.force_chunk_regen));
        self.force_chunk_regen = false;
    }

    pub fn render_scene(self: &mut ChunkDemoScene, assets: &mut AssetLib, 
                            target: &mut glium::Frame, view: &glm::Mat4)
    {
        if self.chunk.instance_buff.is_none()
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

        // Grid plane
        self.grid.view = *view;
        self.grid.draw(target);

        // uniforms
        let light = [-1.0, 0.4, 0.9f32];
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

        let instance_buff = match &self.chunk_instance
        {
            Some(ci) => ci,
            None => panic!("ERROR MISSING CHUNK INSTANCE BUFFER")
        };

        // Draw chunk
        target.draw((&block_mesh.vb, instance_buff.per_instance().unwrap()),
                    &block_mesh.indices, &program.program, uniforms,
                    &params).unwrap();
    }

}