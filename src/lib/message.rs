use crate::lib::{block::Block, position::Position};

#[derive(Debug, Clone)]
pub enum Message {
    SetTime(String),
    SetMines(String),
    UpdateButton(Position, Block, bool),
    End,
    Reset,
    Quit,
}
