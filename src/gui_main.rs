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
			.default_width(320)
			.default_height(200)
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
/*
        let voice_file = gtk::FileDialog::builder()
            .title("Choose an audio file")
            .build();
        container.append(&voice_file);
*/
		window.set_child(Some(&container));

		window.present();
	});

	app.run()
}
