use crate::request::Request;

pub struct Parser<'a> {
	s: &'a str,
}

impl <'a> Parser<'a> {
	pub fn new(s: &'a str) -> Self {
		Parser { s }
	}
}

impl Parser<'_> {
	pub fn parse(self) -> Option<Request> {
		let (mut info, mut headers) = (vec![], vec![]); 
		let mut iter = self.s.split("\r\n");
		for x in iter.next()?.split(' ') { 
			info.push(x.to_owned()); 
		}
		for x in iter.by_ref() {
			if x.is_empty() { break; }
			let mut b = x.split(": ");
			headers.push((
				b.next()?.to_owned(), 
				b.next()?.to_owned()
			));
		}
		let body = iter.next()?.to_owned(); 
		Request::build(info, headers, body)
	}
}
