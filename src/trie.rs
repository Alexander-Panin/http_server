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

	pub fn get<'a>(&self, v: impl IntoIterator<Item = &'a str>) -> Option<&T> {
		let mut node = &self.head;
		for s in v {
			let s = transform(s);
			if node.map.contains_key(s) { 
				node = node.map.get(s).unwrap();
			} else {
				return None;
			}
		}
		node.value.as_ref()
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