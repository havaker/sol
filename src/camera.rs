use nalgebra_glm as glm;

pub struct FPCamera {
    position: glm::Vec3,

    world_up: glm::Vec3,

    yaw: f32,
    pitch: f32,
}

impl FPCamera {
    pub fn new(position: glm::Vec3, yaw: f32, pitch: f32) -> FPCamera {
        let camera = FPCamera {
            position,

            world_up: glm::vec3(0.0, 1.0, 0.0),

            yaw,
            pitch,
        };

        camera
    }

    pub fn move_position(&mut self, direction: &glm::Vec3) {
        let mut front = self.calculate_front();

        // move only along the xz plane
        front.y = 0.0;
        front.normalize_mut();

        let (right, up) = self.calculate_right_and_up(&front);

        self.position += direction.x * right;
        self.position += direction.y * up;
        self.position += direction.z * front;
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
    }

    pub fn view(&self) -> glm::Mat4 {
        let front = self.calculate_front();
        let (_, up) = self.calculate_right_and_up(&front);

        glm::look_at(&self.position, &(self.position + front), &up)
    }

    fn calculate_front(&self) -> glm::Vec3 {
        let x = self.yaw.cos() * self.pitch.cos();
        let y = self.pitch.sin();
        let z = self.yaw.sin() * self.pitch.cos();

        glm::Vec3::new(x, y, z).normalize()
    }

    fn calculate_right_and_up(&self, front: &glm::Vec3) -> (glm::Vec3, glm::Vec3) {
        let right = front.cross(&self.world_up).normalize();
        let up = right.cross(&front).normalize();

        (right, up)
    }
}
