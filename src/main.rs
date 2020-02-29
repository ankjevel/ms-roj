#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate termion;

lazy_static! {
    static ref COLS: u16 = 9;
    static ref ROWS: u16 = 9;
    static ref BLOCK_HEIGHT: u16 = 3;
    static ref BLOCK_WIDTH: u16 = 3;
    static ref BLOCK_PADDING: u16 = 1;
    static ref MIN_SCREEN_WIDTH: u16 = 9 * (3 + 1);
    static ref MIN_SCREEN_HEIGHT: u16 = 9 * (3 + 1);
    static ref MINES: u16 = 10;
}

mod lib;

use lib::{block::Block, gen_field, get_element, get_offset, tile::Tile, Tiles};
use std::{
    cmp::max,
    collections::HashSet,
    io::{stdin, stdout, Stdout, Write},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use termion::{
    clear, cursor,
    event::{Event, Key, MouseEvent},
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
};

fn clear(stdout: &mut Stdout, state: &str) {
    write!(stdout, "{}{}{}", clear::All, cursor::Goto(1, 1), state).unwrap();
}

fn get_adjecent(tile: &Tile, tiles: &Tiles, collected: &HashSet<Tile>) -> Vec<(Tile, Block)> {
    let mut vec: Vec<(isize, isize)> = vec![(1, 0), (0, 1)];

    if tile.x > 0 {
        vec.push((-1, 0));
    }

    if tile.y > 0 {
        vec.push((0, -1));
    }

    let (tile_x, tile_y) = (
        tile.x - tile.x % (*BLOCK_WIDTH + *BLOCK_PADDING),
        tile.y - tile.y % (*BLOCK_HEIGHT + *BLOCK_PADDING),
    );

    let (tile_x, tile_y) = (
        (tile_x / (*BLOCK_WIDTH + *BLOCK_PADDING)),
        (tile_y / (*BLOCK_HEIGHT + *BLOCK_PADDING)),
    );

    vec.into_iter()
        .filter_map(|(x, y)| {
            let (x, y) = ((tile_x as isize + x) as u16, (tile_y as isize + y) as u16);
            let tile = Tile::new(&x, &y);
            let res = tiles.get(&tile);

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

fn get_tiles_around(tile: &Tile, block: &Block, tiles: &Tiles) -> Vec<(Tile, Block)> {
    if block.mines_around != 0 {
        return vec![];
    }

    let mut found = vec![];
    let mut ignore = HashSet::new();
    let mut check_from = vec![tile.to_owned()];
    loop {
        let adj = check_from
            .iter()
            .filter_map(|tile| {
                let adj = get_adjecent(&tile, &tiles, &ignore);

                if adj.is_empty() {
                    None
                } else {
                    Some(adj.to_owned())
                }
            })
            .flat_map(|adj| adj.to_owned())
            .collect::<Vec<(Tile, Block)>>();

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

fn end(stdout: &mut Stdout, offset_x: &u16, offset_y: &u16, tiles: &Tiles) {
    print!(
        "{} ############## {} # IT ESPLODE # {} ############## ",
        termion::cursor::Goto(7 + offset_x, 1),
        termion::cursor::Goto(7 + offset_x, 2),
        termion::cursor::Goto(7 + offset_x, 3),
    );

    print_all(stdout, &offset_x, &offset_y, &tiles);
}

fn print_all(stdout: &mut Stdout, offset_x: &u16, offset_y: &u16, tiles: &Tiles) {
    for (tile, block) in tiles {
        tile.print(stdout, &block, &offset_x, &offset_y, true)
    }
}

fn print_score(stdout: &mut Stdout, tiles: &Tiles) {
    write!(
        stdout,
        "{}total mines: {mines}\r\nflagged: {flagged}",
        termion::cursor::Goto(1, 1),
        mines = *MINES,
        flagged = tiles.iter().fold(0, |num, curr| {
            if curr.1.is_flagged {
                num + 1
            } else {
                num
            }
        }),
    )
    .unwrap();
}

fn win(stdout: &mut Stdout, offset_x: &u16, offset_y: &u16, tiles: &Tiles) {
    print!(
        "{} ########## {} # IT WON # {} ########## ",
        termion::cursor::Goto(5 + offset_x, 1),
        termion::cursor::Goto(5 + offset_x, 2),
        termion::cursor::Goto(5 + offset_x, 3),
    );

    print_all(stdout, &offset_x, &offset_y, &tiles);
}

fn main() {
    let (offset_x, offset_y) = get_offset();

    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    clear(&mut stdout, &cursor::Hide.to_string());
    stdout.flush().unwrap();

    let mut field = gen_field(&mut stdout, &offset_x, &offset_y);

    print_score(&mut stdout, &field);

    stdout.flush().unwrap();

    let ended = Arc::new(Mutex::new(false));

    let cloned_ended = ended.clone();
    let elapsed = Instant::now();
    let stdout = Arc::new(Mutex::new(stdout));
    let thread_stdout = stdout.clone();
    let out = stdout.clone();
    thread::spawn(move || loop {
        let start = Instant::now();
        if let Ok(ended) = cloned_ended.try_lock() {
            if *ended {
                break;
            }
        }

        if let Ok(lock) = thread_stdout.try_lock() {
            let mut stdout = lock;
            let elapsed = elapsed.elapsed();

            let secs = elapsed.as_secs();

            let time = format!(
                "{minutes}:{seconds:0>2}",
                minutes = (secs / 60) % 60,
                seconds = secs % 60,
            );

            write!(stdout, "{}elapsed: {}", termion::cursor::Goto(1, 3), time).unwrap();

            stdout.flush().unwrap();
        }

        let duration = start.elapsed().as_millis();
        thread::sleep(Duration::from_millis(max(1000 - duration, 0) as u64));
    });

    for c in stdin.events() {
        let evt = c.unwrap();

        match evt {
            Event::Key(Key::Char('q')) => {
                if let Ok(lock) = stdout.try_lock() {
                    let mut stdout = lock;
                    clear(&mut stdout, &cursor::Show.to_string());
                }
                break;
            }

            Event::Mouse(me) => match me {
                MouseEvent::Press(mouse_button, x, y) => {
                    if *ended.lock().unwrap() {
                        if let Ok(lock) = stdout.try_lock() {
                            let mut stdout = lock;
                            clear(&mut stdout, &cursor::Show.to_string());
                        }
                        break;
                    }

                    if x < offset_x || y < offset_y {
                        continue;
                    }

                    let tile = get_element(&(x - offset_x), &(y - offset_y), &field);

                    if tile.is_err() {
                        continue;
                    }

                    let tile = tile.unwrap();
                    let block = field.get_mut(&tile).unwrap();

                    match mouse_button {
                        termion::event::MouseButton::Left => {
                            block.is_clicked = true;

                            let block = block.to_owned();
                            let tile = tile.to_owned();
                            let field_ref = field.to_owned();

                            if block.is_mine {
                                if let Ok(lock) = stdout.try_lock() {
                                    let mut stdout = lock;
                                    end(&mut stdout, &offset_x, &offset_y, &field_ref);
                                }

                                if let Ok(guard) = ended.try_lock() {
                                    let mut this = guard;
                                    *this = true;
                                }

                                if let Ok(lock) = stdout.try_lock() {
                                    let mut stdout = lock;
                                    tile.print(&mut stdout, &block, &offset_x, &offset_y, false);
                                }

                                continue;
                            }

                            vec![(tile.to_owned(), block.to_owned())]
                                .iter()
                                .chain(get_tiles_around(&tile, &block, &field_ref).iter())
                                .for_each(|(tile, _block)| {
                                    let block = field.get_mut(&tile).unwrap();

                                    if block.is_mine {
                                        return;
                                    }

                                    block.is_clicked = true;

                                    if let Ok(lock) = stdout.try_lock() {
                                        let mut stdout = lock;
                                        tile.print(
                                            &mut stdout,
                                            &block,
                                            &offset_x,
                                            &offset_y,
                                            false,
                                        );
                                    }
                                });
                        }
                        termion::event::MouseButton::Right => {
                            block.is_flagged = !block.is_flagged;
                            if let Ok(lock) = stdout.try_lock() {
                                let mut stdout = lock;
                                tile.print(&mut stdout, &block, &offset_x, &offset_y, false)
                            }
                        }
                        _ => continue,
                    };

                    if field.iter().fold(0, |num, curr| {
                        if curr.1.is_flagged && curr.1.is_mine {
                            num + 1
                        } else {
                            num
                        }
                    }) == *MINES
                    {
                        if let Ok(lock) = stdout.try_lock() {
                            let mut stdout = lock;
                            win(&mut stdout, &offset_x, &offset_y, &field);
                        }
                        if let Ok(guard) = ended.try_lock() {
                            let mut this = guard;
                            *this = true;
                        }
                    }

                    if let Ok(lock) = stdout.try_lock() {
                        let mut stdout = lock;
                        print_score(&mut stdout, &field);

                        stdout.flush().unwrap();
                    }
                }
                _ => {}
            },
            _ => {}
        }
        if let Ok(lock) = out.try_lock() {
            let mut stdout = lock;
            stdout.flush().unwrap();
        }
    }
}
