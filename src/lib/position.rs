use std::cmp::Ordering::{self, Equal, Greater, Less};

#[derive(Debug, Copy, Clone, Hash)]
pub struct Position(pub u16, pub u16);

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Position {}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.1 == other.1 {
            if self.0 < other.0 {
                Less
            } else if self.0 > other.0 {
                Greater
            } else {
                Equal
            }
        } else if self.1 < other.1 {
            Less
        } else if self.1 > other.1 {
            Greater
        } else {
            if self.0 < other.0 {
                Less
            } else if self.0 > other.0 {
                Greater
            } else {
                Equal
            }
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
