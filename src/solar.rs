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

    program: Program,
}

impl Solar {
    pub fn new(sun: &Assets, earth: &Assets, display: &Display) -> Result<Self, Box<dyn Error>> {
        let mut sun = Model::new(sun, display)?;
        sun.transform = glm::scale(&sun.transform, &glm::vec3(0.1, 0.1, 0.1));

        let mut earth = Model::new(earth, display)?;
        earth.transform = glm::scale(&earth.transform, &glm::vec3(0.1, 0.1, 0.1));
        let earth_pos = glm::vec3(10.0, 0.0, 0.0);
        earth.transform *= glm::translation(&earth_pos);

        let program = Program::from_source(
            display,
            &std::fs::read_to_string(VERTEX_SHADER_SRC)?,
            &std::fs::read_to_string(FRAGMENT_SHADER_SRC)?,
            None,
        )?;

        Ok(Self {
            sun,
            earth,
            program,
        })
    }

    pub fn draw(&self, view: &Mat4, perspective: &Mat4, target: &mut impl Surface) {
        let light_pos: Vec3 = glm::vec3(0.0, 0.0, 0.0);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

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
                    light_pos: *light_pos.as_ref()
                },
                &params,
            )
            .unwrap();

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
                    light_pos: *light_pos.as_ref()
                },
                &params,
            )
            .unwrap();
    }
}
