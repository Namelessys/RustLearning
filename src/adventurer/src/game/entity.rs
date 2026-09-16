pub struct Entity {
	name: String,
}

impl Entity {
	pub fn new(name: &str) -> Self {
		Self {
			name: name.to_string(),
		}
	}
}

pub trait HasName {
   fn name(&self) -> &String;
   fn name_mut(&mut self) -> &mut String;
	
   fn greet(&self) {
      println!("Hello, i am {}", self.name());
   }
	
   fn name_set(&mut self, name: &str) {
      *self.name_mut() = name.to_string();
   }
}


//pub trait IEntity {}

pub trait HasEntity {
	fn entity(&self) -> &Entity;
	fn entity_mut(&mut self) -> &mut Entity;
}
impl HasEntity for Entity {
	fn entity(&self) -> &Entity { self }
	fn entity_mut(&mut self) -> &mut Entity { self }
}

impl<T: HasEntity> HasName for T {
	fn name(&self) -> &String {
		&self.entity().name
	}
	fn name_mut(&mut self) -> &mut String {
		&mut self.entity_mut().name
	}
}