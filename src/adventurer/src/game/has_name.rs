pub trait HasName {
	fn name(&self) -> &str;
	fn name_mut(&mut self) -> &mut String;

	fn greet(&self) {
		println!("Hello, i am {}", self.name());
	}

	fn name_set(&mut self, name: &str) {
		*self.name_mut() = name.to_string();
	}
}