use crate::lib::{gen_mines, position::Position};
use std::{collections::HashMap, time::Instant};

pub type FieldMap = HashMap<Position, Field>;

fn gen(size: u16) -> (Vec<Position>, FieldMap) {
    let mines = gen_mines(size);

    let mut field = HashMap::new();

    for x in 0..size {
        for y in 0..size {
            let pos = Position(x, y);
            let is_mine = mines.contains(&pos);
            field.insert(
                pos,
                Field {
                    is_mine,
                    is_clicked: false,
                    is_flagged: false,
                    mines_around: around(&pos, &mines),
                    adjecent_empty: adjecent_empty(&pos, &mines),
                },
            );
        }
    }

    (mines, field)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Field {
    pub is_mine: bool,
    pub is_clicked: bool,
    pub is_flagged: bool,
    pub mines_around: u16,
    pub adjecent_empty: Vec<Position>,
}

impl Field {
    pub fn mines_around_class_name(&self) -> String {
        format!(
            "btn_nearby_{}",
            match self.mines_around {
                1 => "one",
                2 => "two",
                _ => "multiple",
            }
        )
        .to_string()
    }
}

pub struct Game {
    pub size: u16,
    pub mines: Vec<Position>,
    pub field: FieldMap,
    pub active: bool,
    pub ended: bool,
    pub time: Instant,
}

impl Game {
    pub fn new() -> Self {
        let size = 9;
        let (mines, field) = gen(size);

        Self {
            size,
            mines,
            field,
            active: false,
            ended: false,
            time: Instant::now(),
        }
    }

    pub fn start_timer(&mut self) {
        self.active = true;
        self.time = Instant::now();
    }

    pub fn new_mines(&mut self) {
        let (mines, field) = gen(self.size);

        self.mines = mines;
        self.field = field;
        self.active = false;
        self.ended = false;
        self.time = Instant::now();
    }
}

fn p_around(pos: &Position) -> Vec<(u16, u16)> {
    let (x, y) = (pos.0, pos.1);
    let mut points = vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)];

    let (pos_x, pos_y) = (x > 0, y > 0);

    if pos_y {
        if pos_x {
            points.push((x - 1, y - 1));
        }
        points.push((x, y - 1));
        points.push((x + 1, y - 1));
    }

    if pos_x {
        points.push((x - 1, y));
        points.push((x - 1, y + 1));
    }

    points
}

fn around(pos: &Position, mines: &Vec<Position>) -> u16 {
    p_around(&pos).iter().fold(0, |total, (x, y)| {
        if mines.contains(&Position(*x, *y)) {
            total + 1
        } else {
            total
        }
    })
}

fn adjecent_empty(pos: &Position, mines: &Vec<Position>) -> Vec<Position> {
    p_around(&pos)
        .iter()
        .filter_map(|(x, y)| {
            let position = Position(*x, *y);
            if mines.contains(&position) {
                None
            } else {
                Some(position.to_owned())
            }
        })
        .collect()
}
