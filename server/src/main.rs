extern crate shared;
extern crate time;

fn main(){
	let mut listener = shared::ServerSocket::create("localhost", 8080);

	let mut last_time = time::precise_time_s();
	loop {
		let update_time = time::precise_time_ns();
		&listener.listen(|client, message| {
			println!("Client send {:?}", message);
			if message == shared::NetworkMessage::Ping {
				let ping = ((time::precise_time_s() - client.last_ping_time) * 1000f64) as u32;
				println!("Ping time: {}", ping);
				client.send(shared::NetworkMessage::PingResult(ping));
			}
		});
		if time::precise_time_s() - last_time > 1f64 {
			last_time = time::precise_time_s();
			for client in listener.clients.iter_mut() {
				client.last_ping_time = last_time;
				client.send(shared::NetworkMessage::Ping);
			}
		}

		let delta_time = time::precise_time_ns() - update_time;
		let target_time = 1_000_000_000 / 1000;
		if target_time > delta_time {
			std::thread::sleep(std::time::Duration::new(0, (target_time - delta_time) as u32));
		}
	}
}