fn main() {
	#![allow(unused_variables)]
	
	let mut x = 10;

	
	let r = &mut x;
	
	let mut r = 10;
	
	//#[allow(unused)] //broken LUL
	r = 20;
	
	println!("{}", x);
}
