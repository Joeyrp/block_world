
use std::{thread, time, rc::Rc};

#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;
extern crate image;

use glium::{glutin, Surface};
use glutin::dpi::LogicalPosition;

// Local Modules
pub mod utils;
use utils::FrameTracker;

#[cfg(windows)]
mod win_input;

#[cfg(windows)]
use win_input::{KeyBoard, KeyCode, Mouse};

mod graphics;
use graphics::{Gl, WindowInfo, CameraFPS, GridPlane, Mesh, Program, Texture, Flip};

mod game;
use game::{AssetLib, /* ObjectDemoScene ,*/ ChunkDemoScene, WorldChunk};
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

    // Camera setup
    let mut camera = CameraFPS::new();
    camera.move_up(3.0);
    camera.move_right(7.0);
    camera.move_forward(10.0);
    camera.apply_look_offset(200.0, 0.0);

    // The asset library for the game
    let mut asset_lib = AssetLib::new(&display);

    // Scene for demoing/debugging game objects
    //let mut obj_demo_scene = ObjectDemoScene::new(&mut asset_lib, &display, &perspective).unwrap();
    let mut chunk_test_scene = ChunkDemoScene::new(&mut asset_lib, display.clone(), &perspective).unwrap();
    chunk_test_scene.make_noise_test();
    
    
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
            closed = check_input(&mut camera, &window_info);
        }
        else
        {
            display.gl_window().window().hide_cursor(false);
        }
        //

        ////////////////////
        // Update Game
        chunk_test_scene.update(utils::micros_to_seconds(frame_tracker.get_delta_time()));
        //

        /////////////////////
        // Render frame
        let mut target = display.draw();
        target.clear_color_and_depth((0.2, 0.2, 0.3, 1.0), 1.0);

        // render objects
        // obj_demo_scene.render_scene(&mut asset_lib, &mut target, &camera.get_view());
        chunk_test_scene.render_scene(&mut asset_lib, &mut target, &camera.get_view());

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
#[cfg(windows)]
fn check_input(cam: &mut CameraFPS, window_info: &WindowInfo) -> bool
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

    // Keyboard input
    if KeyBoard::key_is_pressed(KeyCode::Escape)
    {
        return true;
    }

    if KeyBoard::key_is_pressed(KeyCode::W)
    {
        cam.move_forward(-0.05);
    }

    if KeyBoard::key_is_pressed(KeyCode::S)
    {
        cam.move_forward(0.05);
    }

    if KeyBoard::key_is_pressed(KeyCode::A)
    {
        cam.move_right(-0.05);
    }

    if KeyBoard::key_is_pressed(KeyCode::D)
    {
        cam.move_right(0.05);
    }

    if KeyBoard::key_is_pressed(KeyCode::E)
    {
        cam.move_up(0.05);
    }

    if KeyBoard::key_is_pressed(KeyCode::Q)
    {
        cam.move_up(-0.05);
    }

    return false;
}


#[cfg(not(windows))]
fn check_input(cam: &mut CameraFPS, mouse_lock_point: &Point) -> bool
{
    true
}

