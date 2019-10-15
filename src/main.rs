
use std::{thread, time, rc::Rc};

#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;
extern crate image;
extern crate glium_glyph;

use glium_glyph::glyph_brush::{rusttype::Font, Section, rusttype::Scale};
use glium_glyph::GlyphBrush;

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
use game::{GameData, NoiseType, AssetLib, InputManager, InputProcessor, /* ObjectDemoScene ,*/ ChunkDemoScene, WorldChunk};
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
    let dejavu: &[u8] = include_bytes!("../assets/fonts/DejaVuSans-2.37.ttf");
    let fonts = vec![Font::from_bytes(dejavu).unwrap()];

    let mut glyph_brush = GlyphBrush::new(&(*display.inner), fonts);

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
    let mut game_data = GameData { print_help: true, print_chunk_info: true, remake_test_scene: false, noise_type: NoiseType::SIMPLEX_2D, zoom_factor: 0.01, 
                                    threshold: 0.3, threshold_falloff: 20, octaves: 3, bias: 0.5, seed: Some([0; 32]) };

    // Scene for demoing/debugging game objects
    //let mut obj_demo_scene = ObjectDemoScene::new(&mut asset_lib, &display, &perspective).unwrap();
    let mut chunk_test_scene = ChunkDemoScene::new(&mut asset_lib, display.clone(), &perspective).unwrap();

    chunk_test_scene.make_simplex_noise2D(0.01, game_data.seed);
    
    
    ///////////////////////////////////////////////////////////
    // _ BEGIN FRAME LOOP
    // Setup Frame Loop Data
    let mut frame_tracker = FrameTracker::new();
    let mut window_focused = true;
    let mut closed = false;

    let test_scale = 18.0;
    
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
            closed = InputProcessor::process_input(delta_time, &mut camera, &window_info, &mut input_manager, &mut game_data);
        }
        else
        {
            display.gl_window().window().hide_cursor(false);
        }
        //

        ////////////////////
        // Update Game
        if game_data.remake_test_scene
        {
            match game_data.noise_type
            {
                NoiseType::RANDOM_2D =>
                    chunk_test_scene.make_chunk_random2d(game_data.seed),

                NoiseType::RANDOM_3D =>
                    chunk_test_scene.make_chunk_random3d(game_data.threshold, game_data.seed),

                NoiseType::OLC => 
                    chunk_test_scene.make_noise2D_test(game_data.octaves, game_data.bias, game_data.seed),
                
                NoiseType::SIMPLEX_2D => 
                    chunk_test_scene.make_simplex_noise2D(game_data.zoom_factor, game_data.seed),

                NoiseType::SIMPLEX_3D =>
                    chunk_test_scene.make_simplex_noise3D(game_data.zoom_factor, game_data.threshold, game_data.threshold_falloff, game_data.seed),
            };
            
            game_data.remake_test_scene = false;
        }
        chunk_test_scene.update(delta_time);


        //

        /////////////////////
        // Render frame
        let mut target = display.draw();
        target.clear_color_and_depth((0.2, 0.2, 0.3, 1.0), 1.0);

        // render objects
        // obj_demo_scene.render_scene(&mut asset_lib, &mut target, &camera.get_view());
        chunk_test_scene.render_scene(&mut asset_lib, &mut target, &camera.get_view());

        if game_data.print_help
        {              
            glyph_brush.queue(Section {
                text: get_controls_string(),
                scale: Scale { x: test_scale, y: test_scale },
                screen_position: (50.0, 0.0),
                bounds: (window_info.size.width as f32, window_info.size.height as f32 / 2.0),
                ..Section::default()
            });
        }

        if game_data.print_chunk_info
        {
            glyph_brush.queue(Section {
                text: &get_chunk_info_string(chunk_test_scene.get_chunk(), &game_data),
                scale: Scale { x: test_scale, y: test_scale },
                screen_position: (window_info.size.width as f32 / 2.0 + 200.0, 0.0),
                bounds: (250.0, window_info.size.height as f32 / 2.0),
                ..Section::default()
            });
        }


        glyph_brush.draw_queued(&(*display.inner), &mut target);

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

fn get_chunk_info_string(chunk: &WorldChunk, game_data: &GameData) -> String
{
    let mut info = String::from("Chunk Info:\n");
    info += &String::from(format!("\nDimensions: ({}, {}, {})", chunk.width, chunk.height, chunk.depth));
    info += &String::from(format!("\nTotal Blocks: {}\nHidden Blocks: {}\nRendered Blocks: {}", chunk.total_blocks, chunk.hidden_blocks, chunk.rendered_blocks));
    info += &String::from(format!("\n\nNoise Type: {:?}", game_data.noise_type));
    info += &String::from(format!("\n\nSeed: {:?}\n", game_data.seed));

    info += &match game_data.noise_type
    {
        NoiseType::RANDOM_2D => String::from(""),
        NoiseType::RANDOM_3D => String::from(format!("\nThreshold: {}", game_data.threshold)),
        NoiseType::OLC => String::from(format!("\nOctaves: {}\nBias: {}", game_data.octaves, game_data.bias)),
        NoiseType::SIMPLEX_2D => String::from(format!("\nZoom Factor: {}", game_data.zoom_factor)),
        NoiseType::SIMPLEX_3D => String::from(format!("\nZoom Factor: {}\nThreshold: {}\nThreshold Falloff: {}", game_data.zoom_factor, game_data.threshold, game_data.threshold_falloff)),
    };

    info
}

fn get_controls_string() -> &'static str
{
    "Demo Controls:\n\n\tWASD: Move\n\tE: \tMove Up\n\tQ: Move Down\n\tMouse Move: Look\n\t1, 2, 3, 4, 5: Change Noise Type\n\tR: Adjust Bias/Zoom Factor Up\n\tF: Adjust Bias/Zoom Factor Down\n\tT: Adjust Threshold Up\n\tG: Adjust Threshold Down\n\tSPACE: Increase Octave\n\tY: Adjust Threshold Falloff Up\n\tH: Adjust Threshold Falloff Down\n\tV: Use Default Seed\n\tC: Use New Random Seed\n\tSHIFT: Move and Adjust Faster\n\tF1: Show/Hide this message\n\tF2: Show/Hide Chunk Info"
}