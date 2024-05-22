use std::{
	fs,
	error::Error,
	path::Path
};
use reqwest::{
	Client,
	header::{
		ACCEPT,
		AUTHORIZATION,
		CONTENT_TYPE
	}
};
use serde_json::Value;
use lofty::{
	probe::Probe,
	file::AudioFile
};

pub async fn message(token: String, chan: String, file: String) -> Result<(), Box<dyn Error>> {
	let size = fs::metadata(&file)?.len();
	let file_buffer: Vec<u8> = fs::read(&file)?;
	let audio_file = Probe::open(Path::new(&file))?.read()?;

	let client = Client::new();

	let resp = client.post(format!("https://discord.com/api/v9/channels/{}/attachments", chan))
		.header(ACCEPT, "*/*")
		.header(AUTHORIZATION, &token)
		.header(CONTENT_TYPE, "application/json")
		.body(format!("{{\"files\":[{{\"filename\":\"voice-message.ogg\",\"file_size\":{}}}]}}", size))
		.send()
		.await?
		.text()
		.await?;

	let data: Value = serde_json::from_str(&resp)?;
	let upload_url:			&str = data["attachments"][0]["upload_url"].as_str().unwrap();
	let upload_filename:	&str = data["attachments"][0]["upload_filename"].as_str().unwrap();

	let resp = client.put(upload_url)
		.body(file_buffer)
		.send()
		.await?
		.text()
		.await?;

	let resp = client.post(format!("https://discord.com/api/v9/channels/{}/messages", chan))
		.header(ACCEPT, "*/*")
		.header(AUTHORIZATION, &token)
		.header(CONTENT_TYPE, "application/json")
		.header("X-Super-Properties", "eyJvcyI6ImlPUyIsImJyb3dzZXIiOiJEaXNjb3JkIGlPUyIsImRldmljZSI6ImlQaG9uZTksMyIsInN5c3RlbV9sb2NhbGUiOiJlbi1DQSIsImNsaWVudF92ZXJzaW9uIjoiMTcyLjAiLCJyZWxlYXNlX2NoYW5uZWwiOiJzdGFibGUiLCJicm93c2VyX3VzZXJfYWdlbnQiOiIiLCJicm93c2VyX3ZlcnNpb24iOiIiLCJvc192ZXJzaW9uIjoiMTUuNSIsImNsaWVudF9idWlsZF9udW1iZXIiOjQyNjU2LCJjbGllbnRfZXZlbnRfc291cmNlIjpudWxsLCJkZXNpZ25faWQiOjB9")
		.body(format!("{{\"content\":\"\",\"channel_id\":\"{}\",\"type\":0,\"flags\":8192,\"attachments\":[{{\"id\":\"0\",\"filename\":\"voice-message.ogg\",\"uploaded_filename\":\"{}\",\"duration_secs\":{},\"waveform\":\"////\"}}]}}", chan, upload_filename, audio_file.properties().duration().as_secs()))
		.send()
		.await?
		.text()
		.await?;

	println!("{}", resp);

	Ok(())
}
