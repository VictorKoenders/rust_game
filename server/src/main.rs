extern crate shared;
extern crate time;

fn main(){
	let mut listener = shared::ServerSocket::create("localhost", 8080);

	let mut last_time = time::precise_time_s();
	loop {
		let update_time = time::precise_time_ns();
		for message in &listener.listen() {
			println!("Client send {:?}", message);
		}
		if time::precise_time_s() - last_time > 1f64 {
			println!("Sending ping");
			last_time = time::precise_time_s();
			for client in listener.clients.iter_mut() {
				client.send(shared::NetworkMessage::Ping);
			}
		}

		let delta_time = time::precise_time_ns() - update_time;
		let target_time = 1_000_000_000 / 30;
		if target_time > delta_time {
			std::thread::sleep(std::time::Duration::new(0, (target_time - delta_time) as u32));
		}
	}
}