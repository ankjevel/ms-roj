use crate::{lib::block::Block, BLOCK_HEIGHT, BLOCK_PADDING, BLOCK_WIDTH};
use std::{
    cmp::Ordering::{self, Equal, Greater, Less},
    io::{Stdout, Write},
};
use termion::cursor;

#[derive(Debug, Clone, Hash)]
pub struct Tile {
    pub x: u16,
    pub y: u16,
    top_left: (u16, u16),
    top_right: (u16, u16),
    bottom_left: (u16, u16),
    bottom_right: (u16, u16),
}

impl Tile {
    pub fn print(
        &self,
        stdout: &mut Stdout,
        block: &Block,
        offset_x: &u16,
        offset_y: &u16,
        ended: bool,
    ) {
        let wall = if ended || block.is_clicked || block.is_flagged {
            " ".to_string()
        } else {
            "▒".to_string()
        };

        vec![
            (self.x, self.y, &wall),
            (self.x + 1, self.y, &wall),
            (self.x + 2, self.y, &wall),
            (self.x, self.y + 1, &wall),
            (self.x + 2, self.y + 1, &wall),
            (self.x, self.y + 2, &wall),
            (self.x + 1, self.y + 2, &wall),
            (self.x + 2, self.y + 2, &wall),
        ]
        .iter()
        .for_each(|(x, y, tile)| {
            write!(
                stdout,
                "{}{}",
                cursor::Goto(x + offset_x, y + offset_y),
                tile
            )
            .unwrap();
        });

        let (x, y) = (self.x + 1 + offset_x, self.y + 1 + offset_y);

        write!(
            stdout,
            "{}{}",
            cursor::Goto(x, y),
            if ended || block.is_clicked {
                if ended && block.is_flagged && block.is_mine {
                    "▓".to_string()
                } else if block.is_mine {
                    "🔥".to_string()
                } else if block.mines_around == 0 {
                    " ".to_string()
                } else {
                    block.mines_around.to_string()
                }
            } else if block.is_flagged {
                "▓".to_string()
            } else {
                "▓".to_string()
            }
        )
        .unwrap();
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Tile {}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y == other.y {
            if self.x < other.x {
                Less
            } else if self.x > other.x {
                Greater
            } else {
                Equal
            }
        } else if self.y < other.y {
            Less
        } else if self.y > other.y {
            Greater
        } else {
            if self.x < other.x {
                Less
            } else if self.x > other.x {
                Greater
            } else {
                Equal
            }
        }
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Tile {
    pub fn new(x: &u16, y: &u16) -> Self {
        let (x, y) = (
            x.to_owned() * (*BLOCK_WIDTH + *BLOCK_PADDING),
            y.to_owned() * (*BLOCK_HEIGHT + *BLOCK_PADDING),
        );

        Tile {
            x,
            y,
            top_left: (x, y),
            top_right: (x + *BLOCK_WIDTH - 1, y),
            bottom_left: (x, y + *BLOCK_HEIGHT - 1),
            bottom_right: (x + *BLOCK_WIDTH - 1, y + *BLOCK_WIDTH - 1),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let (first, second, third) = (Tile::new(&0, &0), Tile::new(&1, &0), Tile::new(&2, &0));

        assert_eq!(first.top_left, (0, 0));
        assert_eq!(first.bottom_right, (2, 2));

        assert_eq!(second.top_left, (3 + *BLOCK_PADDING, 0));
        assert_eq!(second.bottom_right, (5 + *BLOCK_PADDING, 2));

        assert_eq!(third.top_left, (6 + (*BLOCK_PADDING * 2), 0));
        assert_eq!(third.bottom_right, (8 + (*BLOCK_PADDING * 2), 2));
    }
}
