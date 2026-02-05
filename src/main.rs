use std::io;
fn main() {
	println!("Guess the Number");
	println!("Please Input your Guess");
	
	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to run");
	println!("You guessed : {}",guess);
}
