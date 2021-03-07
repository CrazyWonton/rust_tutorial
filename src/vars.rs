// Variables hold primitive data or references to data (kind of like Java)
// Variables are default immutable
// Block scoped language

pub fn run() {
	// Variables
	let name = "drenor";

	// Mutable variables
	let mut level = 4;
	println!("My character's name is {} and they are a level {}", name, level);	
	level = 5;
	println!("Now {} is a level {}", name, level);

	// Constants
	const ID: i32 = 001;
    println!("ID: {}", ID);
}
