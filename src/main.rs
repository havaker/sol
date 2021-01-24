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

use camera::FPCamera;
use interaction::Interaction;

const CUBE_TEXTURE_PATH: &str = "assets/texture.png";
//const CUBE_TEXTURE_PATH: &str = "assets/Earth_TEXTURE_CM.tga";
const CUBE_MODEL_PATH: &str = "assets/cube-textured.obj";
//const CUBE_MODEL_PATH: &str = "assets/Earth_tr.obj";

const VERTEX_SHADER_SRC: &str = "src/vert.glsl";
const FRAGMENT_SHADER_SRC: &str = "src/frag.glsl";

fn load_image(path: &str) -> Result<glium::texture::RawImage2d<u8>, image::ImageError> {
    use image::io::Reader as ImageReader;

    let img = ImageReader::open(path)?.decode()?.into_rgba8();
    let dimensions = img.dimensions();

    Ok(glium::texture::RawImage2d::from_raw_rgba_reversed(
        &img.into_raw(),
        dimensions,
    ))
}

fn load_model(path: &str) -> Result<Obj<obj::TexturedVertex, u16>, ObjError> {
    use obj::load_obj;
    use std::fs::File;
    use std::io::BufReader;

    let input = BufReader::new(File::open(path)?);
    let obj: Obj<obj::TexturedVertex, u16> = load_obj(input)?;
    return Ok(obj);
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800.0, 600.0))
        .with_title("})");

    let context = ContextBuilder::new();

    let display = glium::Display::new(window, context, &event_loop)?;

    let cube_obj = load_model(CUBE_MODEL_PATH)?;
    let vb = cube_obj.vertex_buffer(&display)?;
    let ib = cube_obj.index_buffer(&display)?;

    let img = load_image(CUBE_TEXTURE_PATH)?;
    let texture = glium::texture::Texture2d::new(&display, img).unwrap();

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
    /*
    trans = glm::rotate(
        &trans,
        glm::radians(&glm::vec1(90.0 + t))[0],
        &glm::vec3(0.5, 1.0, 0.0),
    );
    */

    let camera = FPCamera::new(glm::zero(), 0.0, 0.0);
    let mut interaction = Interaction::new(camera, 0.005, 1.0);

    event_loop.run(move |ev, _, control_flow| {
        interaction.update();

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let view = interaction.camera.view();
        target
            .draw(
                &vb,
                &ib,
                &program,
                &glium::uniform! {
                    tex: &texture,
                    model: *model.as_ref(),
                    view: *view.as_ref(),
                    perspective: *perspective.as_ref()
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
