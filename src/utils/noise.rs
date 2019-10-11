
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
        for i in 0..size
        {
            // Generate a random value between 0 and 1
            let value = rng.gen::<f32>();
            seed2D[i as usize] = value;
        }

        OlcNoise { width, height, seed2D, rng  }
    }



    pub fn sample2D(self: &OlcNoise, x: i32, y: i32, octaves: i32, bias: f32) -> f32
    {
        // Algorithm by OneLoneCoder
        // https://github.com/OneLoneCoder/videos/blob/master/OneLoneCoder_PerlinNoise.cpp

        let mut noise: f32 = 0.0;
        let mut scale_accel: f32 = 0.0;
        let mut scale: f32 = 1.0;

        let width = self.width;

        let x = x % self.width;
        let y = y % self.height;

        for o in 0..octaves
        {
            let pitch: i32 = width >> o;
            let sample_x1: i32 = (x / pitch) * pitch;
            let sample_y1: i32 = (y / pitch) * pitch;

           // println!("x: {}, y: {}, width: {}, pitch: {}, octave: {}", x, y, width, pitch, o);

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

           // println!("Idx1: {}, Idx2: {}", idx1, idx2);

            let sample_t: f32 = (1.0 - blend_x) * self.seed2D[idx1] + blend_x * self.seed2D[idx2];

            let idx3 = (sample_y2 * width + sample_x1) as usize;
            let idx4 = (sample_y2 * width + sample_x2) as usize;

            //println!("Idx3: {}, Idx4: {}", idx3, idx4);

            let sample_b: f32 = (1.0 - blend_x) * self.seed2D[idx3] + blend_x * self.seed2D[idx4];

            scale_accel += scale;
            noise += (blend_y * (sample_b - sample_t) + sample_t) * scale;
            scale = scale / bias;
        }
        noise / scale_accel
    }
}

// Ported from a C implementation of Ken Perlin's simplex noise by Stefan Gustavson (stegu@itn.liu.se)
pub struct SimplexNoise
{
    perm: [i32; 512]
}

impl SimplexNoise
{
    pub fn new(seed: Option<[u8; 32]>) -> SimplexNoise
    {
        let mut perm = [151,160,137,91,90,15,
                    131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,
                    190, 6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,
                    88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,
                    77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,
                    102,143,54, 65,25,63,161, 1,216,80,73,209,76,132,187,208, 89,18,169,200,196,
                    135,130,116,188,159,86,164,100,109,198,173,186, 3,64,52,217,226,250,124,123,
                    5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,
                    223,183,170,213,119,248,152, 2,44,154,163, 70,221,153,101,155,167, 43,172,9,
                    129,22,39,253, 19,98,108,110,79,113,224,232,178,185, 112,104,218,246,97,228,
                    251,34,242,193,238,210,144,12,191,179,162,241, 81,51,145,235,249,14,239,107,
                    49,192,214, 31,181,199,106,157,184, 84,204,176,115,121,50,45,127, 4,150,254,
                    138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180,
                    151,160,137,91,90,15,
                    131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,
                    190, 6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,
                    88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,
                    77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,
                    102,143,54, 65,25,63,161, 1,216,80,73,209,76,132,187,208, 89,18,169,200,196,
                    135,130,116,188,159,86,164,100,109,198,173,186, 3,64,52,217,226,250,124,123,
                    5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,
                    223,183,170,213,119,248,152, 2,44,154,163, 70,221,153,101,155,167, 43,172,9,
                    129,22,39,253, 19,98,108,110,79,113,224,232,178,185, 112,104,218,246,97,228,
                    251,34,242,193,238,210,144,12,191,179,162,241, 81,51,145,235,249,14,239,107,
                    49,192,214, 31,181,199,106,157,184, 84,204,176,115,121,50,45,127, 4,150,254,
                    138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180 ];

        let seed = match seed
        {
            Some(s) => s,
            None => get_system_seed()
        };

        SimplexNoise::shuffle_perm(&mut perm, seed);

        SimplexNoise { perm }
    }

    fn shuffle_perm(perm: &mut [i32; 512], seed: [u8; 32])
    {
        let num_shuffles = 2048;
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        for _ in 0..num_shuffles
        {
            let i = (rng.gen::<u32>() % 512) as usize;
            let j = (rng.gen::<u32>() % 512) as usize;

            if i == j
            {
                continue;
            }

            let temp = perm[i];
            perm[i] = perm[j];
            perm[j] = temp;
        }
    }

// Note: C implementation by Stefan Gustavson (stegu@itn.liu.se)
/*
float  SimplexNoise1234::grad( int hash, float x, float y ) {
    int h = hash & 7;      // Convert low 3 bits of hash code
    float u = h<4 ? x : y;  // into 8 simple gradient directions,
    float v = h<4 ? y : x;  // and compute the dot product with (x,y).
    return ((h&1)? -u : u) + ((h&2)? -2.0f*v : 2.0f*v);
}
*/
    fn grad_2d(hash: i32, x: f32, y: f32) -> f32
    {
        let h = hash & 7;

        let mut u = y;
        if h < 4
        {
            u = x;
        }

        let mut v = x;
        if h < 4
        {
            v = y;
        }

        let mut f = 2.0 * v;
        if h&2 > 0
        {
            f = -2.0 * v;
        }
        //println!("u: {}, v: {}", u, v);

        if h&1 > 0
        {
            return -u + f;
        }

        return u + f;
    }

    #[allow(unused_assignments)]
    pub fn noise_2D(self: &SimplexNoise, x: f32, y: f32) -> f32
    {
         let F2: f32 = 0.366025403;
        let G2: f32 = 0.211324865;

        let mut n0 = 0.0;
        let mut n1 = 0.0;
        let mut n2 = 0.0;

        let s: f32 = (x + y) * F2;
        let xs = x + s;
        let ys = y + s;
    //  println!("xs: {}, ys: {}", xs, ys);

        let i = xs.floor() as i32;
        let j = ys.floor() as i32;

    //  println!("i: {}, j: {}", i, j);

        let t = (i + j) as f32 * G2;
        let X0 = i as f32 - t;
        let Y0 = j as f32 - t;
        let x0 = x - X0;
        let y0 = y - Y0;

        let mut i1 = 0;
        let mut j1 = 1;

        if x0 > y0
        {
            i1 = 1;
            j1 = 0;
        }

        let x1 = x0 - i1 as f32 + G2;
        let y1 = y0 - j1 as f32 + G2;
        let x2 = x0 - 1.0 + 2.0 * G2;
        let y2 = y0 - 1.0 + 2.0 * G2;

        let ii = i & 0xff;
        let jj = j & 0xff;

        let mut t0 = 0.5 - x0*x0-y0*y0;
        if t0 < 0.0
        {
            n0 = 0.0;
        }
        else
        {
            t0 *= t0;
            n0 = t0 * t0 * SimplexNoise::grad_2d(self.perm[(ii + self.perm[jj as usize]) as usize], x0, y0);
        }

        let mut t1 = 0.5 - x1*x1-y1*y1;
        if t1 < 0.0
        {
            n1 = 0.0;
        }
        else
        {
            t1 *= t1;
            n1 = t1 * t1 * SimplexNoise::grad_2d(self.perm[(ii + i1 + self.perm[(jj + j1) as usize]) as usize], x1, y1);
        }

        let mut t2 = 0.5 - x2*x2-y2*y2;
        if t2 < 0.0
        {
            n2 = 0.0;
        }
        else
        {
            t2 *= t2;
            n2 = t2 * t2 * SimplexNoise::grad_2d(self.perm[(ii + 1 + self.perm[(jj+1) as usize]) as usize], x2, y2);
        }

        40.0 * (n0 + n1 + n2)
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
