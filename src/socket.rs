use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream};

pub struct Socket {
	pub stream: TcpStream,
	pub buf: [u8; 1024],
}

impl Socket {
	pub fn new(stream: TcpStream) -> Self {
	    Socket { stream, buf: [0; 1024] }
	}
}

impl Socket {

	pub async fn read(&mut self) -> Result<usize, Box<dyn Error>> {
		Ok(self.stream.read(&mut self.buf).await?)
	}

	pub async fn write_all(&mut self, s: String) -> Result<(), Box<dyn Error>> {
		let x = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\n", s.len());
		self.stream.write_all(&x.into_bytes()).await?;
		self.stream.write_all(&s.into_bytes()).await?;
		Ok(())
	}
}