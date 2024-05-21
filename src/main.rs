use std::{
	env,
	fs,
	path::Path,
	io::{
		Error,
		ErrorKind
	}
};

fn main() -> Result<(), Error> {
	let args: Vec<String> = env::args().collect();

	if args.len() != 4 {
		println!("Usage: {} token channel_id file", args[0]);
		return Err(Error::new(ErrorKind::InvalidInput, "Invalid usage"));
	}

	let token: String = args[1].clone();
	let chan: String = args[2].clone();
	let file: String = args[3].clone();

	let size = fs::metadata(file)?.len();
	println!("{}", size);

	Ok(())
}
