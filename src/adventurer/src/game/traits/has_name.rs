pub struct Name {
	name_internal: String,
}

impl Name {
	pub fn new(name: &str) -> Self {
		Self {
			name_internal: name.to_string(),
		}
	}
}

pub trait HasName {
	fn name(&self) -> &str;
	fn name_mut(&mut self) -> &mut String;

	fn name_set(&mut self, name: &str) {
		*self.name_mut() = name.to_string();
	}
}
macro_rules! impl_has_name_via {
	($t:ty, $field:ident) => {
		impl HasName for $t {
			fn name(&self) -> &str {
				self.$field.name()
			}
			fn name_mut(&mut self) -> &mut String {
				self.$field.name_mut()
			}
		}
	};
}
impl HasName for Name {
	fn name(&self) -> &str {
		&self.name_internal
	}
	fn name_mut(&mut self) -> &mut String {
		&mut self.name_internal
	}
}
pub(crate) use impl_has_name_via;
