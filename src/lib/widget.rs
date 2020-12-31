use crate::lib::{block::Block, position::Position};
use gtk::{ApplicationWindow, Button, Label};
use std::{collections::HashMap, rc::Rc};

#[derive(Clone, Debug)]
pub struct Widget {
    pub window: ApplicationWindow,
    pub mines: Rc<HashMap<Position, Block>>,
    pub label_mines_left: Label,
    pub label_time: Label,
    pub button_reset: Button,
}
