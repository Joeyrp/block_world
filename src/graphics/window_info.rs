
pub struct Size
{
    pub width: i32,
    pub height: i32
}

pub struct Point
{
    pub x: i32,
    pub y: i32
}

#[allow(dead_code)]
pub struct WindowInfo
{
    pub position: Point,
    pub size: Size,
    pub center: Point
}

impl WindowInfo
{
    pub fn calculate_window_info(display: &glium::Display) -> WindowInfo
    {
        let size = display.gl_window().window().get_inner_size().unwrap();
        let pos = display.gl_window().window().get_position().unwrap();
        
        let center = Point { x: (pos.x + (size.width / 2.0)) as i32, 
                            y: (pos.y + (size.height / 2.0)) as i32 };

        WindowInfo { position: Point{ x: pos.x as i32, y: pos.y as i32}, 
                    size: Size { width: size.width as i32, height: size.height as i32 }, center }
    } 
}