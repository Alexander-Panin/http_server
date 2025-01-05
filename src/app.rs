use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::collections::HashMap;
use std::any::Any;
use crate::server::Server;
use crate::request::Request;
use crate::trie::{Trie, RouteTokens};

pub trait Router {
	fn handle(&self, req: Request) -> impl Future<Output = String> + Send;
}

pub trait Middleware: Default + Send + Sync + 'static {
	fn run(&self, req: &mut Request);
}

// todo: example
impl Middleware for u32 {
	fn run(&self, req: &mut Request) { 
		let x: Box<dyn Any + Send> = Box::new(*self);
       	req.context.as_mut().unwrap().insert((*x).type_id(), x);
		println!("{:?}", req);
	}
}

impl Router for &App<()> {
	fn handle(&self, req: Request) -> impl Future<Output = String> + Send {
		App::<()>::handle(self, req)
	}
}

impl<T: Middleware> Router for &App<(T,)> {
	fn handle(&self, mut req: Request) -> impl Future<Output = String> + Send { 
		req.context = Some(HashMap::new());
		T::default().run(&mut req);
		App::<(T,)>::handle(self, req)
	}
}

impl<T: Middleware, Y: Middleware> Router for &App<(T, Y)> {
	fn handle(&self, mut req: Request) -> impl Future<Output = String> + Send {
		req.context = Some(HashMap::new());
		T::default().run(&mut req);
		Y::default().run(&mut req);
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
