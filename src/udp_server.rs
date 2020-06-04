use std::net::UdpSocket;
use std::str;

pub fn serve(address: &str) -> Result<(), failure::Error> {
	let serve_socket = UdpSocket::bind(address)?;
	loop {
		let mut buf = [0u8; 1024];
		let (size, src) = serve_socket.recv_from(&mut buf)?;
		debug!("Handling data from {}", src);
		print!("{}", str::from_utf8(&buf[..size])?);
		serve_socket.send_to(&buf, src)?;
	}
}