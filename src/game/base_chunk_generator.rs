

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

                let height_scale = self.noise.noise_2D(xf, zf, generation_settings.sx_scale);
                
                
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
                let mut final_height = (1.0 + height_scale * ((chunk.height - 1) as f32)) as i32;

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
        // println!("\nNew chunk generated with Simplex Noise:\nSeed: {:?}\nZoom Factor: {}", seed, zoom_factor);
    }
}

impl ChunkGenerator for BaseChunkGenerator
{
    fn generate(&self, chunk: &mut WorldChunk, chunk_coords: [f32; 2], generation_settings: &ChunkGeneration)
    {
        match generation_settings.noise_type
        {
            NoiseType::SIMPLEX_2D => self.generate_2D(chunk, chunk_coords, generation_settings),
            _ => (),
        };
    }
}