pub struct Player {
	name: String,
}

impl Player {
	pub fn new(name: &str) -> Self {
		Self {
			name: name.to_string(),
		}
	}
	
	pub fn greet(&self) {
		println!("Hello, i am {}", self.name);
	}
	
	pub fn name_set(&mut self, name: &str) {
		self.name = name.to_string();
	}
	pub fn _name_get(&self) -> &String {
		&self.name
	}
}