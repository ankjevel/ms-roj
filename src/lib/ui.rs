use crate::{
    lib::{
        block::Block,
        icons::{Icon, Icons},
        position::Position,
        widget::Widget,
    },
    COLS, ROWS,
};

use gtk::{prelude::*, Application, ApplicationWindow, CssProvider, Orientation, StyleContext};

use std::{collections::HashMap, rc::Rc};

pub fn build_ui<'a>(application: &'a Application) -> Widget {
    let icons = Icons::new();

    let window = ApplicationWindow::new(application);
    window.set_title("MS Röj");
    window.set_can_focus(false);
    window.set_resizable(false);
    window.set_show_menubar(false);

    let main_widget = gtk::Box::new(Orientation::Vertical, 0);
    main_widget.set_visible(true);
    main_widget.set_can_focus(true);
    main_widget.set_widget_name("main_widget");

    let top_bar = gtk::Grid::new();
    top_bar.set_visible(true);
    top_bar.set_can_focus(false);
    top_bar.set_border_width(10);
    top_bar.set_orientation(Orientation::Vertical);
    top_bar.set_row_homogeneous(true);
    top_bar.set_column_homogeneous(true);

    let label_mines_left = gtk::Label::new(Some("mines_left"));
    label_mines_left.set_visible(true);
    label_mines_left.set_can_focus(false);
    label_mines_left.set_label("10");

    top_bar.add(&label_mines_left);
    top_bar.set_cell_left_attach(&label_mines_left, 0);
    top_bar.set_cell_top_attach(&label_mines_left, 0);

    let label_time = gtk::Label::new(Some("time"));
    label_time.set_visible(true);
    label_time.set_can_focus(false);
    label_time.set_label("0:00");
    top_bar.add(&label_time);
    top_bar.set_cell_left_attach(&label_time, 1);
    top_bar.set_cell_top_attach(&label_time, 0);

    let button_reset = gtk::Button::new();
    button_reset.set_visible(true);
    button_reset.set_can_focus(false);
    button_reset.set_label(&"".to_string());
    button_reset.set_image(icons.get(&Icon::Happy));
    button_reset.set_receives_default(false);
    top_bar.add(&button_reset);
    top_bar.set_cell_left_attach(&button_reset, 2);
    top_bar.set_cell_top_attach(&button_reset, 0);

    window.add(&main_widget);
    main_widget.add(&top_bar);
    main_widget.set_child_packing(&top_bar, true, true, 0, gtk::PackType::Start);

    let mines_grid = gtk::Grid::new();
    let ctx = mines_grid.get_style_context();

    ctx.add_class("mines");

    let mut mines = HashMap::new();

    for y in 0..*ROWS {
        for x in 0..*COLS {
            let mine = gtk::Button::new();
            mine.reset_style();
            mine.set_label(" ");
            mine.set_can_focus(true);
            mine.set_focus_on_click(false);
            // mine.set_relief(gtk::ReliefStyle::Normal);
            mine.set_receives_default(false);
            mine.set_border_width(2);
            mines_grid.add(&mine);
            mines_grid.set_cell_left_attach(&mine, y as i32);
            mines_grid.set_cell_top_attach(&mine, x as i32);

            mine.set_has_tooltip(true);

            mines.insert(Position(x, y), Block::new(mine));
        }
    }

    main_widget.add(&mines_grid);
    main_widget.set_child_packing(&mines_grid, true, true, 0, gtk::PackType::End);

    window.show_all();

    let screen = window.get_screen().unwrap();
    let style = CssProvider::new();

    style
        .load_from_data(include_bytes!("../../resources/style.css"))
        .unwrap();
    StyleContext::add_provider_for_screen(&screen, &style, gtk::STYLE_PROVIDER_PRIORITY_USER);

    Widget {
        mines: Rc::new(mines),
        window,
        label_mines_left,
        label_time,
        button_reset,
        icons,
    }
}
