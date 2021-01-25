use std::error;
use std::f32::consts::PI;

use glium::glutin::dpi::LogicalSize;
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::{glutin, Surface};
use nalgebra_glm as glm;

mod camera;
mod interaction;
mod model;
mod solar;

use camera::FPCamera;
use interaction::Interaction;
use model::Assets;
use solar::Solar;

fn main() -> Result<(), Box<dyn error::Error>> {
    let earth = Assets::load("assets/Earth_tr.obj", "assets/Earth_TEXTURE_CM.tga")?;
    //let sun = Assets::load("assets/Earth_tr.obj", "assets/sun.jpeg")?;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800.0, 600.0))
        .with_title("})");
    let context = ContextBuilder::new();
    let display = glium::Display::new(window, context, &event_loop)?;

    let solar = Solar::new(&earth, &earth, &display)?;

    let perspective: glm::Mat4 = glm::perspective(4.0 / 3.0, 3.14 / 4.0, 0.1, 100.0);
    // look toward z axis
    let camera = FPCamera::new(glm::vec3(0.0, 0.0, -2.0), PI / 2.0, 0.0);
    let mut interaction = Interaction::new(camera, 0.005, 1.0);

    event_loop.run(move |ev, _, control_flow| {
        /*let delta_time =*/
        interaction.update();

        let view = interaction.camera.view();

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        solar.draw(&view, &perspective, &mut target);

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
