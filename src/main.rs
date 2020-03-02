#![feature(label_break_value)]
#![cfg_attr(
    not(feature = "gtk_3_10"),
    allow(unused_variables, unused_mut, dead_code)
)]

#[macro_use]
extern crate lazy_static;

extern crate gio;
extern crate glib;
extern crate gtk;
extern crate rand;

lazy_static! {
    static ref COLS: u16 = 9;
    static ref ROWS: u16 = 9;
    static ref MINES: u16 = 10;
}

mod lib;

use gio::prelude::*;
use lib::application::Application;
use std::{cell::RefCell, env::args};

fn main() {
    let application =
        gtk::Application::new(Some("com.github.Iteam13337.ms-roj"), Default::default())
            .expect("Initialization failed...");

    application.connect_startup(|app| {
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
