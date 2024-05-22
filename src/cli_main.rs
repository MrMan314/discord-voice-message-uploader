use std::{
	env,
	error::Error
};
mod messenger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();

	if args.len() != 4 {
		println!("Usage: {} token channel_id file", args[0]);
		Err("Invalid Usage")?;
	}

	let token: String = args[1].clone();
	let chan: String = args[2].clone();
	let file: String = args[3].clone();

	messenger::message(token, chan, file).await?;
	Ok(())
}

