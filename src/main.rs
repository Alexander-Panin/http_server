use std::env;
use std::error::Error;
use http_server::request::Request;
use http_server::app::App;
use http_server::route;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:8080".to_owned());
    
    let mut app = App::default();

    route![app, "/hello/world", |req| format!("XXX {:?}", req.body) ];
    route![app, "/foo/bar/<usize>/<int>", |x,y,_req| { format!("{:?} {:?}", x, y) }];
    route![app, "/", || "Hello world".to_owned()];
    route![app, "/foo/bar/<float>", |x,_req| format!("XXX {:?}", x) ];

    app.start(&addr).await?;
    Ok(())
}



