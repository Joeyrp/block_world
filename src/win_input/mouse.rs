

#[derive(Debug)]
pub struct Coord { pub x: i32, pub y: i32 }

#[derive(Debug)]
pub struct MouseState
{
    pub coords: Coord,
    pub left_button: bool,
    pub right_button: bool,
    pub middle_button: bool,
    pub button_one: bool,
    pub button_two: bool
}

pub struct Mouse
{
}

impl Mouse
{
    pub fn get_state() -> MouseState
    {
        use winapi::shared::windef::POINT;
        use winapi::um::{winuser, winuser::{ GetCursorPos, GetAsyncKeyState }};

        let point = &mut POINT { x: 0, y: 0 };
        unsafe 
        {
           GetCursorPos(point);
        }

        let b1: bool;
        let b2: bool;
        let b3: bool;
        let b4: bool;
        let b5: bool;

        unsafe
        {
           b1 = GetAsyncKeyState(winuser::VK_LBUTTON) as u32 & 0x8000 != 0;
           b2 = GetAsyncKeyState(winuser::VK_RBUTTON) as u32 & 0x8000 != 0;
           b3 = GetAsyncKeyState(winuser::VK_MBUTTON) as u32 & 0x8000 != 0;
           b4 = GetAsyncKeyState(winuser::VK_XBUTTON1) as u32 & 0x8000 != 0;
           b5 = GetAsyncKeyState(winuser::VK_XBUTTON2) as u32 & 0x8000 != 0;
        }

        MouseState { coords: Coord {x: point.x, y: point.y}, left_button: b1, right_button: b2, 
                    middle_button: b3, button_one: b4, button_two: b5 }
    }

    pub fn set_position(x: i32, y: i32)
    {
        use winapi::um::winuser::SetCursorPos;

        unsafe
        {
            SetCursorPos(x, y);
        }
    }
}