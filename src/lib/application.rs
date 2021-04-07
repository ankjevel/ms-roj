use crate::{
    lib::{flood, game::Game, message::Message, position::Position, ui::build_ui, widget::Widget},
    MINES,
};
use gtk::prelude::*;
use std::{cell::RefCell, rc::Rc};

pub struct Application {
    pub widget: Rc<Widget>,
    game: Rc<RefCell<Game>>,
}

impl Application {
    pub fn new(app: &gtk::Application) -> Self {
        let (tx, rx): (glib::Sender<Message>, glib::Receiver<Message>) =
            glib::MainContext::channel(glib::PRIORITY_HIGH);

        let app = Application {
            widget: Rc::new(build_ui(app)),
            game: Rc::new(RefCell::new(Game::new())),
        };
        app.update_main_ui_thread(rx);
        app.setup_labels_and_reset(tx.clone());
        app.bind_clock(tx.clone());
        app.bind_click_events(tx.clone())
    }

    fn bind_clock(&self, tx: glib::Sender<Message>) {
        let game = self.game.clone();
        let tx = tx.clone();

        let tick = move || {
            let game = game.borrow();

            if game.active {
                let elapsed = game.time.elapsed();

                let secs = elapsed.as_secs();

                let time = format!(
                    "{minutes}:{seconds:0>2}",
                    minutes = (secs / 60) % 60,
                    seconds = secs % 60,
                )
                .to_string();

                tx.send(Message::SetTime(time)).expect("could not set time");
            }

            glib::Continue(true)
        };

        glib::timeout_add_local(400, tick);
    }

    fn update_main_ui_thread(&self, rx: glib::Receiver<Message>) {
        let show_all_mines_widget = self.widget.clone();
        let show_all_mines_game = self.game.clone();
        let show_all_mines = move || {
            let game = show_all_mines_game.borrow();
            let widget = show_all_mines_widget.clone();

            for mine in &game.mines {
                if let Some(block) = show_all_mines_widget.mines.get(&Position(mine.0, mine.1)) {
                    let button = &block.0;
                    let ctx = button.get_style_context();
                    if ctx.has_class("btn_mine") {
                        continue;
                    }

                    button.set_can_focus(false);
                    button.set_label(" ");
                    ctx.add_class("btn_mine");
                }
            }

            show_all_mines_widget
                .button_reset
                .get_style_context()
                .add_class("end");
        };

        let check_if_completed_widget = self.widget.clone();
        let check_if_completed_game = self.game.clone();
        let check_if_completed = move || {
            let check_if_completed_game = check_if_completed_game.clone();
            let widget = check_if_completed_widget.clone();
            let completed: bool;

            {
                let game = check_if_completed_game.borrow();
                let mines = game
                    .field
                    .iter()
                    .filter_map(|(position, field)| {
                        // && field.is_flagged
                        if game.mines.contains(&position) && !field.is_clicked {
                            Some(position)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<&Position>>();
                let left = game
                    .field
                    .iter()
                    .filter_map(|(position, field)| {
                        if !field.is_clicked {
                            Some(position)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<&Position>>();
                completed = mines.len() == *MINES as usize && mines.len() == left.len();
            }

            if !completed {
                return;
            }

            let mut game = check_if_completed_game.borrow_mut();
            game.ended = true;
            game.active = false;
            widget
                .button_reset
                .get_style_context()
                .add_class("completed");
            widget.label_mines_left.set_label(&"0");
            widget.mines.iter().for_each(|(position, block)| {
                block.0.set_can_focus(false);
            });
        };

        let widget = self.widget.clone();
        let game = self.game.clone();

        let flood_widget = self.widget.clone();
        let flood_game = self.game.clone();
        rx.attach(None, move |msg| {
            match msg {
                Message::Reset => {
                    let ctx = widget.button_reset.get_style_context();
                    clear_all_classes!(ctx);
                    ctx.add_class("reset");
                    widget.label_time.set_label("0:00");
                    widget.label_mines_left.set_label(&format!("{}", *MINES));
                    game.borrow_mut().new_mines();

                    widget.mines.iter().for_each(|(position, block)| {
                        let button = &block.0;
                        button.set_label(" ");
                        button.set_can_focus(true);
                        clear_btn_classes!(button.get_style_context());
                    });
                }
                Message::End => {
                    let mut game = game.borrow_mut();
                    game.ended = true;
                    game.active = false;

                    let ctx = widget.button_reset.get_style_context();
                    clear_all_classes!(ctx);
                    ctx.add_class("completed");
                    widget.label_mines_left.set_label(&"0");
                    widget.mines.iter().for_each(|(position, block)| {
                        block.0.set_can_focus(false);
                    });
                }
                Message::UpdateButton(position, block, flag) => {
                    let button = block.0;
                    let mut empty = false;
                    let mut game_ended = false;

                    'mut_closure: {
                        let mut game = game.borrow_mut();
                        let widget = widget.clone();

                        if game.ended {
                            break 'mut_closure;
                        }

                        if !game.active {
                            game.start_timer();
                        }

                        let field = game.field.get_mut(&position);

                        if field.is_none() {
                            break 'mut_closure;
                        }

                        let mut field = field.unwrap();
                        let ctx = button.get_style_context();

                        if flag && field.is_clicked == false {
                            field.is_flagged = !field.is_flagged;
                            let mut mines: i16 =
                                widget.label_mines_left.get_label().parse().unwrap_or(0);

                            if field.is_flagged {
                                mines -= 1;
                                ctx.add_class("btn_flag");
                            } else {
                                mines += 1;
                                ctx.remove_class("btn_flag");
                            }

                            button.set_label(" ");

                            widget.label_mines_left.set_label(&mines.to_string());

                            break 'mut_closure;
                        }

                        field.is_clicked = true;

                        let (label, class_names) = if field.is_mine {
                            (
                                " ".to_string(),
                                vec!["btn_mine", "btn_mine_clicked"]
                                    .iter()
                                    .map(|str| str.to_string())
                                    .collect(),
                            )
                        } else if field.mines_around == 0 {
                            (" ".to_string(), vec!["btn_empty".to_string()])
                        } else {
                            (
                                field.mines_around.to_string(),
                                vec!["btn_nearby".to_string(), field.mines_around_class_name()],
                            )
                        };

                        if field.is_mine {
                            game.active = false;
                            game.ended = true;
                        }

                        button.set_label(&label);
                        button.set_can_focus(false);

                        let ctx = button.get_style_context();

                        clear_btn_classes!(ctx);

                        for class in &class_names {
                            ctx.add_class(class);
                        }

                        if game.ended {
                            game_ended = true;
                        }

                        if class_names.contains(&"btn_empty".to_string()) {
                            empty = true;
                        }
                    }

                    if empty {
                        flood(flood_widget.clone(), flood_game.clone(), &position);
                    } else if game_ended {
                        show_all_mines();
                    } else {
                        check_if_completed();
                    }
                }
                Message::SetTime(time) => widget.label_time.set_label(&time),
                Message::SetMines(mines) => widget.label_mines_left.set_label(&mines),
                _ => {}
            }
            widget.window.show_all();
            glib::Continue(true)
        });
    }

    fn bind_click_events(self, tx: glib::Sender<Message>) -> Self {
        let widget = self.widget.clone();
        let mine_event = tx.clone();
        let mines = widget.mines.clone();

        mines.iter().for_each(|(position, block)| {
            let send = tx.clone();
            let msg = Message::UpdateButton(position.clone(), block.clone(), true);
            block.0.connect_button_release_event(move |_, event| {
                match event.get_button() {
                    3 => send.send(msg.clone()).expect("couldn't send"),
                    _ => {}
                };
                Inhibit(false)
            });

            let send = tx.clone();
            let msg = Message::UpdateButton(position.clone(), block.clone(), true);
            block.0.connect_key_press_event(move |_, key| {
                match key.get_hardware_keycode() {
                    102 => send.send(msg.clone()).expect("couldn't send"),
                    _ => {}
                }

                Inhibit(false)
            });

            let send = tx.clone();
            let msg = Message::UpdateButton(position.clone(), block.clone(), false);
            block.0.connect_clicked(move |_| {
                send.send(msg.clone()).expect("couldn't send");
            });
        });

        self
    }

    fn setup_labels_and_reset(&self, tx: glib::Sender<Message>) {
        let widget = self.widget.clone();

        widget.label_time.set_label("0:00");
        widget.label_mines_left.set_label(&format!("{}", *MINES));
        widget.button_reset.get_style_context().add_class("reset");

        widget
            .button_reset
            .connect_clicked(move |_| tx.send(Message::Reset).expect("reset error"));
    }
}
