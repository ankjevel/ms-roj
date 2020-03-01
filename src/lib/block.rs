use gtk::Button;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Block(pub Rc<Button>);

impl Block {
    pub fn new(button: Button) -> Self {
        Self(Rc::new(button))
    }
}
