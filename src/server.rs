use std::error::Error;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio::time::{timeout, Duration};
use crate::connection::Connection;
use crate::routing::Router;

pub struct Server {
	listener: TcpListener,
	tasks: Vec<JoinHandle<Result<(), tokio::time::error::Elapsed>>>,
	timeout: Duration, 
	_counter: u16,
}

unsafe fn make_mut_static<T>(t: &mut T) -> &'static mut T {
    std::mem::transmute(t)
}

impl Server {
	pub async fn bind(addr: &str, timeout: Duration) -> Result<Server, Box<dyn Error>> {
		let listener = TcpListener::bind(&addr).await?;
	    println!("Listening on: {addr}");
	    Ok(Server{listener, timeout, tasks: vec![], _counter: 0 })
	}

	pub async fn accept(&mut self, app: impl Router + Send + Copy + 'static) -> Result<(), Box<dyn Error>> {
		self.listen_signals();
		loop {
			let (stream, _) = self.listener.accept().await?;
			let task = tokio::spawn(timeout(self.timeout, async move {
				println!("process task on {:?}", std::thread::current().id());
				let mut p = Connection::new(stream);
				p.start(app).await.unwrap();
			}));
			self.register(task);
		}
	}

	pub async fn shutdown(&mut self) {
		for t in self.tasks.drain(..) { let _ = t.await; }
	}

	fn register(&mut self, t: JoinHandle<Result<(), tokio::time::error::Elapsed>>) {
		self.tasks.push(t);
		let (k, next_round) = self._counter.overflowing_add(1);
		self._counter = k;
		if next_round { self.tasks.retain(|t| !t.is_finished()); }
	}

	fn listen_signals(&mut self) {
		// safe: `server` should be alive before termination
		let this = unsafe { make_mut_static(self) };
		tokio::spawn(async move {
		    tokio::signal::ctrl_c().await.unwrap();
		    println!("\nCTRL-C");
		    println!("Shutdown... waiting for the tasks to complete");
		    this.shutdown().await;
		    println!("Done, thx :)");
		    std::process::exit(0);
		});
	}
} 

