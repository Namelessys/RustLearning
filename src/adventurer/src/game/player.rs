use super::entity::{Entity, HasEntity};

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

impl HasEntity for Player {
	fn entity(&self) -> &Entity {
		&self.entity
	}
	fn entity_mut(&mut self) -> &mut Entity {
		&mut self.entity
	}
}
