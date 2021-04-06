pub mod application;
pub mod block;
pub mod game;
pub mod icons;
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
    COLS, MINES, ROWS,
};

use gtk::prelude::*;
use std::{cell::RefCell, collections::HashSet, rc::Rc};

pub fn gen_mines() -> Vec<Position> {
    let mine = || -> Position {
        let mut rng = rand::thread_rng();
        Position(rng.gen_range(0, *ROWS - 1), rng.gen_range(0, *COLS - 1))
    };

    let mut mines = vec![];

    while mines.len() < *MINES as usize {
        let mine = mine();
        if mines.contains(&mine) {
            continue;
        }
        mines.push(mine.to_owned())
    }

    mines
}

fn get_adjecent(
    pos: &Position,
    field: &FieldMap,
    collected: &HashSet<Position>,
) -> Vec<(Position, Field)> {
    let mut vec: Vec<(isize, isize)> = vec![(1, 0), (0, 1)];

    if pos.0 > 0 {
        vec.push((-1, 0));
    }

    if pos.1 > 0 {
        vec.push((0, -1));
    }

    vec.into_iter()
        .filter_map(|(x, y)| {
            let (x, y) = ((pos.0 as isize + x) as u16, (pos.1 as isize + y) as u16);
            let tile = Position(x, y);
            let res = field.get(&tile);

            if res.is_none() {
                None
            } else {
                let tile = tile.to_owned();
                let block = res.unwrap().to_owned();

                if collected.contains(&tile) {
                    None
                } else {
                    Some((tile, block))
                }
            }
        })
        .collect()
}

pub fn get_tiles_around(
    pos: &Position,
    block: &Field,
    field_map: &FieldMap,
) -> Vec<(Position, Field)> {
    if block.mines_around != 0 {
        return vec![];
    }

    let mut found = vec![];
    let mut ignore = HashSet::new();
    let mut check_from = vec![pos.to_owned()];
    loop {
        let adj = check_from
            .iter()
            .filter_map(|tile| {
                let adj = get_adjecent(&tile, &field_map, &ignore);

                if adj.is_empty() {
                    None
                } else {
                    Some(adj.to_owned())
                }
            })
            .flat_map(|adj| adj.to_owned())
            .collect::<Vec<(Position, Field)>>();

        check_from = adj
            .iter()
            .filter_map(|(tile, block)| {
                if block.mines_around == 0 {
                    Some(tile.to_owned())
                } else {
                    None
                }
            })
            .collect();

        if adj.len() == 0 {
            break;
        }

        for (tile, _block) in &adj {
            ignore.insert(tile.to_owned());
        }

        found.extend(adj);
    }

    found
}

fn flood(flood_widget: Rc<Widget>, flood_game: Rc<RefCell<Game>>, position: &Position) {
    let mut positions = Vec::new();
    {
        let game = flood_game.borrow();
        if let Some(field) = game.field.get(&position) {
            let field_ref = game.field.to_owned();
            let positions = get_tiles_around(&position, &field, &field_ref)
                .iter()
                .for_each(|(position, _)| positions.push(position.to_owned()));
        }
    }

    if positions.is_empty() {
        return;
    }

    let mut game = flood_game.borrow_mut();
    for position in positions {
        let (mut label, mut class_names) = (" ".to_string(), vec![]);
        if let Some(field) = game.field.get_mut(&position) {
            field.is_clicked = true;

            if field.mines_around != 0 {
                label = field.mines_around.to_string();
                class_names.push("btn_close".to_string());
                class_names.push(field.mines_around_class_name());
            } else {
                class_names.push("btn_empty".to_string());
            }
        }

        if let Some(block) = flood_widget.mines.get(&Position(position.0, position.1)) {
            let button = &block.0;
            // button.set_relief(gtk::ReliefStyle::None);
            button.set_label(&label);
            button.set_can_focus(false);
            let ctx = button.get_style_context();

            for class_name in ctx.list_classes() {
                if class_name.starts_with("btn_") {
                    ctx.remove_class(&class_name);
                }
            }

            for class in class_names {
                ctx.add_class(&class);
            }
        }
    }
}
