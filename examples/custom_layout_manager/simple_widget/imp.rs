use std::cell::{Cell, RefCell};

use gtk::{
    glib::{self, clone},
    prelude::*,
    subclass::prelude::*,
};

use crate::custom_layout::CustomLayout;

#[derive(Default, Debug)]
pub struct SimpleWidget {
    pub backward: Cell<bool>,
    pub tick_id: RefCell<Option<gtk::TickCallbackId>>,
    pub start_time: RefCell<Option<std::time::Instant>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SimpleWidget {
    const NAME: &'static str = "SimpleWidget";
    type Type = super::SimpleWidget;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        // We make use of the custom layout manager
        klass.set_layout_manager_type::<CustomLayout>();
    }
}

impl ObjectImpl for SimpleWidget {
    fn constructed(&self) {
        self.parent_constructed();
        let gesture = gtk::GestureClick::new();
        // Trigger a transition on click
        let obj = self.obj();
        gesture.connect_pressed(clone!(
            #[weak(rename_to = this)]
            obj,
            move |_, _, _, _| {
                this.do_transition();
            }
        ));
        self.obj().add_controller(gesture);
    }

    fn dispose(&self) {
        while let Some(child) = self.obj().first_child() {
            child.unparent();
        }
    }
}

impl WidgetImpl for SimpleWidget {}
