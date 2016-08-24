#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate shared;
extern crate time;
extern crate bincode;
extern crate rustc_serialize;

mod network;

use shared::*;
use network::ServerSocket;

fn main(){
	// TODO: Load the world state from database
	// TODO: Load all the players from database
	let mut listener = ServerSocket::create("localhost", 8080);

	let mut last_time = time::precise_time_s();
	let mut last_print_time = 0.0;
	loop {
		let update_time = time::precise_time_ns();


		// Nasty solution to break out of the 3 callback functions that listener.listen has
		// The 3 functions get their own sender, that all loop into the receiver
		// This is because we can't call listener.broadcast in the callback functions because listener is already being used as a mutable
		let (send, receive) = std::sync::mpsc::channel();
		let s1 = send.clone();
		let s2 = send.clone();
		let s3 = send;

		// TODO: move all the data into a world state and pass the world state to the listener
		// These 3 functions then go to the world state instead of in here
		let listen_result = listener.listen(move |new_client| {
			let id = new_client.id;
			try!(new_client.send(NetworkMessage::Identify(id)));
			try!(s1.send(NetworkMessage::SetPosition {
				uid: id,
				position: [-10.0, 0.0, 0.0],
				rotation: [0.0, 0.0, 0.0]
			}));
			Ok(())
		}, move |client, message| {
			// TODO: Find something smart to handle all the different messages
			if message == NetworkMessage::Ping {
				let ping = ((time::precise_time_s() - client.last_ping_time) * 1000f64) as u32;
				try!(client.send(NetworkMessage::PingResult(ping)));
			}
			if let NetworkMessage::SetPosition { position, rotation, .. } = message {
				try!(s2.send(NetworkMessage::SetPosition {
					uid: client.id,
					position: position,
					rotation: rotation,
				}));
			}
			Ok(())
		}, move |client| {
			try!(s3.send(NetworkMessage::RemoveEntity { uid: client.id }));
			Ok(())
		});

		if let Err(e) = listen_result {
			panic!("Could not listen: {:?}", e);
		}

		// Get the broadcast messages from the channels and handle them
		while let Ok(message) = receive.try_recv() {
			listener.broadcast(message);
		}

		// Send all players a ping every second
		// TODO: Will this spam too much? Maybe make it every 5, 10, 60 seconds?
		if time::precise_time_s() - last_time > 1f64 {
			println!("Ping!");
			last_time = time::precise_time_s();
			for client in &mut listener.clients {
				client.last_ping_time = last_time;
				client.send(NetworkMessage::Ping).unwrap();// TODO: Deal with unwrap
			}
		}

		// Sleep so that the server reaches 50 UPS
		let delta_time = time::precise_time_ns() - update_time;
		let target_time = 1_000_000_000 / 50;
		if target_time > delta_time {
			std::thread::sleep(std::time::Duration::new(0, (target_time - delta_time) as u32));
		} else if time::precise_time_s() > last_print_time + 5.0 {
			// Server too slow, can't keep up
			println!("{}: Server couldn't keep up with 50 ups", time::now().strftime("%H:%M:%S").unwrap());// TODO: Deal with unwrap
			last_print_time = time::precise_time_s();
		}
	}
}

