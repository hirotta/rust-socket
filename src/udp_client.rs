use std::net::UdpSocket;
use std::str;

pub fn communicate(address: &str) -> Result<(), failure::Error> {
	let socket = UdpSocket::bind("127.0.0.1:0")?;
	loop {
	let input = "a".to_string().repeat(65507);
	// io::stdin().read_line(&mut input)?;
	socket.send_to(input.as_bytes(), address).expect("failed to send data");
	let mut buffer = [0u8; 1024];
	socket.recv_from(&mut buffer).expect("failed to receive");
	print!(
		"{}",
		str::from_utf8(&buffer).expect("failed to convert to String")
	);
	}
}