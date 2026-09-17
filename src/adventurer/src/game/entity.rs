use super::has_name::HasName;

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

impl<T: HasEntity> HasName for T {
	fn name(&self) -> &str {
		&self.entity().name
	}
	fn name_mut(&mut self) -> &mut String {
		&mut self.entity_mut().name
	}
}

