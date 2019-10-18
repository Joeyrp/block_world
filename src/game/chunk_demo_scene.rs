
use std::rc::Rc;
use glium_glyph::glyph_brush::{rusttype::Font, Section, rusttype::Scale};
use glium_glyph::GlyphBrush;
use crate::graphics::{ Gl, GridPlane, Flip, WindowInfo};
use crate::game::{AssetLib, WorldChunk, world_chunk::Voxel, world_chunk::Attr, GameData, game_data::NoiseType, 
                    ChunkGenerator, BaseChunkGenerator, RandomChunkGenerator};
use crate::utils::{OlcNoise, mat4_to_array};

pub struct ChunkDemoScene<'font, 'a>
{
    gl: Gl,
    grid: GridPlane,
    chunk: WorldChunk,
    perspective: glm::Mat4,
    chunk_instance: Option<Rc<glium::VertexBuffer<Attr>>>,
    force_chunk_regen: bool,
    glyph_brush: GlyphBrush<'font, 'a>
}

impl<'font, 'a> ChunkDemoScene<'font, 'a>
{
    pub fn new(assets: &mut AssetLib, display: Gl, perspective: &glm::Mat4) 
        -> Result<ChunkDemoScene<'font, 'a>, String>
    {
        // Pre Load assets
        assets.get_mesh("assets/Cube/BasicCube.obj")?;
        assets.get_texture("assets/textures/Grass.png", Flip::NONE)?;
        assets.get_texture("assets/textures/Dirt.png", Flip::NONE)?;
        assets.get_texture("assets/textures/Stone.png", Flip::NONE)?;
        assets.get_program("Blocks_instanced", "assets/shaders/block_instanced.vert", "assets/shaders/block.frag")?;

        let mut grid = GridPlane::new(&display, [0.75, 0.75, 0.75], 10.0, 100, 100).unwrap();
        grid.projection = *perspective;

        let dejavu: &[u8] = include_bytes!("../../assets/fonts/open-sans/OpenSans-Bold.ttf");
        let fonts = vec![Font::from_bytes(dejavu).unwrap()];

        let glyph_brush = GlyphBrush::new(&(*display.inner), fonts);

        Ok( ChunkDemoScene { gl: display.clone(), grid, chunk: WorldChunk::new(128, 32, 128), 
                            perspective: *perspective, chunk_instance: None, force_chunk_regen: false, glyph_brush })
    }

    pub fn get_chunk(self: &ChunkDemoScene<'font, 'a>) -> &WorldChunk
    {
        &self.chunk
    }

    pub fn make_chunk_single_layer(self: &mut ChunkDemoScene<'font, 'a>)
    {
        self.chunk.layers[8].fill_with(1);
    }

    pub fn make_chunk_single_layer_with_hole(self: &mut ChunkDemoScene<'font, 'a>)
    {
        self.chunk.layers[5].fill_with(1);

        self.chunk.layers[5].layer[8][8].id = 0;
    }

    pub fn make_test_one(self: &mut ChunkDemoScene<'font, 'a>)
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
    pub fn make_noise2D_test(self: &mut ChunkDemoScene<'font, 'a>,  game_data: &GameData)
    {
        //println!("Generating chunk from 2D noise");
        //println!("Octaves: {}, Bias: {}", octaves, bias);
        // Sample noise to generate a random chunk
        // Limit height by requiring larger sample values for higher blocks

        self.chunk.make_empty();

        // The noise generator
        //let noise_machine = OlcNoise::new(self.chunk.width as i32, self.chunk.depth as i32, seed);
        let noise_machine = OlcNoise::new(32 as i32, 32 as i32, game_data.chunk_generation.seed);

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
                let height_scale = noise_machine.sample2D(x as i32, z as i32, game_data.chunk_generation.octaves, game_data.chunk_generation.bias);
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

        // println!("\nNew chunk generated with OLC Noise:\nseed: {:?}\nnum octaves: {}, bias: {}", seed, octaves, bias);
        self.force_chunk_regen = true;
    }

   

    fn get_chunk_info_string(chunk: &WorldChunk, game_data: &GameData) -> String
    {
        let mut info = String::from("Chunk Info:\n");
        info += &String::from(format!("\nDimensions: ({}, {}, {})", chunk.width, chunk.height, chunk.depth));
        info += &String::from(format!("\nTotal Blocks: {}\nHidden Blocks: {}\nRendered Blocks: {}", chunk.total_blocks, chunk.hidden_blocks, chunk.rendered_blocks));
        info += &String::from(format!("\n\nNoise Type: {:?}", game_data.chunk_generation.noise_type));
        info += &String::from(format!("\n\nSeed: {:?}\n", game_data.chunk_generation.seed));

        info += &match game_data.chunk_generation.noise_type
        {
            NoiseType::RANDOM_2D => String::from(""),
            NoiseType::RANDOM_3D => String::from(format!("\nThreshold: {}", game_data.chunk_generation.threshold)),
            NoiseType::OLC => String::from(format!("\nOctaves: {}\nBias: {}", game_data.chunk_generation.octaves, game_data.chunk_generation.bias)),
            
            NoiseType::SIMPLEX_2D => String::from(format!("\nOffsets: ({}, {})\nZoom Factor: {}\nScale Factor: {}", game_data.chunk_generation.offset.0,
                                                             game_data.chunk_generation.offset.1, game_data.chunk_generation.zoom_factor, 
                                                             game_data.chunk_generation.sx_scale)),

            NoiseType::SIMPLEX_3D => String::from(format!("\nOffsets: ({}, {})\nZoom Factor: {}\nScale Factor: {}\nThreshold: {}\nThreshold Falloff: {}", 
                                                            game_data.chunk_generation.offset.0, game_data.chunk_generation.offset.1,
                                                            game_data.chunk_generation.zoom_factor, game_data.chunk_generation.sx_scale,  
                                                            game_data.chunk_generation.threshold, game_data.chunk_generation.threshold_falloff)),
        };

        info
    }

    fn get_scene_controls_string(game_data: &GameData) -> String
    {
        let mut controls_string = String::from("Demo Controls:\n\nF1: Show/Hide this message\nF2: Show/Hide Chunk Info");
        controls_string += "\n\nWASD: Move\nE/Q: Move Up/Down\nMouse Move: Look\nSHIFT: Move and Adjust Faster";
        controls_string += "\n\n1, 2, 3, 4, 5: Change Noise Type\nV: Use Default Seed\nC: Use New Random Seed";
        controls_string += "\n";
      //  \n\tR: Adjust Bias/Zoom Factor Up\n\tF: Adjust Bias/Zoom Factor Down\n\tT: Adjust Threshold Up\n\tG: Adjust Threshold Down
      //  \n\tY: Adjust Threshold Falloff Up\n\tH: Adjust Threshold Falloff Down\n\t"
    
        controls_string += match game_data.chunk_generation.noise_type
        {
            NoiseType::RANDOM_2D => "",
            NoiseType::RANDOM_3D => "\nT/G: Adjust Threshold Up/Down",
            NoiseType::OLC => "\nR/F: Adjust Bias Up/Down\nSPACE: Increase Octave",
            NoiseType::SIMPLEX_2D => "\nLeft Arrow, Right Arrow: Adjust Noise X Offset\nUp Arrow, Down Arrow: Adjust Noise Z Offset\nR/F: Adjust Zoom Factor Up/Down\nZ/X: Adjust Scale Up/Down",
            NoiseType::SIMPLEX_3D => "\nLeft Arrow, Right Arrow: Adjust Noise X Offset\nUp Arrow, Down Arrow: Adjust Noise Z Offset\nR/F: Adjust Zoom Factor Up/Down\nZ/X: Adjust Scale Up/Down\nT/G: Adjust Threshold Up/Down\nY/H: Adjust Threshold Falloff Up/Down",
        };

        controls_string
    }

    fn base_gen(&mut self, game_data: &mut GameData)
    {
        let mut generator = BaseChunkGenerator::new(game_data.chunk_generation.seed);
        let x = game_data.chunk_generation.offset.0;
        let z = game_data.chunk_generation.offset.1;
        generator.generate(&mut self.chunk, [x, z], &game_data.chunk_generation);
    }

    fn rand_gen(&mut self, game_data: &mut GameData)
    {
        let mut generator = RandomChunkGenerator::new(game_data.chunk_generation.seed);
        generator.generate(&mut self.chunk, [0.0; 2], &game_data.chunk_generation);
    }

    pub fn update(self: &mut ChunkDemoScene<'font, 'a>, game_data: &mut GameData, _delta_time: f64)
    {
        if game_data.debug.remake_test_scene
        {
            match game_data.chunk_generation.noise_type
            {
                NoiseType::RANDOM_2D => self.rand_gen(game_data),
                NoiseType::RANDOM_3D => self.rand_gen(game_data),

                NoiseType::SIMPLEX_2D => self.base_gen(game_data),
                NoiseType::SIMPLEX_3D => self.base_gen(game_data),

                NoiseType::OLC => self.make_noise2D_test(game_data),
            };
            
            self.force_chunk_regen = true;
            game_data.debug.remake_test_scene = false;
        }

        // The instance buffer must be created before drawing begins
        // so this cannot happen in render_scene()
        self.chunk_instance = Some(self.chunk.get_instance_buffer(&self.gl, self.force_chunk_regen));
        self.force_chunk_regen = false;
    }

    pub fn render_scene(self: &mut ChunkDemoScene<'font, 'a>, assets: &mut AssetLib, game_data: &GameData, window_info: &WindowInfo,
                            display: &glium::Display, target: &mut glium::Frame, view: &glm::Mat4)
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

        // On screen text info
        let test_scale = 18.0;
        if game_data.debug.print_help
        {              
            self.glyph_brush.queue(Section {
                text: &ChunkDemoScene::get_scene_controls_string(game_data),
                scale: Scale { x: test_scale, y: test_scale },
                screen_position: (50.0, 0.0),
                bounds: (window_info.size.width as f32, window_info.size.height as f32),
                ..Section::default()
            });
        }

        if game_data.debug.print_chunk_info
        {
            self.glyph_brush.queue(Section {
                text: &ChunkDemoScene::get_chunk_info_string(self.get_chunk(), &game_data),
                scale: Scale { x: test_scale, y: test_scale },
                screen_position: (window_info.size.width as f32 / 2.0 + 200.0, 0.0),
                bounds: (250.0, window_info.size.height as f32 / 2.0),
                ..Section::default()
            });
        }

        self.glyph_brush.draw_queued(display, target);
    }

}