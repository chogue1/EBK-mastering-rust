// semicolon.rs
fn main() {
	let result = if 1 == 2 {
		"Nothing makes sense";
	} else {
		"Sanity reigns";
	};
	
	println!("Result of computation: {:?}", result);
}

//it changed the println! expression slightly so it can print out an empty type

//if the println!("Result of computation: {}", result); was implemented instead it would not work
