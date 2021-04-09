use crate::{
    lib::{block::Block, position::Position, widget::Widget},
    COLS, ROWS,
};

use gio::prelude::*;
use gtk::{prelude::*, Application, ApplicationWindow, CssProvider, Orientation, StyleContext};

use std::{collections::HashMap, rc::Rc};

fn init_menu_bar_actions<'a>(
    application: &'a Application,
    window: &'a ApplicationWindow,
) -> HashMap<String, Rc<gio::SimpleAction>> {
    let menu = gio::Menu::new();
    let mut menu_bar_actions = HashMap::new();

    menu.append(Some("New Game"), Some("app.new_game"));
    menu.append(Some("Quit"), Some("app.quit"));

    application.set_accels_for_action("app.quit", &["<Primary>Q"]);
    application.set_accels_for_action("app.new_game", &["<Primary>N"]);
    application.set_app_menu(Some(&menu));

    let quit = gio::SimpleAction::new("quit", None);
    application.add_action(&quit);
    menu_bar_actions.insert("quit".to_string(), Rc::new(quit));

    let new_game = gio::SimpleAction::new("new_game", None);
    application.add_action(&new_game);
    menu_bar_actions.insert("new_game".to_string(), Rc::new(new_game));

    menu_bar_actions
}

pub fn build_ui<'a>(application: &'a Application) -> Widget {
    let window = ApplicationWindow::new(application);
    window.set_title("MS Röj");
    window.set_can_focus(false);
    window.set_resizable(false);
    window.set_show_menubar(false);

    let main_widget = gtk::Box::new(Orientation::Vertical, 0);
    main_widget.set_visible(true);
    main_widget.set_can_focus(true);
    main_widget.set_widget_name("main_widget");
    main_widget.get_style_context().add_class("main_widget");

    let top_bar = gtk::Box::new(Orientation::Horizontal, 0);
    top_bar.set_visible(true);
    top_bar.set_can_focus(false);
    top_bar.get_style_context().add_class("top_bar");

    let label_mines_left = gtk::Label::new(Some("mines_left"));
    label_mines_left.set_visible(true);
    label_mines_left.set_can_focus(false);
    label_mines_left.set_label("10");
    label_mines_left.get_style_context().add_class("label");
    label_mines_left.set_size_request(50, 0);
    top_bar.add(&label_mines_left);

    let button_reset = gtk::Button::new();
    button_reset.set_visible(true);
    button_reset.set_can_focus(false);
    button_reset.set_label(" ");
    button_reset.get_style_context().add_class("reset");
    button_reset.set_receives_default(false);
    top_bar.add(&button_reset);
    top_bar.set_child_packing(&button_reset, true, false, 0, gtk::PackType::Start);

    let label_time = gtk::Label::new(Some("time"));
    label_time.set_visible(true);
    label_time.set_can_focus(false);
    label_time.set_label("0:00");
    label_time.set_size_request(80, 0);
    label_time.get_style_context().add_class("label");
    top_bar.add(&label_time);

    window.add(&main_widget);
    main_widget.add(&top_bar);
    main_widget.set_child_packing(&top_bar, true, true, 0, gtk::PackType::Start);

    let mines_grid = gtk::Grid::new();
    mines_grid.get_style_context().add_class("mines");

    let mut mines = HashMap::new();

    for y in 0..*ROWS {
        for x in 0..*COLS {
            let mine = gtk::Button::new();
            mine.set_label(" ");
            mine.set_can_focus(true);
            mine.set_focus_on_click(false);
            mine.set_receives_default(false);
            mine.set_border_width(0);
            mine.set_size_request(40, 40);
            mine.get_style_context().add_class("mine");
            mines_grid.add(&mine);
            mines_grid.set_cell_left_attach(&mine, y as i32);
            mines_grid.set_cell_top_attach(&mine, x as i32);
            mines.insert(Position(x, y), Block::new(mine));
        }
    }

    main_widget.add(&mines_grid);
    main_widget.set_child_packing(&mines_grid, true, true, 0, gtk::PackType::End);

    let menu_bar_actions = init_menu_bar_actions(&application, &window);

    let screen = window.get_screen().unwrap();
    let style = CssProvider::new();

    style.load_from_resource("/resources/style.css");
    StyleContext::add_provider_for_screen(&screen, &style, gtk::STYLE_PROVIDER_PRIORITY_USER);

    window.show_all();

    Widget {
        mines: Rc::new(mines),
        window,
        label_mines_left,
        label_time,
        button_reset,
        menu_bar_actions,
    }
}
