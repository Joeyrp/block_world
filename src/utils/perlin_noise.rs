
use rand::{ /* prelude::*, */ rngs::StdRng, SeedableRng};


pub struct PerlinNoise
{
    seed: [u8; 32],
    rng: StdRng,
}

impl PerlinNoise
{
    pub fn new(seed: Option<[u8; 32]> ) -> PerlinNoise
    {
        let seed = match seed
        {
            Some(s) => s,
            None => get_system_seed()
        };

        PerlinNoise { seed, rng: SeedableRng::from_seed(seed) }
    }

    // pub fn sample_at(x: f64, y: f64, z: f64) -> f64
    // {
        

    //     3.14159
    // }
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