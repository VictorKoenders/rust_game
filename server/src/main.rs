extern crate shared;
extern crate time;
extern crate bincode;
extern crate rustc_serialize;

mod file_handler;
mod network;

use shared::*;
use network::ServerSocket;
use file_handler::*;
use std::collections::HashMap;

fn main(){
	/*let mut vec = Vec::new();
	vec.push((
		User { position: [1.0f32, 0.0f32, 0.0f32], rotation: [0.0f32, 0.0f32, 0.0f32], id: 1, name: "Trangar".to_string() },
		UserPassword { password: "test".to_string(), user_id: 1 }
	));
	FileHandler::save_users(&vec);
	return;*/
	let mut listener = ServerSocket::create("localhost", 8080);

	let mut last_time = time::precise_time_s();
	loop {
		let update_time = time::precise_time_ns();
		let (send, receive) = std::sync::mpsc::channel();
		let s1 = send.clone();
		let s2 = send.clone();
		let s3 = send;

		listener.listen(move |new_client| {
			let id = new_client.id;
			new_client.send(NetworkMessage::Identify(id));
			s1.send(NetworkMessage::SetPosition {
				uid: id,
				position: [-10.0, 0.0, 0.0],
				rotation: [0.0, 0.0, 0.0]
			});
		}, move |client, message| {
			if message == NetworkMessage::Ping {
				let ping = ((time::precise_time_s() - client.last_ping_time) * 1000f64) as u32;
				//println!("Ping time: {}", ping);
				client.send(NetworkMessage::PingResult(ping)).unwrap();
			}
			if let NetworkMessage::SetPosition{ uid: _, position: position, rotation: rotation} = message {
				s2.send(NetworkMessage::SetPosition {
					uid: client.id,
					position: position,
					rotation: rotation,
				});
			}
		}, move |client| {
			s3.send(NetworkMessage::RemoveEntity { uid: client.id });
		});

		while let Ok(message) = receive.try_recv() {
			listener.broadcast(message);
		}

		if time::precise_time_s() - last_time > 1f64 {
		    println!("Ping!");
			last_time = time::precise_time_s();
			for client in listener.clients.iter_mut() {
				client.last_ping_time = last_time;
				client.send(NetworkMessage::Ping).unwrap();
			}
		}

		let delta_time = time::precise_time_ns() - update_time;
		let target_time = 1_000_000_000 / 1000;
		if target_time > delta_time {
			std::thread::sleep(std::time::Duration::new(0, (target_time - delta_time) as u32));
		}
	}
}

