use nalgebra_glm as glm;

pub struct FPCamera {
    position: glm::Vec3,

    front: glm::Vec3,
    up: glm::Vec3,
    right: glm::Vec3,

    world_up: glm::Vec3,

    yaw: f32,
    pitch: f32,
}

impl FPCamera {
    pub fn new(position: glm::Vec3, yaw: f32, pitch: f32) -> FPCamera {
        let mut camera = FPCamera {
            position,

            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::zero(),
            right: glm::zero(),

            world_up: glm::vec3(0.0, 1.0, 0.0),

            yaw,
            pitch,
        };
        camera.update();

        camera
    }

    pub fn move_position(&mut self, direction: &glm::Vec3) {
        self.position += direction.x * self.right;
        self.position += direction.y * self.up;
        self.position += direction.z * self.front;
    }

    pub fn rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch += delta_pitch;

        use std::f32::consts::PI;

        if self.pitch > PI / 2.0 {
            self.pitch = PI / 2.0;
        }

        if self.pitch < -PI / 2.0 {
            self.pitch = -PI / 2.0;
        }

        self.update();
    }

    pub fn view(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    fn update(&mut self) {
        // calculate then new self.front vector
        let x = self.yaw.cos() * self.pitch.cos();
        let y = self.pitch.sin();
        let z = self.yaw.sin() * self.pitch.cos();
        self.front = glm::Vec3::new(x, y, z).normalize();

        // re-calculate self.right and self.up
        self.right = self.front.cross(&self.world_up).normalize();
        self.up = self.right.cross(&self.front).normalize();
    }
}
