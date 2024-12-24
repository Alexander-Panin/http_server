use tokio::net::{TcpStream};
use std::error::Error;

use crate::socket::Socket;
use crate::parser::Parser;
use crate::app::App;

pub struct Connection {
	socket: Socket
}

impl Connection {
	pub fn new(stream: TcpStream) -> Self {
		Connection {
			socket: Socket::new(stream)
		}
	}
}

impl Connection	{
	pub async fn start(&mut self, app: &App) -> Result<(), Box<dyn Error>> {
		let n = self.socket.read().await?;
		if n != 0 {
			let s = std::str::from_utf8(&self.socket.buf[..n])?;
			let req = Parser::new(s).parse().unwrap();
			let resp = app.handle(req).await;
	        self.socket.write_headers(resp.len()).await?;
	        self.socket.write_all(resp).await?;
	    }
	    Ok(())
	}
}