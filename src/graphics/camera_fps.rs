

// use nalgebra as na;
extern crate nalgebra_glm as glm;

#[allow(dead_code)]
pub struct CameraFPS 
{
    free_fly: bool,

    position: glm::TVec<f32, glm::U3>,
    front: glm::TVec<f32, glm::U3>,
    up: glm::TVec<f32, glm::U3>,
    right: glm::TVec<f32, glm::U3>,
    world_up: glm::TVec<f32, glm::U3>,

    yaw: f32,
    pitch: f32,

    pub look_sensitivity: f32,
}

#[allow(dead_code)]
impl CameraFPS
{
    pub fn new() -> CameraFPS
    {
        let mut c = CameraFPS { 
            free_fly: true, 
            position: glm::vec3(0.0, 0.0, 0.0), 
            front: glm::vec3(0.0, 0.0, -1.0), 
            up: glm::vec3(0.0, 1.0, 0.0),
            right: glm::vec3(1.0, 0.0, 0.0),
            world_up: glm::vec3(0.0, 1.0, 0.0),
            yaw: -90.0,
            pitch: 0.0,
            look_sensitivity: 0.1 };

        c.update_camera_vectors();
        return c;
    }

    pub fn get_position(&self) -> glm::Vec3
    {
        self.position
    }

    pub fn update_camera_vectors(&mut self)
    {
        //println!("pitch, yaw: {}, {}", self.pitch, self.yaw);
        self.front = glm::vec3(0.0, 0.0, 0.0);

        let yp = glm::radians(&glm::vec2(self.yaw.clone(), self.pitch.clone()));
        let yaw = yp.x;
        let pitch = yp.y;

        self.front.x = yaw.cos() * pitch.cos();
        self.front.y = pitch.sin();
        self.front.z = yaw.sin() * pitch.cos();
        self.front = self.front.normalize();

        self.right = glm::cross::<f32, glm::U3>(&self.front, &self.world_up).normalize();
        self.up = glm::cross::<f32, glm::U3>(&self.right, &self.front).normalize();
    }

    pub fn get_view(&self) -> glm::Mat4
    {
        glm::look_at_lh(&self.position, &(self.position + self.front), &self.up)
    }


    pub fn move_forward(&mut self, velocity: f32)
    {
        let velocity = velocity * -1.0;
        self.position += self.front * velocity;
    }

    pub fn move_right(&mut self, velocity: f32)
    {
        let velocity = velocity * -1.0;
        self.position += self.right * velocity;
    }

    pub fn move_up(&mut self, velocity: f32)
    {
        self.position += self.up * velocity;
    }

    pub fn apply_look_offset(&mut self, xoff: f32, yoff: f32)
    {
        let xoff = xoff * self.look_sensitivity;
        let yoff = yoff * self.look_sensitivity;


        self.yaw += xoff;
        self.pitch += yoff;

        if self.pitch > 89.0
        {
            self.pitch = 89.0;
        }

        if self.pitch < -89.0
        {
            self.pitch = -89.0;
        }

        //println!("pitch, yaw: {}, {}", self.pitch, self.yaw);
        self.update_camera_vectors();
    }
}