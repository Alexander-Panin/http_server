use std::future::Future;
use crate::request::Request;

pub trait Router {
    fn handle(&self, req: Request) -> impl Future<Output = String> + Send;
}

pub trait Middleware: Default + Send + Sync + 'static {
    fn run(&self, req: &mut Request);
}

#[macro_export]
macro_rules! route {
    ( $app:ident, $path:expr, || $handler:expr) => {
		$app.service(
			$path, 
			|_req, _args| $handler
		) 
    };
    ( $app:ident, $path:literal, |$( $x:ident $(:$t:ty)? ),+| $handler:expr ) => {
        {
        	$app.service(
        		$path, 
        		|req, args| $crate::make_call![|$($x,)+| $handler, args, req]
        	)
        }
    };
}

#[macro_export]
macro_rules! make_call {
    (|$a:ident,| $handler:expr, $args:ident, $req: ident) => {{
		(move |$a: Request|$handler)($req)
    }};
    (|$a:ident,$b:ident,| $handler:expr, $args:ident, $req:ident) => {{ 
		(move |$a, $b: Request|$handler)($args[0], $req)
    }};
    (|$a:ident,$b:ident,$c:ident,| $handler:expr, $args:ident, $req:ident) => {{ 
    	(move |$a,$b,$c: Request|$handler)($args[0], $args[1], $req)
    }};
}
