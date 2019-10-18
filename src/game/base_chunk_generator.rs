

use crate::game::{ChunkGenerator, WorldChunk, ChunkGeneration, NoiseType, world_chunk::Voxel};
use crate::utils::{SimplexNoise};


pub struct BaseChunkGenerator
{
    noise: SimplexNoise
}

impl BaseChunkGenerator
{
    pub fn new(seed: Option<[u8; 32]>) -> BaseChunkGenerator
    {
        BaseChunkGenerator { noise: SimplexNoise::new(seed) }
    }

    #[allow(non_snake_case)]
    fn generate_2D(&self, chunk: &mut WorldChunk, chunk_coords: [f32; 2], generation_settings: &ChunkGeneration)
    {
       // println!("Generating new chunk!");
        chunk.make_empty();

        for x in 0..chunk.width
        {
            for z in 0..chunk.depth
            {
                // Zoom into the noise by scaling down the x and z
                // (or if zoom_factor is large than 1 it will scale up - resulting in chaotic noise)
                let xf = (x as f32 + chunk_coords[0]) * generation_settings.zoom_factor;
                let zf = (z as f32 + chunk_coords[1]) * generation_settings.zoom_factor;

                let noise_value = self.noise.noise_2D(xf, zf, generation_settings.sx_scale);
                
                
                // Result of the noise is between -1 and 1. 
                // Need to scale it to be between 0 and 1.
                let noise_value = (noise_value + 1.0) / 2.0;
                let mut final_height = (1.0 + noise_value * ((chunk.height - 1) as f32)) as i32;

                if final_height >= chunk.height as i32
                {
                    // println!("height_scale: {} -- final_height: {}", height_scale, final_height);
                    // TODO: Log chunk generation warning
                    final_height = (chunk.height - 1) as i32;
                }

                // fill chunk column up to height
                for i in 0..(final_height + 1)
                {
                    let value = match i
                    {
                        0...7 => 3,
                        8...10 => 2,
                        _ => 1
                    };

                    // heigth - i is a hack to put the grass on the top and the stone on the bottom of the chunk
                    chunk.layers[i as usize].layer[x][z] = Voxel { id: value, visible: true };
                }
            }
        }
    }

    #[allow(non_snake_case)]
    pub fn generate_3D(&self, chunk: &mut WorldChunk, chunk_coords: [f32; 2], generation_settings: &ChunkGeneration)
    {
        chunk.make_empty();

        for y in 0..chunk.height
        {
            for x in 0..chunk.width
            {
                for z in 0..chunk.depth
                {
                    // Zoom into the noise by scaling down the x and z
                    // (or if zoom_factor is large than 1 it will scale up - resulting in chaotic noise)
                    let xf = (x as f32 + chunk_coords[0]) * generation_settings.zoom_factor;
                    let yf = y as f32 * generation_settings.zoom_factor;
                    let zf = (z as f32 + chunk_coords[1]) * generation_settings.zoom_factor;

                    let noise_value = self.noise.noise_3D(xf, yf, zf, generation_settings.sx_scale);
                                 
                    // Result of the noise is between -1 and 1. 
                    // Need to scale it to be between 0 and 1.
                    let noise_value = (noise_value + 1.0) / 2.0;

                    // use noise_value to decide on the block type
                    // threshold will decide if the block should be created or not
                    
                    // Function to increase threshold as height increases
                    // final = threshold + ((y^2) / 100) / falloff
                    let sqy = (y*y) as i32;
                    let sqy = sqy / 100;
                    let final_threshold = generation_settings.threshold + ((sqy as f32) / generation_settings.threshold_falloff as f32);

                    // ID 0 is an air block (no block)
                    let mut v = Voxel { id: 0, visible: true };

                    if noise_value >= final_threshold
                    {
                        v.id = match y
                        {
                            0...7 => 3,
                            8...10 => 2,
                            _ => 1
                        };
                    }

                    chunk.layers[y as usize].layer[x][z] = v;                    
                }
            }
        }
    }
}

impl ChunkGenerator for BaseChunkGenerator
{
    fn generate(&mut self, chunk: &mut WorldChunk, chunk_coords: [f32; 2], generation_settings: &ChunkGeneration)
    {
        match generation_settings.noise_type
        {
            NoiseType::SIMPLEX_2D => self.generate_2D(chunk, chunk_coords, generation_settings),
            NoiseType::SIMPLEX_3D => self.generate_3D(chunk, chunk_coords, generation_settings),
            _ => (),
        };
    }
}