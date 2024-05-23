use gtk4 as gtk;
use gtk::{
	prelude::*,
	glib,
	FileDialog,
	Application,
	ApplicationWindow,
	Align::{
		Center,
		Start
	}
};

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
			.placeholder_text("Channel ID")
			.build();
		container.append(&channel_field);

		let token_field = gtk::Entry::builder()
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

		let run_button = gtk::Button::with_label("Upload!");
		container.append(&run_button);

		window.set_child(Some(&container));

		window.present();
	});

	app.run()
}
