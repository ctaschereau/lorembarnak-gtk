use lorembarnak_rs;

use gtk::{Box, Button, Label};
use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Lorem GTK+");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(200, 200);

    let container = Box::new(gtk::Orientation::Vertical, 5);
    let label = Label::new(Some("Some label"));
    label.set_line_wrap(true);
    let button = Button::with_label("Click me!");

    let label_clone = label.clone();
    button.connect_clicked(move |_| {
        let new_text = lorembarnak_rs::get_text(Some(10));
        label_clone.set_label(new_text.as_str());
    });

    container.add(&button);
    container.add(&label);
    window.add(&container);

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("io.github.ctaschereau.lorem_gtk"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
