use gtk4 as gtk;
use gtk::{
	prelude::*,
	glib,
	glib::clone,
	FileDialog,
	Application,
	ApplicationWindow,
	Align::{
		Center,
		Start
	}
};
use std::{
	thread,
	time::Duration
};
mod messenger;

#[tokio::main]
async fn main() -> glib::ExitCode {
	let app = Application::builder()
		.application_id("tech.mrman314.voicemessagecord")
		.build();

	app.connect_activate(|app| {
		let window = ApplicationWindow::builder()
			.application(app)
			.default_width(358)
			.default_height(313)
			.title("Discord Voice Message")
			.build();

		let container = gtk::Box::builder()
			.orientation(gtk::Orientation::Vertical)
			.margin_top(24)
			.margin_bottom(24)
			.margin_start(24)
			.margin_end(24)
			.halign(Center)
			.valign(Center)
			.spacing(24)
			.build();

		let title = gtk::Label::builder()
			.label("Discord Voice Message Uploader")
			.halign(Start)
			.build();
		title.add_css_class("title-2");
		container.append(&title);

		let channel_field = gtk::Entry::builder()
			.input_purpose(gtk::InputPurpose::Digits)
			.placeholder_text("Channel ID")
			.build();
		container.append(&channel_field);

		let token_field = gtk::PasswordEntry::builder()
			.placeholder_text("Token")
			.build();
		container.append(&token_field);

		let file_selector = gtk::Box::builder()
			.orientation(gtk::Orientation::Horizontal)
			.halign(Center)
			.valign(Center)
			.spacing(24)
			.build();
		
		let path_field = gtk::Entry::builder()
			.placeholder_text("Enter path")
			.build();

		let file_button = gtk::Button::with_label("Select voice file");
		file_button.connect_clicked(glib::clone!(@weak window, @weak path_field => move |_| {
			let voice_file = gtk::FileDialog::builder()
				.title("Choose an audio file")
				.build();

			voice_file.open(Some(&window), gio::Cancellable::NONE, move |file| {
				if let Ok(file) = file {
					let filename = file.path().expect("Could not get file path.");
					path_field.buffer().set_text(&filename.into_os_string().into_string().unwrap());
				}
			});
		}));
		file_selector.append(&file_button);
		file_selector.append(&path_field);

		container.append(&file_selector);

		let (sender, receiver) = async_channel::bounded(1);

		let run_button = gtk::Button::with_label("Upload!");
		run_button.connect_clicked(move |_| {
			let sender = sender.clone();
			glib::spawn_future_local(clone!(@strong sender, @weak channel_field, @weak token_field, @weak path_field => async move {
				let token = token_field.text().as_str().to_string();
				let channel = channel_field.text().as_str().to_string();
				let path = path_field.text().as_str().to_string();
				sender
					.send_blocking(false)
					.expect("Channel is not open.");
				match messenger::message(token, channel, path).await {
					Ok(()) => println!("OK"),
					Err(error) => println!("{}", error)
				};
				sender
					.send_blocking(true)
					.expect("Channel is not open.");
			}));
		});

		glib::spawn_future_local(clone!(@weak run_button => async move {
			while let Ok(enable_button) = receiver.recv().await {
				run_button.set_sensitive(enable_button);
			}
		}));

		container.append(&run_button);

		window.set_child(Some(&container));

		window.present();
	});

	app.run()
}
