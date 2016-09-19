#![feature(drop_types_in_const)]

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use] extern crate glium;
extern crate glium_text;
extern crate image;
extern crate time;
extern crate vecmath;
extern crate shared;

mod render;
mod model;
mod game_state;
mod network;
mod ui;
mod handler;
#[cfg(test)]
mod test;
mod error;

use game_state::{Entity, GameState};
use render::*;
use glium::Surface;
use glium::glutin::{VirtualKeyCode, Event};
use shared::*;
use model::Model;
use std::io::{self, Write};
use std::fs;
use std::error::Error;


fn main() {
	if let Err(e) = run() {
		log_error(e).expect("Error to be logged");
	}
}
fn log_error(e: error::GameError) -> io::Result<()> {
	let mut f = try!(fs::File::create("err.log"));
	try!(write!(&mut f, "{}", e.description()));
	Ok(())
}

fn run() -> Result<(), error::GameError> {
	let mut display_data = DisplayData::new();
	handler::texture::init(&display_data);
	let mut game_state = GameState::new();
	let model = try!(Model::new_cube(&display_data));
	let mut network = network::Network::new();

	let mut last_time = time::precise_time_ns();
	let mut ui = ui::UI::new();

	let size = display_data.get_screen_dimensions();
	ui.resize(&display_data, size.0, size.1);
	ui.load(&display_data, ui::UIView::Login);
	loop {

		let time_now = time::precise_time_ns();
		let diff: f32 = ((time_now - last_time) / 1000) as f32;
		last_time = time_now;

		game_state.update(diff);
		try!(display_data.update(&mut game_state));
		network.update(&mut game_state);
		ui.update(diff);

		let mut target = display_data.display.draw();
		if game_state.player.is_some() {
			target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
			for entity in &game_state.entities {
				if let Some(ref model) = entity.model {
					try!(model.render(&display_data, &mut target, entity));
				}
			}
			try!(model.render(&display_data, &mut target, &Entity::empty()));
		}
		//if let Some(ref player) = game_state.player {
		//	if let Some(ref model) = player.model {
		//		model.render(&display_data, &mut target, &player);
		//	}
		//}

		ui.render(&mut target, &display_data);

		try!(target.finish().map_err(error::GameError::from_swap_buffers_error));

		game_state.mouse.reset();

		if let Some(ref player) = game_state.player {
			network.send_throttled(NetworkMessage::SetPosition {
				uid: 0,
				position: player.position,
				rotation: player.rotation,
			}, 100);
		}

		let mut new_size = None;
		for ev in display_data.display.poll_events() {
			if ui.handle_event(&ev) {
				continue;
			}

			// TODO: Move all the logic to the elements that decide on it
			// And allow certain elements to override each other
			// For example: When typing in a textbox, you don't want to move your character or hit any other buttons
			match ev {
				Event::Closed => return Ok(()),
				Event::KeyboardInput(state, _, Some(key)) => {
					game_state.keyboard.update(key, state);

					if game_state.keyboard.is_pressed(VirtualKeyCode::Escape) {
						return Ok(());
					}
				}
				Event::MouseMoved(x, y) => game_state.mouse.mouse_moved(x, y, display_data.get_screen_dimensions()),
				Event::MouseInput(state, button) => game_state.mouse.mouse_button(button, state),
				Event::Resized(width, height) => new_size = Some((width, height)),
				_ => ()
			}
		}

		if let Some(size) = new_size {
			display_data.resize(size.0, size.1);
			ui.resize(&display_data, size.0, size.1);
		}
	}
}
