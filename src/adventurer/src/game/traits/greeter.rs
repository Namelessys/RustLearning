use super::has_name::{HasName};

pub trait Greeter: HasName {
	fn greet(&self) {
		println!("Hello, i am {}", self.name());
	}
}
