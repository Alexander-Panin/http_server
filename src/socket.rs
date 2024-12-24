use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream};

pub struct Socket {
	pub stream: TcpStream,
	pub buf: Vec<u8>,
}

impl Socket {
	pub fn new(stream: TcpStream) -> Self {
	    Socket { 
	    	stream,
	    	buf: vec![0; 1024]
	    }
	}
}

impl Socket {

	pub async fn read(&mut self) -> Result<usize, Box<dyn Error>> {
		Ok(self.stream.read(&mut self.buf).await?)
	}

	pub async fn write_headers(&mut self, n: usize) -> Result<(), Box<dyn Error>> {
		// todo: correct count bytes
		let x = format!("HTTP/1.1 200 OK\r\nContent-Length: {n}\r\n\n");
		Ok(self.stream.write_all(x.as_bytes()).await?)
	}

	pub async fn write_all(&mut self, s: String) -> Result<(), Box<dyn Error>> {
		self.buf = s.into_bytes(); // todo append to end
		Ok(self.stream.write_all(&self.buf).await?)
	}
}