use std::collections::HashMap;

pub struct Trie<T> {
	head: Node<T>, 
}

struct Node<T> {
	value: Option<T>,
	map: HashMap<String, Node<T>> 
}

impl<T> Default for Trie<T> {
	fn default() -> Self {
		let head = Node { value: None, map: HashMap::new() };
		Trie { head }
	}
}

impl<T> Trie<T> {
	pub fn insert<'a>(&mut self, v: impl IntoIterator<Item = &'a str>, elem: T) {
		let mut node = &mut self.head;
		for s in v {
			if node.map.contains_key(s) { 
				node = node.map.get_mut(s).unwrap();
			} else {
				let next = Node { value: None, map: HashMap::new() };
				node.map.insert(s.to_owned(), next);
				node = node.map.get_mut(s).unwrap(); 
			}
		}
		node.value = Some(elem);
	}

	pub fn get<'a>(&self, v: impl IntoIterator<Item = &'a str>) -> Option<(&T, Vec<RouteTokens>)> {
		let mut node = &self.head;
		let mut args = vec![];
		for s in v {
			let t = transform(s);
			if node.map.contains_key(t) { 
				node = node.map.get(t).unwrap();
			} else {
				return None;
			}
			args.push(mapping(t, s));
		}
		args.retain(|t| t != &RouteTokens::NaN);
		node.value.as_ref().map(|fun| (fun, args))
	}
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RouteTokens { Int(i32), Usize(usize), Float(f32), NaN }

fn mapping(t: &str, x: &str) -> RouteTokens {
	use RouteTokens::{Float, Usize, Int, NaN};
	match t {
		"<float>" => Float(x.parse::<f32>().unwrap()),
		"<usize>" => Usize(x.parse::<usize>().unwrap()),
		"<int>" => Int(x.parse::<i32>().ok().unwrap()),
		_ => NaN,
	}
}

fn transform(s: &str) -> &str {
	let is_usize = s.starts_with(|c: char| c.is_ascii_digit());
	let is_float = is_usize && s.contains('.');
	let is_int = s.starts_with('-') && s.contains(|c: char| c.is_ascii_digit());
	match [is_float, is_usize, is_int] {
		[true, _, _] => "<float>",
		[_, true, _] => "<usize>",
		[_, _, true] => "<int>",
		_ => s,
	}
}