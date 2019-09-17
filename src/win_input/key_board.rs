

use winapi::um::winuser;

#[allow(dead_code)]
pub enum KeyCode
{
    NUM0 = '0' as isize,
    NUM1 = '1' as isize,
    NUM2 = '2' as isize,
    NUM3 = '3' as isize,
    NUM4 = '4' as isize,
    NUM5 = '5' as isize,
    NUM6 = '6' as isize,
    NUM7 = '7' as isize,
    NUM8 = '8' as isize,
    NUM9 = '9' as isize,
    A = 'A' as isize,
    B = 'B' as isize,
    C = 'C' as isize,
    D = 'D' as isize,
    E = 'E' as isize,
    F = 'F' as isize,
    G = 'G' as isize,
    H = 'H' as isize,
    I = 'I' as isize,
    J = 'J' as isize,
    K = 'K' as isize,
    L = 'L' as isize,
    M = 'M' as isize,
    N = 'N' as isize,
    O = 'O' as isize,
    P = 'P' as isize,
    Q = 'Q' as isize,
    R = 'R' as isize,
    S = 'S' as isize,
    T = 'T' as isize,
    U = 'U' as isize,
    V = 'V' as isize,
    W = 'W' as isize,
    X = 'X' as isize,
    Y = 'Y' as isize,
    Z = 'Z' as isize,

    F1 = winuser::VK_F1 as isize,
    F2 = winuser::VK_F2 as isize,
    F3 = winuser::VK_F3 as isize,
    F4 = winuser::VK_F4 as isize,
    F5 = winuser::VK_F5 as isize,
    F6 = winuser::VK_F6 as isize,
    F7 = winuser::VK_F7 as isize,
    F8 = winuser::VK_F8 as isize,
    F9 = winuser::VK_F9 as isize,
    F10 = winuser::VK_F10 as isize, 
    F11 = winuser::VK_F11 as isize, 
    F12 = winuser::VK_F12 as isize, 
    SPACE = winuser::VK_SPACE  as isize,
    LCONTROL = winuser::VK_LCONTROL as isize,
    RCONTROL = winuser::VK_RCONTROL as isize,
    LSHIFT = winuser::VK_LSHIFT as isize,
    RSHIFT = winuser::VK_RSHIFT as isize,
    LMENU = winuser::VK_LMENU as isize,
    RMENU = winuser::VK_RMENU as isize,
    ENTER = winuser::VK_RETURN as isize,
    Escape = winuser::VK_ESCAPE as isize
}


pub struct KeyBoard
{

}

impl KeyBoard
{
    pub fn key_is_pressed(key: KeyCode) -> bool
    {
        use winapi::um::winuser::{ GetAsyncKeyState };
        let mut state = false;

        unsafe
        {
            if GetAsyncKeyState(key as i32) as u32 & 0x8000 != 0
            {
                state = true;
            }
        }

        return state;
    }
}