use crate::lib::widget::Widget;

use gio::prelude::*;
use gtk::{
    prelude::*, Application, ApplicationWindow, ApplicationWindowExt, CssProvider, Orientation,
    StyleContext,
};

use std::{cell::RefCell, collections::HashMap, rc::Rc};

fn init_menu_bar_actions<'a>(
    application: &'a Application,
    window: &'a ApplicationWindow,
) -> HashMap<String, Rc<gio::SimpleAction>> {
    let menu = gio::Menu::new();
    let mut menu_bar_actions = HashMap::new();

    menu.append(Some("New Game"), Some("app.new_game"));
    menu.append(Some("Quit"), Some("app.quit"));
    menu.append(Some("Game 1"), Some("app.game_1"));
    menu.append(Some("Game 2"), Some("app.game_2"));
    menu.append(Some("Game 3"), Some("app.game_3"));

    application.set_accels_for_action("app.quit", &["<Primary>Q"]);
    application.set_accels_for_action("app.new_game", &["<Primary>N"]);
    application.set_accels_for_action("app.game_1", &["<Primary>1"]);
    application.set_accels_for_action("app.game_2", &["<Primary>2"]);
    application.set_accels_for_action("app.game_3", &["<Primary>3"]);
    application.set_app_menu(Some(&menu));

    let quit = gio::SimpleAction::new("quit", None);
    application.add_action(&quit);
    menu_bar_actions.insert("quit".to_string(), Rc::new(quit));

    let new_game = gio::SimpleAction::new("new_game", None);
    application.add_action(&new_game);
    menu_bar_actions.insert("new_game".to_string(), Rc::new(new_game));

    let game_1 = gio::SimpleAction::new("game_1", None);
    application.add_action(&game_1);
    menu_bar_actions.insert("game_1".to_string(), Rc::new(game_1));

    let game_2 = gio::SimpleAction::new("game_2", None);
    application.add_action(&game_2);
    menu_bar_actions.insert("game_2".to_string(), Rc::new(game_2));

    let game_3 = gio::SimpleAction::new("game_3", None);
    application.add_action(&game_3);
    menu_bar_actions.insert("game_3".to_string(), Rc::new(game_3));

    menu_bar_actions
}

pub fn build_ui<'a>(application: &'a Application) -> Widget {
    let window = ApplicationWindow::new(application);
    window.set_title("MS Röj");
    window.set_can_focus(true);
    window.set_resizable(true);
    window.set_show_menubar(false);

    let main_widget = gtk::Box::new(Orientation::Vertical, 0);
    main_widget.set_visible(true);
    main_widget.set_can_focus(true);
    main_widget.set_widget_name("main_widget");
    main_widget.get_style_context().add_class("main_widget");

    // #-- top bar
    let top_bar = gtk::Box::new(Orientation::Horizontal, 0);
    top_bar.set_visible(true);
    top_bar.set_can_focus(false);
    top_bar.set_vexpand(false);
    top_bar.set_hexpand(true);
    top_bar.get_style_context().add_class("top_bar");

    let label_mines_left = gtk::Label::new(Some("mines_left"));
    let label_mines_left_box = gtk::Box::new(Orientation::Horizontal, 0);
    label_mines_left.set_visible(true);
    label_mines_left.set_can_focus(false);
    label_mines_left.set_label("10");
    label_mines_left.get_style_context().add_class("label");
    label_mines_left.set_size_request(50, 0);
    label_mines_left.set_hexpand(true);
    label_mines_left.set_halign(gtk::Align::Start);
    // --
    label_mines_left_box.add(&label_mines_left);
    label_mines_left_box.set_halign(gtk::Align::Start);
    label_mines_left_box.set_size_request(100, 0);
    top_bar.add(&label_mines_left_box);
    top_bar.set_child_packing(&label_mines_left_box, true, true, 0, gtk::PackType::Start);

    let button_reset = gtk::Button::new();
    let button_reset_box = gtk::Box::new(Orientation::Horizontal, 0);
    button_reset.set_visible(true);
    button_reset.set_can_focus(false);
    button_reset.set_label(" ");
    button_reset.get_style_context().add_class("reset");
    button_reset.set_receives_default(false);
    button_reset.set_halign(gtk::Align::Center);
    button_reset.set_hexpand(true);
    // --
    button_reset_box.add(&button_reset);
    button_reset_box.set_halign(gtk::Align::Center);
    button_reset_box.set_size_request(100, 0);
    top_bar.add(&button_reset_box);
    top_bar.set_child_packing(&button_reset_box, true, true, 0, gtk::PackType::Start);

    let label_time = gtk::Label::new(Some("time"));
    let label_time_box = gtk::Box::new(Orientation::Horizontal, 0);
    label_time.set_visible(true);
    label_time.set_can_focus(false);
    label_time.set_label("0:00");
    label_time.set_size_request(80, 0);
    label_time.get_style_context().add_class("label");
    label_time.set_halign(gtk::Align::End);
    label_time.set_hexpand(true);
    // --
    label_time_box.add(&label_time);
    label_time_box.set_halign(gtk::Align::End);
    label_time_box.set_size_request(100, 0);
    top_bar.add(&label_time_box);
    top_bar.set_child_packing(&label_time_box, true, true, 0, gtk::PackType::Start);
    // #-- end top bar

    // #-- mines
    let mines_grid = gtk::Grid::new();
    mines_grid.get_style_context().add_class("mines");
    mines_grid.hide_on_delete();
    let mut mines = HashMap::new();

    main_widget.add(&top_bar);
    main_widget.set_child_packing(&top_bar, false, true, 0, gtk::PackType::Start);
    main_widget.add(&mines_grid);
    main_widget.set_child_packing(&mines_grid, true, true, 0, gtk::PackType::Start);

    let menu_bar_actions = init_menu_bar_actions(&application, &window);

    let screen = window.get_screen().unwrap();
    let style = CssProvider::new();

    style.load_from_resource("/resources/style.css");

    StyleContext::add_provider_for_screen(
        &screen,
        &style,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    window.add(&main_widget);
    window.show_all();

    Widget {
        mines: Rc::new(RefCell::new(mines)),
        mines_grid: Rc::new(RefCell::new(mines_grid)),
        window,
        label_mines_left,
        label_time,
        button_reset,
        menu_bar_actions,
    }
}
