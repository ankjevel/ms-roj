pub mod block;
pub mod tile;

use crate::{
    rand::Rng, BLOCK_HEIGHT, BLOCK_PADDING, BLOCK_WIDTH, COLS, MINES, MIN_SCREEN_HEIGHT,
    MIN_SCREEN_WIDTH, ROWS,
};
use block::Block;
use std::{collections::BTreeMap, io::Stdout};
use tile::Tile;

fn gen_mines() -> Vec<(u16, u16)> {
    let mine = || -> (u16, u16) {
        let mut rng = rand::thread_rng();
        (rng.gen_range(0, *ROWS - 1), rng.gen_range(0, *COLS - 1))
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

pub type Tiles = BTreeMap<Tile, Block>;

pub fn get_offset() -> (u16, u16) {
    let (x_max, y_max);

    if let Ok((x, y)) = termion::terminal_size() {
        x_max = x;
        y_max = y;
    } else {
        panic!("cant get size!");
    }

    if x_max < *MIN_SCREEN_WIDTH || y_max < *MIN_SCREEN_HEIGHT {
        panic!("horrible screensize");
    }

    let (offset_x, offset_y) = (
        (x_max / 2) - (*MIN_SCREEN_WIDTH / 2),
        (y_max / 2) - (*MIN_SCREEN_HEIGHT / 2),
    );

    (offset_x, offset_y)
}

/// get element based on area
pub fn get_element(x: &u16, y: &u16, tiles: &Tiles) -> Result<Tile, ()> {
    let (x, y) = (
        x - x % (*BLOCK_WIDTH + *BLOCK_PADDING),
        y - y % (*BLOCK_HEIGHT + *BLOCK_PADDING),
    );

    let tile = Tile::new(
        &(x / (*BLOCK_WIDTH + *BLOCK_PADDING)),
        &(y / (*BLOCK_HEIGHT + *BLOCK_PADDING)),
    );

    if tiles.contains_key(&tile) {
        Ok(tile.to_owned())
    } else {
        Err(())
    }
}

pub fn gen_field(stdout: &mut Stdout, offset_x: &u16, offset_y: &u16) -> Tiles {
    let mines = gen_mines();

    let mut tiles: Tiles = BTreeMap::new();

    for x in 0..*ROWS {
        for y in 0..*COLS {
            let pos = (x, y);
            let is_mine = mines.contains(&pos);
            tiles.insert(
                Tile::new(&x, &y),
                Block {
                    is_mine,
                    is_clicked: false,
                    is_flagged: false,
                    mines_around: around(&pos, &mines),
                },
            );
        }
    }

    for (tile, block) in &tiles {
        tile.print(stdout, &block, offset_x, offset_y, false);
    }

    tiles
}

fn around(pos: &(u16, u16), mines: &Vec<(u16, u16)>) -> u16 {
    let (x, y) = pos;
    let mut points = vec![(*x, *y), (x + 1, *y), (*x, y + 1), (x + 1, y + 1)];

    let (pos_x, pos_y) = (x > &0, y > &0);

    if pos_y {
        if pos_x {
            points.push((x - 1, y - 1));
        }
        points.push((*x, y - 1));
        points.push((x + 1, y - 1));
    }

    if pos_x {
        points.push((x - 1, *y));
        points.push((x - 1, y + 1));
    }

    points.iter().fold(0, |total, point| {
        if mines.contains(point) {
            total + 1
        } else {
            total
        }
    })
}
