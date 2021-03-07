mod print;

fn main() {
	// Printing 
	print::run();

	// Formatting
	println!("{} have {} cats", "Do I", 413);

	// Positional Arguments
	println!("{0} is a {1}. {0} is also a {2}", "Sydney", "dog", "city");

	// Named Arguments
	println!("{player} is level {number}", player="Drenor", number="4"); 

	// Placeholder Traits
	println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

	// Placeholder for debug trait
	println!("{:?}", (12, true, "Hello"));

	// Basic math
	println!("10 + 10 = {}", 10+10);
}
