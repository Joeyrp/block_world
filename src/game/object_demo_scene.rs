/******************************************************************************
*	File		-	object_demo_scene.rs
*	Author		-	Joey Pollack
*	Date		-	2019/09/17 (y/m/d)
*	Mod Date	-	2019/09/17 (y/m/d)
*	Description	-	scene to test asset rendering
******************************************************************************/

use crate::{ utils::mat4_to_array, GridPlane, AssetLib, Flip };

#[allow(dead_code)]
pub struct ObjectDemoScene
{
    grid: GridPlane,
    grass_transform: glm::Mat4,
    dirt_transform: glm::Mat4,
    stone_transform: glm::Mat4,
    perspective: glm::Mat4
}

impl ObjectDemoScene
{
    pub fn new(assets: &mut AssetLib, display: &glium::Display, perspective: &glm::Mat4) 
        -> Result<ObjectDemoScene, String>
    {
        // Pre Load assets
        assets.get_mesh("assets/Cube/BasicCube.obj")?;
        assets.get_texture("assets/textures/Grass.png", Flip::NONE)?;
        assets.get_texture("assets/textures/Dirt.png", Flip::NONE)?;
        assets.get_texture("assets/textures/Stone.png", Flip::NONE)?;
        assets.get_program("Blocks", "assets/shaders/block.vert", "assets/shaders/block.frag")?;

        let grass_transform = glm::translate(&glm::Mat4::identity(), &glm::Vec3::new(0.0, 1.0, 0.0));
        let dirt_transform = glm::translate(&glm::Mat4::identity(), &glm::Vec3::new(3.0, 1.0, 0.0));
        let stone_transform = glm::translate(&glm::Mat4::identity(), &glm::Vec3::new(-3.0, 1.0, 0.0));

        let mut grid = GridPlane::new(&display, [0.75, 0.75, 0.75], 1.0, 20, 20).unwrap();
        grid.projection = *perspective;

        Ok( ObjectDemoScene { grid, grass_transform, dirt_transform, stone_transform, perspective: *perspective })
    }

    pub fn render_scene(self: &mut ObjectDemoScene, assets: &mut AssetLib, 
                            target: &mut glium::Frame, view: &glm::Mat4)
    {
        use glium::Surface;
        let block_mesh = assets.get_mesh("assets/Cube/BasicCube.obj").unwrap();
        let grass_tex = assets.get_texture("assets/textures/Grass.png", Flip::NONE).unwrap();
        let dirt_tex = assets.get_texture("assets/textures/Dirt.png", Flip::NONE).unwrap();
        let stone_tex = assets.get_texture("assets/textures/Stone.png", Flip::NONE).unwrap();
        let program = assets.get_program("Blocks", "assets/shaders/block.vert", "assets/shaders/block.frag").unwrap();

        
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        // Grass block
        let light = [-1.0, 0.4, 0.9f32];
        let grass_uniforms = &uniform! 
        { 
            model: mat4_to_array(&self.grass_transform), 
            view: mat4_to_array(view),
            perspective: mat4_to_array(&self.perspective),
            u_light: light,
            tex: grass_tex.get_texture()
        };

        // Dirt block
        let dirt_uniforms = &uniform! 
        { 
            model: mat4_to_array(&self.dirt_transform), 
            view: mat4_to_array(view),
            perspective: mat4_to_array(&self.perspective),
            u_light: light,
            tex: dirt_tex.get_texture()
        };

        // Stone block
        let stone_uniforms = &uniform! 
        { 
            model: mat4_to_array(&self.stone_transform), 
            view: mat4_to_array(view),
            perspective: mat4_to_array(&self.perspective),
            u_light: light,
            tex: stone_tex.get_texture()
        };
        
        self.grid.view = *view;
        self.grid.draw(target);
        target.draw(&block_mesh.vb, &block_mesh.indices, &program.program, grass_uniforms, &params).unwrap();
        target.draw(&block_mesh.vb, &block_mesh.indices, &program.program, dirt_uniforms, &params).unwrap();
        target.draw(&block_mesh.vb, &block_mesh.indices, &program.program, stone_uniforms, &params).unwrap();
    }

}