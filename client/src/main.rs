#[macro_use] extern crate glium;
extern crate image;
extern crate time;
extern crate vecmath;
extern crate shared;

mod render;
mod model;
mod game_state;

use game_state::GameState;
use render::*;
use model::*;
use glium::Surface;
use glium::glutin::VirtualKeyCode;

fn main() {
    let mut display_data = DisplayData::new();
	let mut game_state = GameState::new();
    let model = Model::new(&display_data);
	let mut network = match shared::ClientSocket::connect("localhost", 8080) {
		Some(n) => n,
		None => {
			return;
		}
	};

	let mut last_time = time::precise_time_ns();
    loop {
	    while let Some(message) = network.get_message() {
		    println!("message: {:?}", message);
			network.send(message);
	    }

        let time_now = time::precise_time_ns();
	    let diff: f32 = ((time_now - last_time) / 1000) as f32;
	    last_time = time_now;

	    display_data.update(&game_state, diff);

        let mut target = display_data.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        model.render(&display_data, &mut target);
        target.finish().unwrap();

	    game_state.mouse.reset();

        for ev in display_data.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::KeyboardInput(state, _, Some(key)) => {
	                game_state.keyboard.update(key, state);

	                if game_state.keyboard.is_pressed(VirtualKeyCode::Escape) {
                        return;
                    }
                }
	            glium::glutin::Event::MouseMoved(x, y) => game_state.mouse.mouse_moved(x, y),
	            glium::glutin::Event::MouseInput(state, button) => game_state.mouse.mouse_button(button, state),
                _ => ()
            }
        }
    }
}
