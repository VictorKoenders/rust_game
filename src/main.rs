#[macro_use] extern crate glium;
extern crate image;

mod render;
mod model;

use render::*;
use model::*;
use glium::Surface;

fn main() {
    let display_data = DisplayData::new();
    let model = Model::new(&display_data);

    loop {
        let mut target = display_data.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        model.render(&display_data, &mut target);
        target.finish().unwrap();

        for ev in display_data.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::KeyboardInput(_, _, Some(key)) => {
                    if key == glium::glutin::VirtualKeyCode::Escape {
                        return;
                    }
                }
                _ => ()
            }
        }
    }
}
