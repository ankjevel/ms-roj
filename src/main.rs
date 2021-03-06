#![feature(label_break_value)]
#![allow(unused_variables, unused_mut, dead_code)]

macro_rules! clear_classes {
    ($style_context:expr, $class:expr) => {
        for class_name in $style_context.list_classes() {
            if class_name.starts_with($class) {
                $style_context.remove_class(&class_name);
            }
        }
    };
}

extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate rand;

mod lib;

use gio::prelude::*;
use lib::application::Application;
use std::{cell::RefCell, env::args};

fn main() {
    let application = gtk::Application::new(Some("com.github.ankjevel.ms-roj"), Default::default())
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        let res_bytes = include_bytes!("../resources/resources.gresource");
        let data = glib::Bytes::from(&res_bytes[..]);
        let resource = gio::Resource::from_data(&data).unwrap();
        gio::resources_register(&resource);

        let application = Application::new(app);
        let application_container = RefCell::new(Some(application));
        app.connect_shutdown(move |_| {
            let application = application_container
                .borrow_mut()
                .take()
                .expect("Shutdown called multiple times");
            drop(application);
        });
    });

    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
