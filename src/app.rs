use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::collections::HashMap;
use std::any::Any;
use tokio::time::Duration;
use crate::server::Server;
use crate::request::Request;
use crate::trie::{Trie, RouteTokens};
use crate::routing::{Router, Middleware};

#[derive(Default)]
pub struct App<T> {
	map: Trie<fn(Request, Vec<RouteTokens>) -> String>,
	phantom: PhantomData<T>,
}

unsafe fn make_static<T>(t: &T) -> &'static T {
    std::mem::transmute(t)
}

impl<T: Send + Sync + 'static> App<T> where for<'a> &'a App<T>: Router {
	pub async fn start_with_timeout(&self, addr: &str, timeout: Duration) -> Result<(), Box<dyn Error>> {
	    let mut server = Server::bind(addr, timeout).await?;
	    // safe: `app` should be alive before termination
	    server.accept(unsafe { make_static(self) }).await?;
    	Ok(())
	}

	const TIMEOUT: Duration = Duration::from_millis(100); 
	pub async fn start(&self, addr: &str) -> Result<(), Box<dyn Error>> {
		self.start_with_timeout(addr, Self::TIMEOUT).await	
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

pub type SimpleApp = App<()>;

// todo: example
impl Middleware for u32 {
	fn run(&self, req: &mut Request) { 
		let x: Box<dyn Any + Send> = Box::new(*self);
       	req.context.as_mut().unwrap().insert((*x).type_id(), x);
	}
}

macro_rules! routes_impls {
    ( $x:ident, $( $y:ident $(,)?)* ) => {
        impl<$x: Middleware, $( $y: Middleware ),*> Router for &App<($x, $( $y ),*)> {
			fn handle(&self, mut req: Request) -> impl Future<Output = String> + Send {
				req.context = Some(HashMap::new());
				$x::default().run(&mut req);
				$(
					$y::default().run(&mut req);
				)*
				App::<($x, $( $y ),*)>::handle(self, req)
			}
        }
        routes_impls!($( $y, )*);
    };

    () => {
		impl Router for &App<()> {
			fn handle(&self, req: Request) -> impl Future<Output = String> + Send {
				App::<()>::handle(self, req)
			}
		}
    };
}

routes_impls![A,B,C,D, E,F,G,H, I,J,K,L, M,N,O,P,Q];
