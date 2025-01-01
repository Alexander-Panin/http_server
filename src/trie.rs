use std::collections::HashMap;

pub struct Trie<T> {
	head: Node<T>, 
}

struct Node<T> {
	value: Option<T>,
	map: HashMap<&'static str, Node<T>> 
}

impl<T> Default for Trie<T> {
	fn default() -> Self {
		let head = Node { value: None, map: HashMap::new() };
		Trie { head }
	}
}

impl<T> Trie<T> {
	pub fn insert(&mut self, v: impl IntoIterator<Item = &'static str>, elem: T) {
		let mut node = &mut self.head;
		for s in v {
			if node.map.contains_key(s) { 
				node = node.map.get_mut(s).unwrap();
			} else {
				let next = Node { value: None, map: HashMap::new() };
				node.map.insert(s, next);
				node = node.map.get_mut(s).unwrap(); 
			}
		}
		node.value = Some(elem);
	}

	pub fn get<'a>(&self, v: impl IntoIterator<Item = &'a str>) -> Option<(&T, Vec<RouteTokens>)> {
		let mut node = &self.head;
		let mut args = vec![];
		for s in v {
			let (s, token) = mapping(s);
			if node.map.contains_key(s) { 
				node = node.map.get(s).unwrap();
			} else {
				return None;
			}
			if token != RouteTokens::NaN { args.push(token); }
		}
		node.value.as_ref().map(|fun| (fun, args))
	}
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RouteTokens { Int(i32), Usize(usize), Float(f32), NaN }

fn mapping(s: &str) -> (&str, RouteTokens) {
	use RouteTokens::{ Int, Usize, Float, NaN };
	let is_usize = s.starts_with(|c: char| c.is_ascii_digit());
	let is_int = s.starts_with('-') && s[1..].starts_with(|c: char| c.is_ascii_digit());
	let is_float = (is_usize || is_int) && s.contains('.');
	match [is_float, is_usize, is_int] {
		[true, _, _] => ("<float>", Float(s.parse::<f32>().unwrap())),
		[_, true, _] => ("<usize>", Usize(s.parse::<usize>().unwrap())),
		[_, _, true] => ("<int>", Int(s.parse::<i32>().ok().unwrap())),
		_ => (s, NaN),
	}
}