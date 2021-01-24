use std::time::Instant;

use glium::glutin;
use nalgebra_glm as glm;

use crate::camera::FPCamera;

pub struct Interaction {
    pub camera: FPCamera,

    clock: Instant,

    direction: glm::Vec3,
    direction_complement: glm::Vec3,

    mouse_recent: glm::Vec2,
    mouse_delta: glm::Vec2,

    dragging: bool,

    sensitivity: f32,
    speed: f32,
}

impl Interaction {
    pub fn new(camera: FPCamera, sensitivity: f32, speed: f32) -> Self {
        Self {
            camera,
            clock: Instant::now(),

            direction: glm::zero(),
            direction_complement: glm::zero(),

            mouse_recent: glm::zero(),
            mouse_delta: glm::zero(),

            dragging: false,

            sensitivity,
            speed,
        }
    }

    fn compute_delta_time(&mut self) -> f32 {
        let delta = self.clock.elapsed().as_secs_f32();
        self.clock = Instant::now();
        delta
    }

    pub fn update(&mut self) {
        let delta_time = self.compute_delta_time();
        let delta_direction = (self.direction - self.direction_complement) * delta_time;
        let movement = delta_direction * self.speed;

        self.camera.move_position(&movement);
        if self.dragging {
            self.mouse_delta.y *= -1.0;
            let rotation = -self.mouse_delta * self.sensitivity;
            self.mouse_delta = glm::zero();

            self.camera.rotate(rotation.x, rotation.y);
        }
    }

    fn process_keyboard_input(&mut self, input: glutin::event::KeyboardInput) {
        let pressed = if input.state == glutin::event::ElementState::Pressed {
            1.0
        } else {
            0.0
        };

        let key = match input.virtual_keycode {
            Some(key) => key,
            None => return,
        };

        match key {
            glutin::event::VirtualKeyCode::E => self.direction.y = pressed,
            glutin::event::VirtualKeyCode::Q => self.direction_complement.y = pressed,
            glutin::event::VirtualKeyCode::D => self.direction.x = pressed,
            glutin::event::VirtualKeyCode::A => self.direction_complement.x = pressed,
            glutin::event::VirtualKeyCode::W => self.direction.z = pressed,
            glutin::event::VirtualKeyCode::S => self.direction_complement.z = pressed,
            _ => (),
        };
    }

    pub fn process_event(&mut self, event: &glutin::event::WindowEvent<'_>) {
        use glium::backend::glutin::glutin::event::ElementState;
        use glium::glutin::event::MouseButton;

        match *event {
            glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                self.process_keyboard_input(input)
            }
            glutin::event::WindowEvent::CursorMoved { position, .. } => {
                let mouse = glm::vec2(position.x as f32, position.y as f32);
                self.mouse_delta = self.mouse_recent - mouse;
                self.mouse_recent = mouse;
            }
            glutin::event::WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => self.dragging = true,
            glutin::event::WindowEvent::MouseInput {
                state: ElementState::Released,
                button: MouseButton::Left,
                ..
            } => self.dragging = false,
            _ => return,
        };
    }
}
