

pub struct GameData
{
    pub print_help: bool,
    pub print_chunk_info: bool,
    pub remake_test_scene: bool,
    pub noise_type: NoiseType,
    pub zoom_factor: f32,
    pub threshold: f32,
    pub threshold_falloff: i32,
    pub octaves: i32,
    pub bias: f32,
    pub seed: Option<[u8; 32]>
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub enum NoiseType
{
    RANDOM_2D,
    RANDOM_3D,
    OLC,
    SIMPLEX_2D,
    SIMPLEX_3D
}