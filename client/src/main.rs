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
use shared::*;

fn main() {
    let mut display_data = DisplayData::new();
	let mut game_state = GameState::new();
    let mut model = Model::new_cube(&display_data);
	let mut cubes: Vec<Model> = Vec::new();
	let mut network = ClientSocket::create("localhost", 8080);

	let mut last_time = time::precise_time_ns();
	let mut last_connect_time = None;
	let mut position_interval = 0;
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
						if message == NetworkMessage::Ping {
							if let Err(e) = network.send(NetworkMessage::Ping) {
								println!("Socket error: {:?}", e);
								network.disconnect();
								last_connect_time = Some(time::precise_time_s());
								break;
							}
						}
						if let NetworkMessage::Identify(uid) = message {
							model.id = uid;
						}
						if let NetworkMessage::RemoveEntity { uid } = message{
							if let Some(index) = cubes.iter().position(|x| x.id == uid) {
								cubes.remove(index);
							}
						}
						if let NetworkMessage::SetPosition { uid, position, rotation: _ } = message {
							if uid != model.id {
								let mut found = false;
								{
									let item = cubes.iter_mut().find(|c| c.id == uid);
									if let Some(c) = item {
										c.position = position;
										found = true;
									}
								}
								if !found {
									let mut c = Model::new_cube(&display_data);
									c.id = uid;
									c.position = position;
									c.scale = [0.1, 0.1, 0.1];
									cubes.push(c);
								}
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
		for cube in &cubes{
			cube.render(&display_data, &mut target);
		}
        target.finish().unwrap();

	    game_state.mouse.reset();

		if network.is_connected() {
			position_interval += 1;
			if position_interval > 30 {
				position_interval = 0;
				let pos = display_data.camera_position;
				let msg = NetworkMessage::SetPosition {
					uid: 0,
					position: [-pos[0], -pos[1], -pos[2]],
					rotation: display_data.camera_rotation.clone(),
				};
				network.send(msg).unwrap();
			}
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
