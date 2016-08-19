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
	let mut network = shared::ClientSocket::create("localhost", 8080);

	let mut last_time = time::precise_time_ns();
	let mut last_connect_time = None;
    loop {
		if !network.is_connected() {
			let should_connect = match last_connect_time {
				None => true,
				Some(t) => time::precise_time_s() - t > 1f64
			};
			if should_connect {
				if let Err(_) = network.connect() {
					last_connect_time = Some(time::precise_time_s());
				} else {
					last_connect_time = None;
				}
			}
		} else {
			loop {
				match network.get_message() {
					Ok(Some(message)) => {
						println!("message: {:?}", message);
						if message == shared::NetworkMessage::Ping {
							if let Err(e) = network.send(shared::NetworkMessage::Ping) {
								println!("Socket error: {:?}", e);
								network.disconnect();
								last_connect_time = Some(time::precise_time_s());
								break;
							}
						}
					},
					Ok(None) => break,
					Err(e) => {
						println!("Socket error: {:?}", e);
						network.disconnect();
						last_connect_time = Some(time::precise_time_s());
						break;
					}
				}
			}
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
