

use rand::{ /* prelude::*, */ Rng, rngs::StdRng, SeedableRng};
use crate::game::{ChunkGenerator, WorldChunk, ChunkGeneration, NoiseType, world_chunk::Voxel};

pub struct RandomChunkGenerator
{
    rng: StdRng,
}

impl RandomChunkGenerator
{
    pub fn new(seed: Option<[u8; 32]>) -> RandomChunkGenerator
    {
        let seed = match seed
        {
            Some(s) => s,
            None => RandomChunkGenerator::get_system_seed(),
        };

        RandomChunkGenerator { rng: SeedableRng::from_seed(seed) }
    }

    fn get_system_seed() -> [u8; 32]
    {
        let mut seed: [u8; 32] = [0; 32];
        for i in 0..32
        {
            seed[i] = rand::random::<u8>();
        }

        return seed;
    }

    #[allow(non_snake_case)]
    fn generate_2D(&mut self, chunk: &mut WorldChunk)
    {
        chunk.make_empty();
        for x in 0..chunk.width
        {
            for z in 0..chunk.depth
            {
                let noise_value = self.rng.gen::<f32>();
                
                let final_height = (1.0 + noise_value * ((chunk.height - 1) as f32)) as i32; 

                // fill chunk column up to height
                for i in 0..(final_height + 1)
                {
                    let value = match i
                    {
                        0...7 => 3,
                        8...10 => 2,
                        _ => 1
                    };

                    chunk.layers[i as usize].layer[x][z] = Voxel { id: value, visible: true };
                }
            }
        }
    }

    #[allow(non_snake_case)]
    fn generate_3D(&mut self, chunk: &mut WorldChunk, generation_settings: &ChunkGeneration)
    {
        chunk.make_empty();

        for y in 0..chunk.height
        {
            for x in 0..chunk.width
            {
                for z in 0..chunk.depth
                {
                    let noise_value = self.rng.gen::<f32>();

                    let mut v = Voxel { id: 0, visible: true };

                    if noise_value >= generation_settings.threshold
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

impl ChunkGenerator for RandomChunkGenerator
{
    fn generate(&mut self, chunk: &mut WorldChunk, _chunk_coords: [f32; 2], generation_settings: &ChunkGeneration)
    {
        match generation_settings.noise_type
        {
            NoiseType::RANDOM_2D => self.generate_2D(chunk),
            NoiseType::RANDOM_3D => self.generate_3D(chunk, generation_settings),
            _ => (),
        };
    }
}