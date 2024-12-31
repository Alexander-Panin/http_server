use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream};

pub struct Socket {
	stream: TcpStream,
	buf: [u8; 1024],
}

impl Socket {
	pub fn new(stream: TcpStream) -> Self {
	    Socket { stream, buf: [0; 1024] }
	}
}

impl Socket {

	pub async fn read_all(&mut self) -> Result<&str, Box<dyn Error>> {
		let n = self.stream.read(&mut self.buf).await?;
		Ok(std::str::from_utf8(&self.buf[..n])?)
	}

	pub async fn write_all(&mut self, s: String) -> Result<(), Box<dyn Error>> {
		let x = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\n", s.len());
		self.stream.write_all(&x.into_bytes()).await?;
		self.stream.write_all(&s.into_bytes()).await?;
		Ok(())
	}
}