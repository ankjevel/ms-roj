use crate::lib::{block::Block, position::Position};
use gtk::{ApplicationWindow, Button, Label};
use std::{collections::HashMap, rc::Rc};

#[derive(Clone, Debug)]
pub struct Widget {
    pub mines: Rc<HashMap<Position, Block>>,
    pub mines_left: Label,
    pub time: Label,
    pub reset: Button,
    pub window: ApplicationWindow,
}
