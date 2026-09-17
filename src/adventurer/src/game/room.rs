use super::traits::{
	Name, HasName, impl_has_name_via,
	Greeter,
};

pub struct Room {
	name: Name,
}

impl Room {
	pub fn new(name: &str) -> Self {
		Self {
			name: Name::new(name),
		}
	}
}

pub trait HasRoom {
	fn room(&self) -> &Room;
	fn room_mut(&mut self) -> &mut Room;
}
macro_rules! impl_room {
	($t:ty, $field:ident) => {
		impl HasRoom for $t {
			fn room(&self) -> &Room {
				&self.$field
			}
			fn room_mut(&mut self) -> &mut Room {
				&mut self.$field
			}
		}
	};
}
pub(crate) use impl_room;
impl HasRoom for Room {
	fn room(&self) -> &Room {
		self
	}
	fn room_mut(&mut self) -> &mut Room {
		self
	}
}

impl_has_name_via!(Room, name);

impl Greeter for Room {
	fn greet(&self) {
		println!("Room: {}", self.name());
	}
}