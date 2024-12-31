use tokio::net::{TcpStream};
use std::error::Error;

use crate::socket::Socket;
use crate::parser::Parser;
use crate::app::App;

pub struct Connection {
	socket: Socket,
}

impl Connection {
	pub fn new(stream: TcpStream) -> Self {
		Connection {
			socket: Socket::new(stream),
		}
	}
}

impl Connection	{
	pub async fn start(&mut self, app: &App) -> Result<(), Box<dyn Error>> {
		let s = self.socket.read_all().await?;
		if !s.is_empty() {
			let req = Parser::new(s).parse().ok_or("failed to parse")?;
			let resp = app.handle(req).await;
	        self.socket.write_all(resp).await?;
	    }
	    Ok(())
	}
}