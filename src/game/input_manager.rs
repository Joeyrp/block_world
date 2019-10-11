
use crate::win_input::{KeyBoard, KeyCode};

#[allow(dead_code)]
pub struct InputManager
{
    key_states: [bool; 256],
}

impl InputManager
{
    pub fn new() -> InputManager
    {
        InputManager { key_states: [false; 256] }
    }

    pub fn key_down(self: &InputManager, key: KeyCode) -> bool
    {
        KeyBoard::key_down(key)
    }


    pub fn key_pressed(self: &mut InputManager, key: KeyCode) -> bool
    {
        let down_now = KeyBoard::key_down(key);
        if down_now && self.key_states[key as usize] == false
        {
            self.key_states[key as usize] = true;
            return true;
        }
        
        if !down_now
        {
            self.key_states[key as usize] = false;
        }

        return false;
    }
}