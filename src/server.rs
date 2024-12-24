use tokio::net::{TcpListener};
use std::error::Error;

use crate::connection::Connection;
use crate::app::App;

pub struct Server {
	listener: TcpListener,
}

impl Server {
	pub async fn bind(addr: &str) -> Result<Server, Box<dyn Error>> {
		let listener = TcpListener::bind(&addr).await?;
	    println!("Listening on: {addr}");
	    Ok(Server{listener})
	}

	pub async fn accept(&mut self, app: &'static App) -> Result<(), Box<dyn Error>> {
		loop {
			let (stream, _) = self.listener.accept().await?;
			tokio::spawn(async move {
				println!("process task on {:?}", std::thread::current().id());
				let mut p = Connection::new(stream);
				p.start(app).await.unwrap();
			});
		}
	}
} 
