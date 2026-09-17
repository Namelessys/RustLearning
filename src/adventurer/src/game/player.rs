use super::entity::{Entity, HasEntity, impl_entity};

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