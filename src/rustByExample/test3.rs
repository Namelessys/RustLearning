fn main() {
	let mut test = 1;
	
	let test2 = &test;
	
	let ptr = &mut test as *mut i32;
	
	
	
	unsafe {
		
		*ptr = 2;
	}
	
	//println!("{}", ptr);
	
	println!("{}", test2);
	
}