
#![allow(dead_code)]

pub use self::frame_tracker::FrameTracker;
pub use self::noise::OlcNoise;
pub use self::noise::SimplexNoise;

mod frame_tracker;
mod noise;

use std::{time::Instant};
extern crate nalgebra_glm as glm;


const ONE_MILLION: u128 = 1000000;
const ONE_THOUSAND: u128 = 1000;

/// Converts a glm::Mat4 to a raw array
pub fn mat4_to_array(mat: &glm::Mat4) -> [[f32; 4]; 4]
{
    [
        [mat.m11, mat.m21, mat.m31, mat.m41],
        [mat.m12, mat.m22, mat.m32, mat.m42],
        [mat.m13, mat.m23, mat.m33, mat.m43],
        [mat.m14, mat.m24, mat.m34, mat.m44], 
    ]
}

/// Returns the elapsed time since a given Instant as an f64 with microsecond resolution.
/// The non-fractional part is the elapsed time in seconds.
pub fn time_since_micros(start: &Instant) -> f64
{
    let delta_time = Instant::now() - *start;
    (delta_time.as_micros() as f64) / (ONE_MILLION as f64)
}

/// Returns the elapsed time since a given Instant as an f64 with milisecond resolution.
/// The non-fractional part is the elapsed time in seconds.
pub fn time_since_millis(start: &Instant) -> f64
{
    let delta_time = Instant::now() - *start;
    (delta_time.as_millis() as f64) / (ONE_THOUSAND as f64)
}

/// Converts microseconds to seconds and returns the result as an f64.
/// The fractional part is the remaining microseconds.
pub fn micros_to_seconds(micros: u128) -> f64
{
    (micros as f64) / (ONE_MILLION as f64)
}


pub fn print_mat(mat: &glm::Mat4)
{
    let ar_mat = mat4_to_array(mat);
    println!("Matrix: ");
    println!("[");
    println!("\t[{}, {}, {}, {}]", ar_mat[0][0], ar_mat[0][1], ar_mat[0][2], ar_mat[0][3]);
    println!("\t[{}, {}, {}, {}]", ar_mat[1][0], ar_mat[1][1], ar_mat[1][2], ar_mat[1][3]);
    println!("\t[{}, {}, {}, {}]", ar_mat[2][0], ar_mat[2][1], ar_mat[2][2], ar_mat[2][3]);
    println!("\t[{}, {}, {}, {}]", ar_mat[3][0], ar_mat[3][1], ar_mat[3][2], ar_mat[3][3]);
    println!("]");

    let matrix = [
        [0.01, 0.0, 0.0, 0.0],
        [0.0, 0.01, 0.0, 0.0],
        [0.0, 0.0, 0.01, 0.0],
        [0.0, 0.0, 10.0, 1.0f32]
    ];
    println!("\n control matrix:");
    println!("\t[{}, {}, {}, {}]", matrix[0][0], matrix[0][1], matrix[0][2], matrix[0][3]);
    println!("\t[{}, {}, {}, {}]", matrix[1][0], matrix[1][1], matrix[1][2], matrix[1][3]);
    println!("\t[{}, {}, {}, {}]", matrix[2][0], matrix[2][1], matrix[2][2], matrix[2][3]);
    println!("\t[{}, {}, {}, {}]", matrix[3][0], matrix[3][1], matrix[3][2], matrix[3][3]);
    println!("]");
}