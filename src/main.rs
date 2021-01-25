use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::Program;

use glium::{glutin, texture, Surface};
use nalgebra_glm as glm;
use obj::{Obj, ObjError};
use std::error;

mod camera;
mod interaction;
mod model;

use camera::FPCamera;
use interaction::Interaction;
use model::{Assets, Model};

const VERTEX_SHADER_SRC: &str = "src/vert.glsl";
const FRAGMENT_SHADER_SRC: &str = "src/frag.glsl";

fn main() -> Result<(), Box<dyn error::Error>> {
    let cube_assets = Assets::load("assets/cube-textured.obj", "assets/texture.png")?;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800.0, 600.0))
        .with_title("})");
    let context = ContextBuilder::new();
    let display = glium::Display::new(window, context, &event_loop)?;

    let cube = Model::new(&cube_assets, &display)?;

    let program = Program::from_source(
        &display,
        &std::fs::read_to_string(VERTEX_SHADER_SRC)?,
        &std::fs::read_to_string(FRAGMENT_SHADER_SRC)?,
        None,
    )?;

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let perspective: glm::Mat4 = glm::perspective(4.0 / 3.0, 3.14 / 4.0, 0.1, 100.0);

    let mut model: glm::Mat4 = glm::identity();
    model = glm::scale(&model, &glm::vec3(0.5, 0.5, 0.5));

    let light_pos: glm::Vec3 = glm::vec3(10.0, 10.0, 0.0);
    let camera = FPCamera::new(glm::zero(), 0.0, 0.0);
    let mut interaction = Interaction::new(camera, 0.005, 1.0);

    event_loop.run(move |ev, _, control_flow| {
        let delta_time = interaction.update();

        model = glm::rotate(
            &model,
            glm::radians(&glm::vec1(delta_time * 16.0))[0],
            &glm::vec3(0.0, 1.0, 0.0),
        );

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let view = interaction.camera.view();
        target
            .draw(
                &cube.vertex_buffer,
                &cube.index_buffer,
                &program,
                &glium::uniform! {
                    tex: &cube.texture,
                    model: *model.as_ref(),
                    view: *view.as_ref(),
                    perspective: *perspective.as_ref(),
                    light_pos: *light_pos.as_ref()
                },
                &params,
            )
            .unwrap();

        target.finish().unwrap();

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                ev => {
                    interaction.process_event(&ev);
                }
            },
            _ => (),
        }
    });
}
