mod imp;

use gio::SimpleAction;
use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::{Application, Orientation};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::new(&[("application", app)]).expect("Failed to create Window")
    }
    fn add_actions(&self) {
        // Get state
        let imp = imp::Window::from_instance(self);
        let label = imp.label.get();
        let gtk_box = imp.gtk_box.get();
        let button = imp.button.get();

        // Add stateful action "count" to `window` which takes an integer as parameter
        let action_count =
            SimpleAction::new_stateful("count", Some(&i32::static_variant_type()), &0.to_variant());
        action_count.connect_activate(clone!(@weak label => move |action, parameter| {
            // Get state
            let mut state = action
            .state()
            .expect("Could not get state.")
            .get::<i32>()
            .expect("The value needs to be of type `i32`.");

            // Get parameter
            let parameter = parameter
                .expect("Could not get parameter.")
                .get::<i32>()
                .expect("The value needs to be of type `i32`.");

            // Increase state by parameter and save state
            state += parameter;
            action.set_state(&state.to_variant());

            // Update label with new state
            label.set_label(&format!("Counter: {}", state));
        }));
        self.add_action(&action_count);

        // Add action "quit" to `window` which takes no parameter
        let action_quit = SimpleAction::new("quit", None);
        action_quit.connect_activate(clone!(@weak self as self_ => move |_, _| {
            self_.close();
        }));
        self.add_action(&action_quit);

        // Add stateful action "sensitive-button" to `window` which takes no parameter
        let action_sensitive_button =
            SimpleAction::new_stateful("sensitive-button", None, &true.to_variant());
        action_sensitive_button.connect_activate(clone!(@weak button => move |action, _| {
            // Get state
            let mut state = action
            .state()
            .expect("Could not get state.")
            .get::<bool>()
            .expect("The value needs to be of type `bool`.");

            // Increase state by parameter and save state
            state = !state;
            action.set_state(&state.to_variant());

            // Activate or deactivate button according to current state
            button.set_sensitive(state);

        }));
        self.add_action(&action_sensitive_button);

        // Add stateful action "orientation" to `window` which takes a string as parameter
        let action_orientation = SimpleAction::new_stateful(
            "orientation",
            Some(&String::static_variant_type()),
            &"Vertical".to_variant(),
        );
        action_orientation.connect_activate(clone!(@weak gtk_box => move |action, parameter| {
            // Get parameter
            let parameter = parameter
                .expect("Could not get parameter.")
                .get::<String>()
                .expect("The value needs to be of type `String`.");

            let orientation = match parameter.as_str() {
                "Horizontal" => Orientation::Horizontal,
                "Vertical" => Orientation::Vertical,
                _ => unimplemented!()
            };

            // Set orientation and save state
            gtk_box.set_orientation(orientation);
            action.set_state(&parameter.to_variant());
        }));
        self.add_action(&action_orientation);
    }
}
