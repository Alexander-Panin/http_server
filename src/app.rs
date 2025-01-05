use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::any::Any;
use crate::server::Server;
use crate::request::Request;
use crate::trie::{Trie, RouteTokens};

pub trait Router {
	fn handle(&self, req: Request) -> impl Future<Output = String> + Send;
}

pub trait Middleware {
	fn run(&self, req: &Request) -> Box<dyn Any + Send>;
}

// todo: example
impl Middleware for u32 {
	fn run(&self, _req: &Request) -> Box<dyn Any + Send> {
		Box::new(*self)
	}
}

impl Router for &App<()> {
	fn handle(&self, req: Request) -> impl Future<Output = String> + Send {
		App::<()>::handle(self, req)
	}
}

impl<T: Middleware + Default + Send + Sync + 'static> Router for &App<(T,)> {
	fn handle(&self, mut req: Request) -> impl Future<Output = String> + Send { 
		let x: Box<dyn Any + Send> = T::default().run(&req);
		req.context.insert((*x).type_id(), x);
		App::<(T,)>::handle(self, req)
	}
}

impl<T,Y> Router for &App<(T, Y)>
where  
	T: Middleware + Default + Send + Sync + 'static,
	Y: Middleware + Default + Send + Sync + 'static,
{
	fn handle(&self, mut req: Request) -> impl Future<Output = String> + Send {
		let x: Box<dyn Any + Send> = T::default().run(&req);
		req.context.insert((*x).type_id(), x);
		let y: Box<dyn Any + Send> = Y::default().run(&req);
		req.context.insert((*y).type_id(), y);
		App::<(T,Y)>::handle(self, req)
	}
}

#[derive(Default)]
pub struct App<T> {
	map: Trie<fn(Request, Vec<RouteTokens>) -> String>,
	phantom: PhantomData<T>,
}

unsafe fn make_static<T>(t: &T) -> &'static T {
    std::mem::transmute(t)
}

impl<T: Send + Sync + 'static> App<T> where for<'a> &'a App<T>: Router {
	pub async fn start(&self, addr: &str) -> Result<(), Box<dyn Error>> {
	    let mut server = Server::bind(addr).await?;
	    // safe: `app` should be alive before termination
	    server.accept(unsafe { make_static(self) }).await?;
    	Ok(())
	}

	pub async fn handle(&self, req: Request) -> String {
		let b = req.url.split('/').filter(|s| !s.is_empty());
		self.map.get(b)
			.map(|(fun, args)| fun(req, args))
			.unwrap_or("Not found".to_owned())
	}

	pub fn service(&mut self, key: &'static str, fun: fn(Request, Vec<RouteTokens>) -> String) {
		let b = key.split('/').filter(|s| !s.is_empty());
		self.map.insert(b, fun);
	}
}
