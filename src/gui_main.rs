use gtk4 as gtk;
use gtk::{
	prelude::*,
	glib,
	Application,
	ApplicationWindow,
	Button
};

fn main() -> glib::ExitCode {
	let app = Application::builder()
		.application_id("tech.mrman314.voicemessagecord")
		.build();

	app.connect_activate(|app| {
		let window = ApplicationWindow::builder()
			.application(app)
			.default_width(320)
			.default_height(200)
			.title("Discord Voice Message")
			.build();

		let button = Button::with_label("the");
		button.connect_clicked(|_| {
			println!("test");
		});
		window.set_child(Some(&button));

		window.present();
	});

	app.run()
}
