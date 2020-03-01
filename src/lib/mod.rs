// pub mod block;
// pub mod tile;

pub mod application;
pub mod block;
pub mod game;
pub mod message;
pub mod position;
pub mod ui;
pub mod widget;

use crate::{lib::position::Position, rand::Rng, COLS, MINES, ROWS};

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

// pub fn gen_field(stdout: &mut Stdout, offset_x: &u16, offset_y: &u16) -> Tiles {
//     let mines = gen_mines();

//     let mut tiles: Tiles = BTreeMap::new();

//     for x in 0..*ROWS {
//         for y in 0..*COLS {
//             let pos = (x, y);
//             let is_mine = mines.contains(&pos);
//             tiles.insert(
//                 Tile::new(&x, &y),
//                 Block {
//                     is_mine,
//                     is_clicked: false,
//                     is_flagged: false,
//                     mines_around: around(&pos, &mines),
//                 },
//             );
//         }
//     }

//     for (tile, block) in &tiles {
//         tile.print(stdout, &block, offset_x, offset_y, false);
//     }

//     tiles
// }

// fn around(pos: &(u16, u16), mines: &Vec<(u16, u16)>) -> u16 {
//     let (x, y) = pos;
//     let mut points = vec![(*x, *y), (x + 1, *y), (*x, y + 1), (x + 1, y + 1)];

//     let (pos_x, pos_y) = (x > &0, y > &0);

//     if pos_y {
//         if pos_x {
//             points.push((x - 1, y - 1));
//         }
//         points.push((*x, y - 1));
//         points.push((x + 1, y - 1));
//     }

//     if pos_x {
//         points.push((x - 1, *y));
//         points.push((x - 1, y + 1));
//     }

//     points.iter().fold(0, |total, point| {
//         if mines.contains(point) {
//             total + 1
//         } else {
//             total
//         }
//     })
// }
