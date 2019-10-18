

use crate::game::{WorldChunk, ChunkGeneration};

pub trait ChunkGenerator
{
    /// Runs custom generation code on a given chunk at the given coordinates.
    /// chunk_coords should be an x,z pair describing where in the world grid the chunk is.
    fn generate(&self, chunk: &mut WorldChunk, chunk_coords: [f32; 2], generation_settings: &ChunkGeneration);
}