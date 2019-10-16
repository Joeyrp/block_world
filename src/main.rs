
use std::{thread, time, rc::Rc};

#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;
extern crate image;
extern crate glium_glyph;


use glium::{glutin, Surface};
use glutin::dpi::LogicalPosition;

// Local Modules
pub mod utils;
use utils::FrameTracker;

#[cfg(windows)]
mod win_input;

#[cfg(windows)]
use win_input::{Mouse};

mod graphics;
use graphics::{Gl, WindowInfo, CameraFPS, GridPlane, Mesh, Program, Texture, Flip};

mod game;
use game::{GameData, DebugSettings, ChunkGeneration, NoiseType, AssetLib, 
            InputManager, InputProcessor, /* ObjectDemoScene ,*/ ChunkDemoScene, WorldChunk};
//


///////////////////////////////////////////////
//      Main Function
///////////////////////////////////////////////
fn main() 
{
    // Window and OpenGL initialization
    let mut events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = Gl { inner: Rc::new(glium::Display::new(wb, cb, &events_loop).unwrap()) } ;

    display.gl_window().window().set_title("Block World");
    display.gl_window().window().set_position(LogicalPosition::new(400.0, 200.0));

    // Window info setup
    let mut window_info = WindowInfo::calculate_window_info(&display);
    let perspective = glm::perspective_lh(window_info.size.width as f32 / window_info.size.height as f32, 
                                            (60.0 as f32).to_radians(), 0.1, 1024.0);
    
    // Test font
    // let dejavu: &[u8] = include_bytes!("../assets/fonts/open-sans/OpenSans-Bold.ttf");
    // let fonts = vec![Font::from_bytes(dejavu).unwrap()];

    // let mut glyph_brush = GlyphBrush::new(&(*display.inner), fonts);

    // Camera setup
    let mut camera = CameraFPS::new();
    camera.move_up(35.0);
    camera.move_right(7.0);
    camera.move_forward(10.0);
    camera.apply_look_offset(200.0, 0.0);

    // The asset library for the game
    let mut asset_lib = AssetLib::new(&display);

    // Input manager
    let mut input_manager = InputManager::new();

    // Data for use with the game
    // let mut game_data = GameData { print_help: true, print_chunk_info: true, remake_test_scene: false, noise_type: NoiseType::SIMPLEX_2D, 
    //                                 zoom_factor: 0.01, threshold: 0.3, threshold_falloff: 20, octaves: 3, bias: 0.5, seed: Some([0; 32]) };
    let mut game_data = GameData { debug: DebugSettings { print_help: true, print_chunk_info: true, remake_test_scene: false }, 
                                    chunk_generation: ChunkGeneration { noise_type: NoiseType::SIMPLEX_2D, offset:(0.0, 0.0), zoom_factor: 0.01, 
                                                                        sx_scale: 32.0, threshold: 0.3, threshold_falloff: 20, 
                                                                        octaves: 3, bias: 0.5, seed: Some([0; 32]) } };

    // Scenes for demoing/debugging game systems
    //let mut obj_demo_scene = ObjectDemoScene::new(&mut asset_lib, &display, &perspective).unwrap();
    let mut chunk_test_scene = ChunkDemoScene::new(&mut asset_lib, display.clone(), &perspective).unwrap();

    chunk_test_scene.make_simplex_noise2D(&game_data);
    //
    
    ///////////////////////////////////////////////////////////
    // _ BEGIN FRAME LOOP
    // Setup Frame Loop Data
    let mut frame_tracker = FrameTracker::new();
    let mut window_focused = true;
    let mut closed = false;
    
    Mouse::set_position(window_info.center.x, window_info.center.y);
    while !closed 
    {
        //////////////////////
        //  Frame Timer
        frame_tracker.update();
        let delta_time = utils::micros_to_seconds(frame_tracker.get_delta_time());

        //////////////////////
        // Info in window title
        let fps = frame_tracker.get_current_fps();
        let average_time = utils::micros_to_seconds(frame_tracker.get_average_frame_time());
        let total_time = utils::micros_to_seconds(frame_tracker.get_elapsed_time());
        
        display.gl_window().window().set_title(&format!(
            "Block World - FPS: {} || Average Frame Time: {:.6} || Program Run Time: {:.3}", 
            fps, average_time, total_time));


        ////////////////////
        // Input
        if window_focused
        {
            display.gl_window().window().hide_cursor(true);
            //closed = check_input(delta_time, &mut camera, &window_info, &mut input_manager, &mut game_data);
            closed = InputProcessor::process_input_debug(delta_time, &mut camera, &window_info, &mut input_manager, &mut game_data);
        }
        else
        {
            display.gl_window().window().hide_cursor(false);
        }
        //

        ////////////////////
        // Update Game

        chunk_test_scene.update(&mut game_data, delta_time);
        //

        /////////////////////
        // Render frame
        let mut target = display.draw();
        target.clear_color_and_depth((0.2, 0.2, 0.3, 1.0), 1.0);

        // render objects
        // obj_demo_scene.render_scene(&mut asset_lib, &mut target, &camera.get_view());
        chunk_test_scene.render_scene(&mut asset_lib, &game_data, &window_info, &(*display.inner), &mut target, &camera.get_view());

        target.finish().unwrap();
        //

        //////////////////////
        // Window message loop
        // listing the events produced by application and waiting to be received
        events_loop.poll_events(|ev| 
        {
            match ev 
            {
                glutin::Event::WindowEvent { event, .. } => match event 
                {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::Focused(f) => 
                    {
                        window_focused = f;
                        Mouse::set_position(window_info.center.x, window_info.center.y);
                    },
                    glutin::WindowEvent::Resized(_) => window_info = WindowInfo::calculate_window_info(&display),
                    glutin::WindowEvent::Moved(_) => window_info = WindowInfo::calculate_window_info(&display),
                    _ => (),
                },
                _ => (),
            }
        });
        //

        ////////////////////
        // Return control to the cpu to avoid running at 100%
        thread::sleep(time::Duration::from_micros(1));
        //
    }
}

///////////////////////////////////////////////
//      Check Input Function
///////////////////////////////////////////////
// #[cfg(windows)]
// fn check_input(dt: f64, cam: &mut CameraFPS, window_info: &WindowInfo, input_manager: &mut InputManager, game_data: &mut GameData) -> bool
// {
    
// }


#[cfg(not(windows))]
fn check_input(cam: &mut CameraFPS, mouse_lock_point: &Point) -> bool
{
    true
}

#[allow(dead_code)]
fn print_controls()
{
    // println!("\nDemo Controls:\n\tWASD: Move\n\tE: Move Up\n\tQ: Move Down\n\tMouse Move: Look\n\t1, 2, 3, 4, 5: Change Noise Type (Random 2D), (Random 3D), (OLC), (Simplex 2D), (Simplex 3D)");
    // println!("\tR: Adjust Bias/Zoom Factor Up\n\tF: Adjust Bias/Zoom Factor Down\n\tT: Adjust Threshold Up\n\tG: Adjust Threshold Down\n\tSPACE: Increase Octave");
    // println!("\tY: Adjust Threshold Falloff Up\n\tH: Adjust Threshold Falloff Down\n\tV: Use Default Seed\n\tC: Use New Random Seed\n\tSHIFT: Move and Adjust Faster\n\tF1: Reprint this message\n");
    
}



