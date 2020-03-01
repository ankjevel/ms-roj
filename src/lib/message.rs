use crate::lib::{block::Block, position::Position};

#[derive(Debug, Clone)]
pub enum Message {
    SetTime(String),
    SetMines(u8),
    UpdateButton(Position, Block),
    Reset,
    Quit,
}
