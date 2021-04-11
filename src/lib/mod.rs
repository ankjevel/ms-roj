pub mod application;
pub mod block;
pub mod game;
pub mod gen_mine_grid;
pub mod message;
pub mod position;
pub mod ui;
pub mod widget;

use crate::{
    lib::{
        game::{Field, FieldMap, Game},
        position::Position,
        widget::Widget,
    },
    rand::Rng,
};

use gtk::prelude::*;
use std::{cell::RefCell, collections::HashSet, mem, rc::Rc};

pub fn gen_mines(size: u16) -> Vec<Position> {
    let mine = || -> Position {
        let mut rng = rand::thread_rng();
        Position(rng.gen_range(0, size - 1), rng.gen_range(0, size - 1))
    };

    let mut mines = vec![];

    let mut total_mines = size + 1;

    if total_mines > 10 {
        total_mines = 10 + ((total_mines - 10) * 3);
    }

    while mines.len() < total_mines as usize {
        let mine = mine();
        if mines.contains(&mine) {
            continue;
        }
        mines.push(mine.to_owned())
    }

    mines
}

pub fn get_tiles_around(pos: &Position, block: &Field, field_map: &FieldMap) -> HashSet<Position> {
    let mut around: HashSet<Position> = HashSet::new();
    if block.mines_around != 0 {
        return around;
    }

    let mut adj = vec![block.to_owned()];
    let mut checked = HashSet::new();
    loop {
        let mut new_adj = vec![];
        adj.iter().for_each(|field| {
            if checked.contains(field) {
                return;
            }
            checked.insert(field.to_owned());

            if field.is_mine || field.mines_around != 0 {
                return;
            }

            &field.adjecent_empty.iter().for_each(|pos| {
                if around.contains(&pos) {
                    return;
                }

                around.insert(pos.to_owned());
                if let Some(field) = field_map.get(pos) {
                    if !field.is_mine || field.mines_around == 0 {
                        new_adj.push(field.to_owned());
                    }
                }
            });
        });

        if new_adj.is_empty() {
            break;
        }

        let _ = mem::replace(&mut adj, new_adj);
    }

    around
}

fn flood(flood_widget: Rc<Widget>, flood_game: Rc<RefCell<Game>>, position: &Position) {
    let mut positions: HashSet<Position> = HashSet::new();

    {
        let game = flood_game.borrow();
        if let Some(field) = game.field.get(&position) {
            let field_ref = game.field.to_owned();
            let _ = mem::replace(
                &mut positions,
                get_tiles_around(&position, &field, &field_ref),
            );
        }
    }

    if positions.is_empty() {
        return;
    }

    let mut game = flood_game.borrow_mut();
    let mut mines_modified = 0;
    for position in positions {
        let (mut label, mut class_names) = (" ".to_string(), vec![]);

        if let Some(field) = game.field.get_mut(&position) {
            field.is_clicked = true;

            if field.mines_around != 0 {
                label = field.mines_around.to_string();
                class_names.push("btn_nearby".to_string());
                class_names.push(field.mines_around_class_name());
            } else {
                class_names.push("btn_empty".to_string());
            }
        }

        if let Some(block) = flood_widget
            .mines
            .borrow_mut()
            .get(&Position(position.0, position.1))
        {
            let button = &block.0;
            button.set_label(&label);
            button.set_can_focus(false);
            let ctx = button.get_style_context();

            if ctx.has_class("btn_flag") {
                mines_modified += 1;
            }

            clear_classes!(ctx, "btn_");
            for class in class_names {
                ctx.add_class(&class);
            }
        }
    }

    if mines_modified != 0 {
        let mut mines: i16 = flood_widget
            .label_mines_left
            .get_label()
            .parse()
            .unwrap_or(0);
        mines += mines_modified;
        flood_widget.label_mines_left.set_label(&mines.to_string());
    }
}
