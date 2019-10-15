
// use crate::graphics::{Gl, Texture, Program};
// use glyph_brush::{rusttype::*, *};
// use std::{
//     env,
//     ffi::CString,
//     io::{self, Write},
//     mem, ptr, str,
// };

// struct GlGlyphTexture 
// {
//     name: u32,
// }

// impl GlGlyphTexture {
//     fn new(gl: &glium::Display, width: u32, height: u32) -> Self 
//     {
//         let mut name: u32 = 0;
//         // Create a texture for the glyphs
//         // The texture holds 1 byte per pixel as alpha data
//         glium::texture::texture2d::with_format(gl, )

//         Self { name }
//     }
// }


// pub struct TextRenderer
// {
//     gl: Gl,
//     canvas_width: i32,
//     canvas_height: i32,
//     program: Program
// }

// impl TextRenderer
// {
//     pub fn new(gl: Gl, canvas_width: i32, canvas_height: i32) -> TextRenderer
//     {
//         TextRenderer { gl: gl.clone(), canvas_width, canvas_height }
//     }
// }

// #[rustfmt::skip]
// fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> [f32; 16] {
//     let tx = -(right + left) / (right - left);
//     let ty = -(top + bottom) / (top - bottom);
//     let tz = -(far + near) / (far - near);
//     [
//         2.0 / (right - left), 0.0, 0.0, 0.0,
//         0.0, 2.0 / (top - bottom), 0.0, 0.0,
//         0.0, 0.0, -2.0 / (far - near), 0.0,
//         tx, ty, tz, 1.0,
//     ]
// }