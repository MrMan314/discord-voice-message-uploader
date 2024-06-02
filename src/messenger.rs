use std::{
	fs,
	error::Error,
	path::Path,
	cmp::min,
	fmt::Write
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
use async_stream::stream;
use tokio_util::io::ReaderStream;
use futures_util::StreamExt;
use indicatif::{
	ProgressBar,
	ProgressState,
	ProgressStyle
};
use console::style;
use rand::Rng;

const B64SET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const WAVEFORM_LEN: usize = 64;

pub async fn message(token: String, chan: String, file_name: String) -> Result<(), Box<dyn Error>> {
	let file = tokio::fs::File::open(&file_name).await?;
	let size = file.metadata().await.unwrap().len();
	let audio_file = Probe::open(Path::new(&file_name))?.read()?;
	let mut reader_stream: ReaderStream<tokio::fs::File> = ReaderStream::new(file);

	let client = Client::new();

	println!("{} Sending attachment request...", style("[1/3]").bold().dim());

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
	if data["attachments"][0]["upload_url"].as_str() == None {
		Err(data["message"].as_str().unwrap())?;
	}
	let upload_url:			&str = data["attachments"][0]["upload_url"].as_str().unwrap();
	let upload_filename:	&str = data["attachments"][0]["upload_filename"].as_str().unwrap();

	let bar = ProgressBar::new(size);
	bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
		.unwrap()
		.with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
		.progress_chars("#>-"));

	let async_stream = stream! {
		let mut uploaded = 0;
		while let Some(chunk) = reader_stream.next().await {
			if let Ok(chunk) = &chunk {
				let new = min(uploaded + (chunk.len() as u64), size);
				uploaded = new;
				bar.set_position(new);
			}
			yield chunk;
		}
	};

	println!("{} Uploading file...", style("[2/3]").bold().dim());

	let resp = client.put(upload_url)
		.body(reqwest::Body::wrap_stream(async_stream))
		.send()
		.await?
		.text()
		.await?;

	println!("{} Sending message...", style("[3/3]").bold().dim());

	let mut rng = rand::thread_rng();
	let waveform: String = (0..WAVEFORM_LEN)
		.map(|_| {
			B64SET[rng.gen_range(0..B64SET.len())] as char
		})
		.collect();

	let resp = client.post(format!("https://discord.com/api/v9/channels/{}/messages", chan))
		.header(ACCEPT, "*/*")
		.header(AUTHORIZATION, &token)
		.header(CONTENT_TYPE, "application/json")
		.header("X-Super-Properties", "eyJvcyI6ImlPUyIsImJyb3dzZXIiOiJEaXNjb3JkIGlPUyIsImRldmljZSI6ImlQaG9uZTksMyIsInN5c3RlbV9sb2NhbGUiOiJlbi1DQSIsImNsaWVudF92ZXJzaW9uIjoiMTcyLjAiLCJyZWxlYXNlX2NoYW5uZWwiOiJzdGFibGUiLCJicm93c2VyX3VzZXJfYWdlbnQiOiIiLCJicm93c2VyX3ZlcnNpb24iOiIiLCJvc192ZXJzaW9uIjoiMTUuNSIsImNsaWVudF9idWlsZF9udW1iZXIiOjQyNjU2LCJjbGllbnRfZXZlbnRfc291cmNlIjpudWxsLCJkZXNpZ25faWQiOjB9")
		.body(format!("{{\"content\":\"\",\"channel_id\":\"{}\",\"type\":0,\"flags\":8192,\"attachments\":[{{\"id\":\"0\",\"filename\":\"voice-message.ogg\",\"uploaded_filename\":\"{}\",\"duration_secs\":{},\"waveform\":\"{}\"}}]}}", chan, upload_filename, audio_file.properties().duration().as_secs(), waveform))
		.send()
		.await?
		.text()
		.await?;

	println!("Done!");

	Ok(())
}
