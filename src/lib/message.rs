use crate::lib::{block::Block, position::Position};

#[derive(Debug, Clone)]
pub enum Message {
    SetTime(u64),
    SetMines(u8),
    UpdateButton(Position, Block),
    Reset,
    Quit,
}
