use std::env;
mod messenger;

#[tokio::main]
async fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 4 {
		println!("Usage: {} token channel_id file", args[0]);
		panic!("Invalid Usage");
	}

	let token: String = args[1].clone();
	let chan: String = args[2].clone();
	let file: String = args[3].clone();

	match messenger::message(token, chan, file).await {
		Ok(()) => {
			return;
		},
		Err(error) => {
			panic!("Error: {:?}", error);
		}
	};
}

