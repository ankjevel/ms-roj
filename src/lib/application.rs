use crate::{
    lib::{game::Game, message::Message, ui::build_ui, widget::Widget},
    MINES,
};
// use glib::clone;
use gtk::prelude::*;
use std::{cell::RefCell, rc::Rc};

pub struct Application {
    pub widget: Rc<Widget>,
    game: Rc<RefCell<Game>>,
}

impl Application {
    pub fn new(app: &gtk::Application) -> Self {
        let (tx, rx): (glib::Sender<Message>, glib::Receiver<Message>) =
            glib::MainContext::channel(glib::PRIORITY_DEFAULT);

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

        gtk::timeout_add(200, tick);
    }

    fn update_main_ui_thread(&self, rx: glib::Receiver<Message>) {
        let widget = self.widget.clone();
        let game = self.game.clone();
        rx.attach(None, move |msg| {
            match msg {
                Message::Reset => {
                    widget.reset.set_label("🙂");
                    widget.time.set_label("0:00");
                    widget.mines_left.set_label(&format!("{}", *MINES));
                    game.borrow_mut().new_mines();

                    widget.mines.iter().for_each(|(position, block)| {
                        block.0.set_label(" ");
                        block.0.set_can_focus(true);
                        block.0.set_relief(gtk::ReliefStyle::Normal);
                    });
                }
                Message::UpdateButton(position, block, flag) => {
                    let button = block.0;
                    let mut game = game.borrow_mut();

                    if game.ended {
                        return glib::Continue(true);
                    }

                    button.set_relief(gtk::ReliefStyle::None);

                    if !game.active {
                        game.start_timer();
                    }

                    if let Some(field) = game.field.get_mut(&position) {
                        if flag && field.is_clicked == false {
                            field.is_flagged = !field.is_flagged;

                            let (label, class) = if field.is_flagged {
                                ("🏴".to_string(), "flag")
                            } else {
                                (" ".to_string(), "")
                            };

                            button.set_relief(gtk::ReliefStyle::Normal);
                            button.set_label(&label);

                            return glib::Continue(true);
                        }

                        field.is_clicked = true;

                        let (label, class) = if field.is_mine {
                            ("🔥".to_string(), "mine")
                        } else if field.mines_around == 0 {
                            (" ".to_string(), "empty")
                        } else {
                            (field.mines_around.to_string(), "close")
                        };

                        if field.is_mine {
                            game.active = false;
                            game.ended = true;
                        }

                        button.set_label(&label);
                        button.set_can_focus(false);
                    }
                }
                Message::SetTime(time) => widget.time.set_label(&time),
                Message::SetMines(mines) => widget.mines_left.set_label(&mines.to_string()),
                _ => {}
            }
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
                match key.get_keyval() {
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

        widget.time.set_label("0:00");
        widget.mines_left.set_label(&format!("{}", *MINES));
        widget.reset.set_label("🙂");

        widget
            .reset
            .connect_clicked(move |_| tx.send(Message::Reset).expect("reset error"));
    }
}
