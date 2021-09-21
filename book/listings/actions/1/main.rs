use gio::SimpleAction;
use glib::clone;
use gtk::prelude::*;
use gtk::{gio, glib};
use gtk::{Application, ApplicationWindow};

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk.example")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Set keyboard accelerator to trigger "win.quit".
    app.set_accels_for_action("win.quit", &["<primary>Q"]);

    // Run the application
    app.run();
}

// ANCHOR: build_ui
fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    // Add action "quit" to `window` which takes no parameter
    let action_quit = SimpleAction::new("quit", None);
    action_quit.connect_activate(clone!(@weak window => move |_, _| {
        window.close();
    }));
    window.add_action(&action_quit);

    // Present window to the user
    window.present();
}
// ANCHOR: build_ui
