fn main() {
	
	fn print_name(name: String) -> String {
	    println!("{}", name);
		 name
	}
	
	let mut name = String::from("Alice");
	
	name = print_name(name);	
	
	
	println!("{}", name);
}
