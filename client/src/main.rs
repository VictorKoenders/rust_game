#[macro_use] extern crate glium;
extern crate image;
extern crate time;
extern crate vecmath;
extern crate shared;

mod render;
mod model;
mod game_state;
mod network;

use game_state::GameState;
use render::*;
use glium::Surface;
use glium::glutin::VirtualKeyCode;
use shared::*;
use model::Model;

fn main() {
    let mut display_data = DisplayData::new();
	let mut game_state = GameState::new();
    let mut model = Model::new_cube(&display_data);
	let mut network = network::Network::new();

	let mut last_time = time::precise_time_ns();
    loop {

        let time_now = time::precise_time_ns();
	    let diff: f32 = ((time_now - last_time) / 1000) as f32;
	    last_time = time_now;

	    display_data.update(&mut game_state, diff);
		network.update(&mut game_state);

        let mut target = display_data.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
		if let Some(ref player) = game_state.player {
			if let Some(ref model) = player.model {
				model.render(&display_data, &mut target);
			}
		}
		for entity in &game_state.entities {
			if let Some(ref model) = entity.model {
				model.render(&display_data, &mut target);
			}
		}
		model.render(&display_data, &mut target);

        target.finish().unwrap();

	    game_state.mouse.reset();

		if let Some(ref player) = game_state.player {
			network.send_throttled(NetworkMessage::SetPosition {
				uid: 0,
				position: [player.position[0], player.position[1], player.position[2]],
				rotation: display_data.camera_rotation.clone(),
			}, 200);
		}

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
