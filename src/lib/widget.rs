use crate::lib::{block::Block, position::Position};
use gio::SimpleAction;
use gtk::{ApplicationWindow, Button, Label};
use std::{collections::HashMap, rc::Rc};

#[derive(Clone, Debug)]
pub struct Widget {
    pub window: ApplicationWindow,
    pub mines: Rc<HashMap<Position, Block>>,
    pub label_mines_left: Label,
    pub label_time: Label,
    pub button_reset: Button,
    pub menu_bar_actions: HashMap<String, Rc<SimpleAction>>,
}
