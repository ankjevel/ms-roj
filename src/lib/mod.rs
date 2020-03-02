// pub mod block;
// pub mod tile;

pub mod application;
pub mod block;
pub mod game;
pub mod message;
pub mod position;
pub mod ui;
pub mod widget;

use crate::{
    lib::{
        game::{Field, FieldMap},
        position::Position,
    },
    rand::Rng,
    COLS, MINES, ROWS,
};

use std::collections::HashSet;

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
