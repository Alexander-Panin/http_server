#[macro_export]
macro_rules! route {
    ( $app:ident, $path:expr, || $handler:expr) => {
		$app.service(
			$path, 
			Box::new(|_req| $handler)
		) 
    };
    ( $app:ident, $path:literal, |$( $x:ident $(:$t:ty)? ),+| $handler:expr ) => {
        {
        	let types: Vec<_> = $path.split('/').zip(0..).filter_map(|(x,i)|{
        		let opt = x.strip_prefix('<').and_then(|x| x.strip_suffix('>'));
    			if let Some(t) = opt { Some((t,i)) } else { None }
	        }).collect();

        	$app.service(
        		$path, 
        		Box::new(move |req| {
        			let v: Vec<_> = req.url.split('/').collect();
        			let mut args = (&types)
        				.into_iter()
        				.map(|(t,i)| (t, v[*i]));
        			$crate::make_call![|$($x,)+| $handler, args, req]
        		})
        	)
        }
    };
}

#[derive(Debug)]
pub enum TT { Int(i32), Usize(usize), Float(f32) }

pub fn mapping(t: &str, x: &str) -> TT {
	use TT::{Float, Usize, Int};
	match t {
		"float" => Float(x.parse::<f32>().unwrap()),
		"usize" => Usize(x.parse::<usize>().unwrap()),
		"int" => Int(x.parse::<i32>().ok().unwrap()),
		_ => panic!("hmm...")
	}
}

#[macro_export]
macro_rules! make_call {
    (|$a:ident,| $handler:expr, $args:ident, $req: ident) => {{
		(move |$a: Request|$handler)($req)
    }};
    (|$a:ident,$b:ident,| $handler:expr, $args:ident, $req:ident) => {{ 
		let fun = move |$a, $b: Request|$handler;
		let (t,x) = $args.next().unwrap();
		fun($crate::routing::mapping(t,x), $req)
    }};
    (|$a:ident,$b:ident,$c:ident,| $handler:expr, $args:ident, $req:ident) => {{ 
		let (t,x) = $args.next().unwrap();
		let (t2,x2) = $args.next().unwrap();
    	let fun = move |$a,$b,$c: Request|$handler;
    	fun($crate::routing::mapping(t,x), $crate::routing::mapping(t2,x2), $req)
    }};
}
