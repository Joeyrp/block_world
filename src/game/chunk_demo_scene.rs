
use std::rc::Rc;
use crate::{ graphics::Gl, utils::mat4_to_array, GridPlane, AssetLib, Flip, 
                WorldChunk, game::world_chunk::Voxel, game::world_chunk::Attr, utils::OlcNoise, utils::SimplexNoise };


pub struct ChunkDemoScene
{
    gl: Gl,
    grid: GridPlane,
    chunk: WorldChunk,
    perspective: glm::Mat4,
    chunk_instance: Option<Rc<glium::VertexBuffer<Attr>>>,
    force_chunk_regen: bool,
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

        let mut grid = GridPlane::new(&display, [0.75, 0.75, 0.75], 10.0, 100, 100).unwrap();
        grid.projection = *perspective;

        Ok( ChunkDemoScene { gl: display.clone(), grid, chunk: WorldChunk::new(64, 32, 64), 
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
    pub fn make_noise2D_test(self: &mut ChunkDemoScene, octaves: i32, bias: f32, seed: Option<[u8; 32]>)
    {
        //println!("Generating chunk from 2D noise");
        //println!("Octaves: {}, Bias: {}", octaves, bias);
        // Sample noise to generate a random chunk
        // Limit height by requiring larger sample values for higher blocks

        self.chunk.make_empty();

        // The noise generator
        //let noise_machine = OlcNoise::new(self.chunk.width as i32, self.chunk.depth as i32, seed);
        let noise_machine = OlcNoise::new(32 as i32, 32 as i32, seed);

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
                
                // use height_scale to lerp between 1 and the chunk height
                // a + x * (b - a)
                // Obviously can be simplified
                let final_height = (1.0 + height_scale * ((self.chunk.height - 1) as f32)) as i32; 
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

        println!("\nNew chunk generated with OLC Noise:\nseed: {:?}\nnum octaves: {}, bias: {}", seed, octaves, bias);
        self.force_chunk_regen = true;
    }

    #[allow(non_snake_case)]
    pub fn make_simplex_noise2D(self: &mut ChunkDemoScene, zoom_factor: f32, seed: Option<[u8; 32]>)
    {
        self.chunk.make_empty();

        // The noise generator
        let noise_machine = SimplexNoise::new(seed);

        // Only testing 2D noise to start
        // In this test the chunk will be solid (no caves)
        // but will have variable height
        for x in 0..self.chunk.width
        {
            for z in 0..self.chunk.depth
            {
                // Zoom into the noise by scaling down the x and z
                // (or if zoom_factor is large than 1 it will scale up - resulting in chaotic noise)
                let xf = x as f32 * zoom_factor;
                let zf = z as f32 * zoom_factor;

                let height_scale = noise_machine.noise_2D(xf, zf);
                
                
                // Result of the noise is between -1 and 1. Need to scale it to be between
                // 0 and 1:
                // NewValue = (((OldValue - OldMin) * (NewMax - NewMin)) / (OldMax - OldMin)) + NewMin

                //  or split into 3 lines:
                // OldRange = (OldMax - OldMin)  
                // NewRange = (NewMax - NewMin)  
                // NewValue = (((OldValue - OldMin) * NewRange) / OldRange) + NewMin
                let old_range = 2.0;
                let new_range = 1.0;
                let height_scale = ((height_scale + 1.0) * new_range) / old_range;
                let final_height = (1.0 + height_scale * ((self.chunk.height - 1) as f32)) as i32;
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
        println!("\nNew chunk generated with Simplex Noise:\n Seed: {:?}\nZoom Factor: {}", seed, zoom_factor);
        self.force_chunk_regen = true;
    }

    #[allow(non_snake_case)]
    pub fn make_simplex_noise3D(self: &mut ChunkDemoScene, zoom_factor: f32, threshold: f32, seed: Option<[u8; 32]>)
    {
        self.chunk.make_empty();

        // The noise generator
        let noise_machine = SimplexNoise::new(seed);

        // Only testing 2D noise to start
        // In this test the chunk will be solid (no caves)
        // but will have variable height
        for y in 0..self.chunk.height
        {
            for x in 0..self.chunk.width
            {
                for z in 0..self.chunk.depth
                {
                    // Zoom into the noise by scaling down the x and z
                    // (or if zoom_factor is large than 1 it will scale up - resulting in chaotic noise)
                    let xf = x as f32 * zoom_factor;
                    let yf = y as f32 * zoom_factor;
                    let zf = z as f32 * zoom_factor;

                    let noise_value = noise_machine.noise_3D(xf, yf, zf);
                    
                    
                    // Result of the noise is between -1 and 1. Need to scale it to be between
                    // 0 and 1:
                    let old_range = 2.0;
                    let new_range = 1.0;
                    let noise_value = ((noise_value + 1.0) * new_range) / old_range;

                    // use noise_value to decide on the block type
                    // threshold will decide if the block should be created or not
                    // uses the y (height) of the block to adjust the threshold
                    // the higher up the block the smaller the threshold 
                    // should help stop grass from going all the way to the top of the chunk
                    // TODO: tweak this threshold adjustment so it's not linear
                    // let threshold = 0.3;// * (0.05 * ((y + 1) as f32));
                    //println!("threshold: {}, y: {}", threshold, y);

                    let mut v = Voxel { id: 0, visible: true };

                    if noise_value >= threshold
                    {
                        v.id = match y
                        {
                            0...7 => 3,
                            8...9 => 2,
                            _ => 1
                        };
                    }

                    self.chunk.layers[y as usize].layer[x][z] = v; 
                    
                }
            }
        }
        
        println!("\nNew chunk generated with Simplex Noise:\n Seed: {:?}\nZoom Factor: {}\nThreshold: {}", seed, zoom_factor, threshold);
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