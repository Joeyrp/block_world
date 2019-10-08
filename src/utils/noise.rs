
// Should remove unused_variables, unused_mut when module is complete
#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use rand::{ /* prelude::*, */ Rng, rngs::StdRng, SeedableRng};

pub struct OlcNoise
{
    width: i32,
    height: i32,
    seed2D: Vec<f32>,
    rng: StdRng,
}

impl OlcNoise
{
    pub fn new(width: i32, height: i32, seed: Option<[u8; 32]> ) -> OlcNoise
    {
        let seed = match seed
        {
            Some(s) => s,
            None => get_system_seed()
        };

        // Generate the 2D Noise Seed
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let size: usize = (width * height) as usize;
        let mut seed2D: Vec<f32> = vec![0.0; size];
        for i in 0..width
        {
            // TODO: This is supposed to be a value between 0 and 1
            let value = rng.gen::<f32>();
            seed2D[i as usize] = value;
        }

        OlcNoise { width, height, seed2D, rng  }
    }

    pub fn sample2D(self: &OlcNoise, x: i32, y: i32, octaves: i32, bias: f32) -> f32
    {
        // C++ code from OneLonerCoder
        // ========================================
        // float fNoise = 0.0f;
        // float fScaleAcc = 0.0f;
        // float fScale = 1.0f;

        // for (int o = 0; o < nOctaves; o++)
        // {
        //     int nPitch = nWidth >> o;
        //     int nSampleX1 = (x / nPitch) * nPitch;
        //     int nSampleY1 = (y / nPitch) * nPitch;
            
        //     int nSampleX2 = (nSampleX1 + nPitch) % nWidth;					
        //     int nSampleY2 = (nSampleY1 + nPitch) % nWidth;

        //     float fBlendX = (float)(x - nSampleX1) / (float)nPitch;
        //     float fBlendY = (float)(y - nSampleY1) / (float)nPitch;

        //     float fSampleT = (1.0f - fBlendX) * fSeed[nSampleY1 * nWidth + nSampleX1] + fBlendX * fSeed[nSampleY1 * nWidth + nSampleX2];
        //     float fSampleB = (1.0f - fBlendX) * fSeed[nSampleY2 * nWidth + nSampleX1] + fBlendX * fSeed[nSampleY2 * nWidth + nSampleX2];

        //     fScaleAcc += fScale;
        //     fNoise += (fBlendY * (fSampleB - fSampleT) + fSampleT) * fScale;
        //     fScale = fScale / fBias;
        // }

        // // Scale to seed range
        // fOutput[y * nWidth + x] = fNoise / fScaleAcc;

        // TODO: Sample_x1 and sample_y1 seem to always be zero
        //         run OLCs code and debug

        let mut noise: f32 = 0.0;
        let mut scale_accel: f32 = 0.0;
        let mut scale: f32 = 1.0;

        let width = self.width;

        for o in 0..octaves
        {
            let pitch: i32 = width >> o;
            let sample_x1: i32 = (x / pitch) * pitch;
            let sample_y1: i32 = (y / pitch) * pitch;

           // println!("x: {}, y: {}, pitch: {}", x, y, pitch);

            // if sample_y1 > 0 || sample_x1 > 0
            // {
            //     println!("pitch: {}, octave: {}, sample_x1: {}, sample_y1: {}", pitch, o, sample_x1, sample_y1);
            // }

            
            let sample_x2: i32 = (sample_x1 + pitch) % width;
            let sample_y2: i32 = (sample_y1 + pitch) % width;

            // if sample_y2 > 0 || sample_x2 > 0
            // {
            //     println!("pitch: {}, octave: {}, sample_x2: {}, sample_y2: {}", pitch, o, sample_x2, sample_y2);
            // }

            let blend_x: f32 = ((x - sample_x1) as f32) / (pitch as f32);
            let blend_y: f32 = ((y - sample_y1) as f32) / (pitch as f32);


            //println!("pitch: {}, sample_y1: {}, width: {}", pitch, sample_y1, width);
            let idx1 = (sample_y1 * width + sample_x1) as usize;
            let idx2 = (sample_y1 * width + sample_x2) as usize;

            let sample_t: f32 = (1.0 - blend_x) * self.seed2D[idx1] + blend_x * self.seed2D[idx2];

            let idx3 = (sample_y2 * width + sample_x1) as usize;
            let idx4 = (sample_y2 * width + sample_x2) as usize;

            let sample_b: f32 = (1.0 - blend_x) * self.seed2D[idx3] + blend_x * self.seed2D[idx4];

            scale_accel += scale;
            noise += (blend_y * (sample_b - sample_t) + sample_t) * scale;
            scale = scale / bias;
        }
        noise / scale_accel
    }
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