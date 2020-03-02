use crate::{
    lib::{block::Block, position::Position, widget::Widget},
    COLS, ROWS,
};
use gtk::{prelude::*, Application, ApplicationWindow, Builder, Button, CssProvider, StyleContext};

use std::{collections::HashMap, rc::Rc};

pub fn build_ui<'a>(application: &'a Application) -> Widget {
    let glade_src = include_str!("../../resources/game.glade");
    let builder = Builder::new_from_string(glade_src);
    let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    let mut mines = HashMap::new();

    for x in 0..*COLS {
        for y in 0..*ROWS {
            let id = format!("mine_{}{}", x, y);
            let button: Button = builder.get_object(&id).expect("Couldn't get mine");
            button.set_label("");
            button.set_relief(gtk::ReliefStyle::Normal);
            mines.insert(Position(x, y), Block::new(button));
        }
    }

    let mines_left = builder
        .get_object("mines_left")
        .expect("Couldn't get label for mines");
    let reset = builder
        .get_object("reset")
        .expect("Couldn't get reset button");
    let time = builder
        .get_object("time")
        .expect("Couldn't get label for time left");

    window.show_all();

    let screen = window.get_screen().unwrap();
    let style = CssProvider::new();

    style
        .load_from_data(include_bytes!("../../resources/style.css"))
        .unwrap();
    StyleContext::add_provider_for_screen(&screen, &style, gtk::STYLE_PROVIDER_PRIORITY_USER);

    Widget {
        builder,
        mines: Rc::new(mines),
        window,
        mines_left,
        time,
        reset,
    }
}
