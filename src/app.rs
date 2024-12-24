use std::error::Error;
use crate::server::Server;
use crate::request::Request;
use crate::trie::{Trie, RouteTokens};

unsafe fn make_static<T>(t: &T) -> &'static T {
    std::mem::transmute(t)
}

#[derive(Default)]
pub struct App {
	map: Trie<fn(Request, Vec<RouteTokens>) -> String>,
}

impl App {
	pub async fn start(&self, addr: &str) -> Result<(), Box<dyn Error>> {
	    let mut server = Server::bind(addr).await?;
	    server.accept(unsafe { make_static(self) }).await?;
    	Ok(())
	}

	pub async fn handle(&self, req: Request) -> String {
		let b = req.url.split('/').filter(|s| !s.is_empty());
		self.map.get(b)
			.map(|(fun, args)| fun(req, args))
			.unwrap_or("Not found".to_owned())
	}

	pub fn service(&mut self, key: &str, fun: fn(Request, Vec<RouteTokens>) -> String) {
		let b = key.split('/').filter(|s| !s.is_empty());
		self.map.insert(b, fun);
	}
}