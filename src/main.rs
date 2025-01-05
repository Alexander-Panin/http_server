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
    
    let mut app = App::<(u32,)>::default();

    route![app, "/", || "Hello world".to_owned()];
    route![app, "/products/<usize>", |id,req| { format!("url {:?} and id {:?}", req.url, id) }];
    route![app, "/measure/<float>/and/<int>", |x,y,_req| format!("{:?} and {:?}", x, y)];
    route![app, "/pong/", |req| format!("XXX {:?}", req.body) ];

    app.start(&addr).await?;
    Ok(())
}
