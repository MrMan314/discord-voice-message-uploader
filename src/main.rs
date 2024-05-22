use std::{
	env,
	fs,
	path::Path,
    error::Error
};
use reqwest::{
    Client,
    header::{
        ACCEPT,
        AUTHORIZATION,
        CONTENT_TYPE
    }
};

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

	let size = fs::metadata(file)?.len();

    let client = reqwest::Client::new();

    let resp = client.post(format!("https://canary.discord.com/api/v9/channels/{}/attachments", chan))
        .header(ACCEPT, "*/*")
        .header(AUTHORIZATION, &token)
        .header(CONTENT_TYPE, "application/json")
        .body(format!("{{\"files\":[{{\"filename\":\"voice-message.ogg\",\"file_size\":{}}}]}}", size))
        .send()
        .await?
        .text()
        .await?;
    println!("{:#?}", resp);

	Ok(())
}
