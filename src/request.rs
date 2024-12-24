#[derive(Default, Debug, Clone)]
pub struct Request {
	pub method: String,
	pub url: String,
	pub version: String,
	pub query: String,
	pub headers: Vec<(String, String)>,
	pub body: String,  
}

fn parse_url(s: String) -> (String, String) {
	if let Some(i) = s.find('?') {
		(s[..i].to_owned(), s[i+1..].to_owned())
	} else {
		(s, "".to_owned())
	}
}

impl Request {
	pub fn build(mut info: Vec<String>, headers: Vec<(String, String)>, body: String) -> Option<Self> {
		let version = info.pop()?;
		let (url, query) = parse_url(info.pop()?);
		let method = info.pop()?;
		Some(Request {method, url, query, version, headers, body})
	}
}