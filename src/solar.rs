use nalgebra_glm as glm;

use std::error::Error;

use glium::{Display, Program, Surface};
use glm::{Mat4, Vec3};

use crate::model::{Assets, Model};

const VERTEX_SHADER_SRC: &str = "src/vert.glsl";
const FRAGMENT_SHADER_SRC: &str = "src/frag.glsl";

pub struct Solar {
    sun: Model,
    earth: Model,
    moon: Model,
    skybox: Model,

    program: Program,
    time: f32,
}

impl Solar {
    pub fn new(
        sun: &Assets,
        earth: &Assets,
        moon: &Assets,
        skybox: &Assets,
        display: &Display,
    ) -> Result<Self, Box<dyn Error>> {
        let sun = Model::new(sun, display)?;
        //sun.transform = glm::scale(&sun.transform, &glm::vec3(0.1, 0.1, 0.1));

        let earth = Model::new(earth, display)?;
        let moon = Model::new(moon, display)?;

        let mut skybox = Model::new(skybox, display)?;
        skybox.transform = glm::scale(&skybox.transform, &glm::vec3(100.0, 100.0, 100.0));

        let program = Program::from_source(
            display,
            &std::fs::read_to_string(VERTEX_SHADER_SRC)?,
            &std::fs::read_to_string(FRAGMENT_SHADER_SRC)?,
            None,
        )?;

        Ok(Self {
            sun,
            earth,
            moon,
            skybox,
            program,
            time: 0.0,
        })
    }

    pub fn update(&mut self, delta_time: f32) {
        self.time += delta_time;

        let earth_rotation = glm::rotate(&glm::identity(), self.time, &glm::vec3(0.0, 1.0, 0.0));
        let earth_translation = glm::translation(&glm::vec3(10.0, 0.0, 0.0));
        let earth_rotation_around_sun = glm::rotate(
            &glm::identity(),
            self.time / 32.0,
            &glm::vec3(0.0, 1.0, 0.0),
        );

        self.earth.transform = earth_rotation_around_sun * earth_translation * earth_rotation;

        let moon_rotation =
            glm::rotate(&glm::identity(), self.time / 4.0, &glm::vec3(0.0, 1.0, 0.0));
        let moon_translation = glm::translation(&glm::vec3(0.5, 0.0, 0.0));
        let moon_rotation_around_earth =
            glm::rotate(&glm::identity(), self.time / 4.0, &glm::vec3(0.2, 1.0, 0.0));
        self.moon.transform = earth_rotation_around_sun
            * earth_translation
            * moon_rotation_around_earth
            * moon_translation
            * moon_rotation;

        let earth_scale = glm::scale(&glm::identity(), &glm::vec3(0.1, 0.1, 0.1));
        let moon_scale = glm::scale(&glm::identity(), &glm::vec3(0.01, 0.01, 0.01));
        self.earth.transform *= earth_scale;
        self.moon.transform *= moon_scale;
    }

    pub fn draw(&self, view: &Mat4, perspective: &Mat4, target: &mut impl Surface) {
        let light_pos: Vec3 = glm::vec3(0.0, 0.0, 0.0);
        let light_color: Vec3 = glm::vec3(1.0, 1.0, 1.0);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let sun_ambient_strength: f32 = 1.0;
        target
            .draw(
                &self.sun.vertex_buffer,
                &self.sun.index_buffer,
                &self.program,
                &glium::uniform! {
                    tex: &self.sun.texture,
                    model: *self.sun.transform.as_ref(),
                    view: *view.as_ref(),
                    perspective: *perspective.as_ref(),
                    light_pos: *light_pos.as_ref(),
                    light_color: *light_color.as_ref(),
                    ambient_strength: sun_ambient_strength
                },
                &params,
            )
            .unwrap();

        let earth_ambient_strength: f32 = 0.01;
        target
            .draw(
                &self.earth.vertex_buffer,
                &self.earth.index_buffer,
                &self.program,
                &glium::uniform! {
                    tex: &self.earth.texture,
                    model: *self.earth.transform.as_ref(),
                    view: *view.as_ref(),
                    perspective: *perspective.as_ref(),
                    light_pos: *light_pos.as_ref(),
                    light_color: *light_color.as_ref(),
                    ambient_strength: earth_ambient_strength
                },
                &params,
            )
            .unwrap();

        let moon_ambient_strength: f32 = 0.02;
        target
            .draw(
                &self.moon.vertex_buffer,
                &self.moon.index_buffer,
                &self.program,
                &glium::uniform! {
                    tex: &self.moon.texture,
                    model: *self.moon.transform.as_ref(),
                    view: *view.as_ref(),
                    perspective: *perspective.as_ref(),
                    light_pos: *light_pos.as_ref(),
                    light_color: *light_color.as_ref(),
                    ambient_strength: moon_ambient_strength
                },
                &params,
            )
            .unwrap();

        let skybox_ambient_strength: f32 = 0.1;
        target
            .draw(
                &self.skybox.vertex_buffer,
                &self.skybox.index_buffer,
                &self.program,
                &glium::uniform! {
                    tex: &self.skybox.texture,
                    model: *self.skybox.transform.as_ref(),
                    view: *view.as_ref(),
                    perspective: *perspective.as_ref(),
                    light_pos: *light_pos.as_ref(),
                    light_color: *light_color.as_ref(),
                    ambient_strength: skybox_ambient_strength
                },
                &params,
            )
            .unwrap();
    }
}
