

#[derive(Copy, Clone, Debug)]
pub struct ChunkGeneration
{
    pub noise_type: NoiseType,
    pub offset: (f32, f32),
    pub zoom_factor: f32,
    pub threshold: f32,
    pub threshold_falloff: i32,
    pub octaves: i32,
    pub bias: f32,
    pub seed: Option<[u8; 32]>
}

#[derive(Copy, Clone, Debug)]
pub struct DebugSettings
{
    pub print_help: bool,
    pub print_chunk_info: bool,
    pub remake_test_scene: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct GameData
{
    pub debug: DebugSettings,
    pub chunk_generation: ChunkGeneration
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NoiseType
{
    RANDOM_2D,
    RANDOM_3D,
    OLC,
    SIMPLEX_2D,
    SIMPLEX_3D
}