use super::entity::{Entity, HasEntity, impl_entity};
use super::traits::{
	Name, HasName, impl_has_name_via,
	Greeter,
};

pub struct Player {
	entity: Entity,
}

impl Player {
	pub fn new(name: &str) -> Self {
		Self {
			entity: Entity::new(name),
		}
	}
}

impl_entity!(Player, entity);
impl_has_name_via!(Player, entity);

impl Greeter for Player {
	fn greet(&self) {
		println!("Hellow, I am {}", self.name());
	}
}