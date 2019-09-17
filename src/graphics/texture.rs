
extern crate image;

#[allow(dead_code)]
pub enum Flip
{
    NONE,
    VERTICAL,
    HORIZONTAL
}

pub struct Texture
{
    texture: glium::texture::Texture2d
}

impl Texture
{
    pub fn new(gl: &glium::Display, filename: &str, flip: Flip) -> Result<Texture, String>
    {
        let img = image::open(filename);
        let img = match img
        {
            Ok(i) => i,
            Err(e) => return Err(format!("Error Loading Texture! File: {}, Error: {:?}", filename, e))
        };

        let img_rgb = img.to_rgb(); 
        let width = img_rgb.width();
        let height = img_rgb.height();
        let raw: Vec<u8> = match flip
        {
            Flip::NONE => img.raw_pixels(),
            Flip::VERTICAL => img.flipv().raw_pixels(),
            Flip::HORIZONTAL => img.fliph().raw_pixels()
        };

        let image = match img.color()
        {
            image::ColorType::RGB(_x) => 
            {
                glium::texture::RawImage2d::from_raw_rgb(raw, (width, height))
            },
            image::ColorType::RGBA(_x) => 
            {
                glium::texture::RawImage2d::from_raw_rgba(raw, (width, height))
            },
            _ => return Err(String::from("UNKNOWN TEXTURE FORMAT!"))
        };


        let texture = glium::texture::Texture2d::new(gl, image).unwrap();
        Ok( Texture { texture })
    }

    pub fn get_texture(self: &Texture) -> &glium::texture::Texture2d
    {
        &self.texture
    }
}