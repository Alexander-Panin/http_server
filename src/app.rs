use std::error::Error;
use crate::server::Server;
use crate::request::Request;
use crate::trie::Trie;

unsafe fn make_static<T>(t: &T) -> &'static T {
    std::mem::transmute(t)
}

#[derive(Default)]
pub struct App {
	map: Trie<Box<dyn Fn(Request) -> String + Send + Sync>>,
}

impl App {
	pub async fn start(&self, addr: &str) -> Result<(), Box<dyn Error>> {
	    let mut server = Server::bind(addr).await?;
	    server.accept(unsafe { make_static(self) }).await?;
    	Ok(())
	}

	pub async fn handle(&self, req: Request) -> String {
		let b = req.url.split('/').filter(|s| !s.is_empty());
		self.map.get(b).map(|fun| fun(req)).unwrap_or("Not found".to_owned())
	}

	pub fn service(&mut self, key: &str, fun: Box<dyn Fn(Request) -> String + Send + Sync>) {
		let b = key.split('/').filter(|s| !s.is_empty());
		self.map.insert(b, fun);
	}
}