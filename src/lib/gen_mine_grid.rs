use crate::lib::{block::Block, message::Message, position::Position, widget::Widget};

use gtk::{prelude::*, Button};

pub fn gen_mine_grid(widget: &Widget, tx: glib::Sender<Message>, size: u16) {
    let mut mines = widget.mines.borrow_mut();
    let grid = widget.mines_grid.borrow_mut();

    for (_, block) in mines.iter() {
        grid.remove(&*block.0);
        unsafe { block.0.destroy() }
    }

    mines.clear();

    for y in 0..size {
        for x in 0..size {
            let mine = Button::new();
            mine.set_label(" ");
            mine.set_can_focus(true);
            mine.set_focus_on_click(false);
            mine.set_receives_default(false);
            mine.set_border_width(0);
            mine.set_size_request(40, 40);
            mine.get_style_context().add_class("mine");
            grid.add(&mine);
            grid.set_cell_left_attach(&mine, y as i32);
            grid.set_cell_top_attach(&mine, x as i32);
            mines.insert(Position(x, y), Block::new(mine));
        }
    }

    grid.show_all();

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
}
