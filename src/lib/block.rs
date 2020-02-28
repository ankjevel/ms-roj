#[derive(Debug, Clone)]
pub struct Block {
    pub is_mine: bool,
    pub is_clicked: bool,
    pub is_flagged: bool,
    pub mines_around: u16,
}
