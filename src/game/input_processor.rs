
use crate::graphics::{WindowInfo, CameraFPS};
use crate::game::{GameData, NoiseType, InputManager};

#[cfg(windows)]
use crate::win_input::{KeyCode, Mouse};

pub struct InputProcessor
{

}

impl InputProcessor
{
    pub fn process_input(dt: f64, cam: &mut CameraFPS, window_info: &WindowInfo, input_manager: &mut InputManager, game_data: &mut GameData) -> bool
    {
        // Handle Mouse Movement
    let mouse_state = Mouse::get_state();

    // TODO: Need a way to check if the button was just released
    //       so we can reset the mouse position and not use
    //       its offsets
    if !mouse_state.left_button
    {
        let delta_x = window_info.center.x - mouse_state.coords.x;
        let delta_y = window_info.center.y - mouse_state.coords.y;

        cam.apply_look_offset(delta_x as f32, delta_y as f32);
    
        Mouse::set_position(window_info.center.x, window_info.center.y);
    }
    //  

    let mut speed = 35.0 * dt as f32;

    if input_manager.key_down(KeyCode::LSHIFT)
    {
        speed = speed * 2.0;
    }

    // reprint help messsage
    if input_manager.key_pressed(KeyCode::F1)
    {
        game_data.print_help = !game_data.print_help;
    }

    // print chunk info
    if input_manager.key_pressed(KeyCode::F2)
    {
        game_data.print_chunk_info = !game_data.print_chunk_info;
    }

    // Camera movement
    if input_manager.key_down(KeyCode::Escape)
    {
        return true;
    }

    if input_manager.key_down(KeyCode::W)
    {
        cam.move_forward(-speed);
    }

    if input_manager.key_down(KeyCode::S)
    {
        cam.move_forward(speed);
    }

    if input_manager.key_down(KeyCode::A)
    {
        cam.move_right(-speed);
    }

    if input_manager.key_down(KeyCode::D)
    {
        cam.move_right(speed);
    }

    if input_manager.key_down(KeyCode::E)
    {
        cam.move_up(speed);
    }

    if input_manager.key_down(KeyCode::Q)
    {
        cam.move_up(-speed);
    }

    // Octaves
    if input_manager.key_pressed(KeyCode::SPACE)
    {
        game_data.octaves += 1;
        if game_data.octaves > 6
        {
            game_data.octaves = 1;
        }

        game_data.remake_test_scene = true;
    }

    // Seed generation
    if input_manager.key_pressed(KeyCode::C)
    {
        let mut seed: [u8; 32] = [0; 32];
        for i in 0..32
        {
            seed[i] = rand::random::<u8>();
        }
        
        game_data.seed = Some(seed);
        game_data.remake_test_scene = true;
    }

    if input_manager.key_pressed(KeyCode::V)
    {
        game_data.seed = Some([0; 32]);
        game_data.remake_test_scene = true;
    }

    // Zoom Factor/Bias
    if input_manager.key_down(KeyCode::R)
    {
        let ntype = game_data.noise_type;

        // use a closure to avoid duplicating this code
        // a function would work too but then we'd need
        // to pass in the game_data and input_manager

        let mut inc_zoom = || -> _ {
            if input_manager.key_down(KeyCode::LSHIFT)
            {
                game_data.zoom_factor += 0.005;
            }
            else
            {
                game_data.zoom_factor += 0.0005;
            }
        };

        match ntype
        {

            NoiseType::SIMPLEX_2D => { inc_zoom() },
            NoiseType::SIMPLEX_3D => { inc_zoom() },
            
            NoiseType::OLC =>
            {
                game_data.bias += 0.05;
            }

            _ => ()
        };

        game_data.remake_test_scene = true;
    }

    if input_manager.key_down(KeyCode::F)
    {
        let ntype = game_data.noise_type;

        // use a closure to avoid duplicating this code
        let mut dec_zoom = || -> _ {
                if input_manager.key_down(KeyCode::LSHIFT)
                {
                    game_data.zoom_factor -= 0.005;
                }
                else
                {
                    game_data.zoom_factor -= 0.0005;
                }

                if game_data.zoom_factor < 0.0005
                {
                    game_data.zoom_factor = 0.00005;
                }
        };

        match ntype
        {
            NoiseType::OLC =>
            {
                game_data.bias -= 0.05;

                if game_data.bias < 0.2
                {
                    game_data.bias = 0.2;
                }
            },

            NoiseType::SIMPLEX_2D => dec_zoom(),
            NoiseType::SIMPLEX_3D => dec_zoom(),

            _ => ()
        };

        game_data.remake_test_scene = true;
    }

    // Threshold
    if input_manager.key_down(KeyCode::T)
    {
        if input_manager.key_down(KeyCode::LSHIFT)
        {
            game_data.threshold += 0.01;
        }
        else 
        {
            game_data.threshold += 0.001;
        }

        game_data.remake_test_scene = true;
    }

    if input_manager.key_down(KeyCode::G)
    {
        if input_manager.key_down(KeyCode::LSHIFT)
        {
            game_data.threshold -= 0.01;
        }
        else 
        {
            game_data.threshold -= 0.001;
        }

        if game_data.threshold < 0.0
        {
            game_data.threshold = 0.0;
        }

        game_data.remake_test_scene = true;
    }

    // Threshold Falloff
    if input_manager.key_down(KeyCode::Y)
    {
        if input_manager.key_down(KeyCode::LSHIFT)
        {
            game_data.threshold_falloff += 5;
        }
        else 
        {
            game_data.threshold_falloff += 1;
        }

        game_data.remake_test_scene = true;
    }

    if input_manager.key_down(KeyCode::H)
    {
        if input_manager.key_down(KeyCode::LSHIFT)
        {
            game_data.threshold_falloff -= 5;
        }
        else 
        {
            game_data.threshold_falloff -= 1;
        }

        if game_data.threshold_falloff < 1
        {
            game_data.threshold_falloff = 1;
        }

        game_data.remake_test_scene = true;
    }

    // Noise modes: 
    if input_manager.key_pressed(KeyCode::NUM1)
    {
        game_data.noise_type = NoiseType::RANDOM_2D;
        game_data.remake_test_scene = true;
    }

    if input_manager.key_pressed(KeyCode::NUM2)
    {
        game_data.noise_type = NoiseType::RANDOM_3D;
        game_data.remake_test_scene = true;
    }

    if input_manager.key_pressed(KeyCode::NUM3)
    {
        game_data.noise_type = NoiseType::OLC;
        game_data.remake_test_scene = true;
    }

    if input_manager.key_pressed(KeyCode::NUM4)
    {
        game_data.noise_type = NoiseType::SIMPLEX_2D;
        game_data.remake_test_scene = true;
    }

    if input_manager.key_pressed(KeyCode::NUM5)
    {
        game_data.noise_type = NoiseType::SIMPLEX_3D;
        game_data.remake_test_scene = true;
    }

    return false;
    }
}