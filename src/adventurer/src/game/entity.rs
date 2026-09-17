use super::traits::{
	Name, HasName, impl_has_name_via,
	Greeter,
};

pub struct Entity {
	pub name: Name,
}

impl Entity {
	pub fn new(name: &str) -> Self {
		Self {
			name: Name::new(name),
		}
	}
}

pub trait HasEntity {
	fn entity(&self) -> &Entity;
	fn entity_mut(&mut self) -> &mut Entity;
}
macro_rules! impl_entity {
	($t:ty, $field:ident) => {
		impl HasEntity for $t {
			fn entity(&self) -> &Entity {
				&self.$field
			}
			fn entity_mut(&mut self) -> &mut Entity {
				&mut self.$field
			}
		}
	};
}
pub(crate) use impl_entity;
impl HasEntity for Entity {
	fn entity(&self) -> &Entity {
		self
	}
	fn entity_mut(&mut self) -> &mut Entity {
		self
	}
}

impl_has_name_via!(Entity, name);

impl Greeter for Entity {
	fn greet(&self) {
		println!("Entity: {}", self.name());
	}
}