use gdk_pixbuf::{InterpType, Pixbuf};
use gio::{MemoryInputStream, NONE_CANCELLABLE};
use gtk::Image;
use std::collections::HashMap;

lazy_static_include_bytes! {
    BOMB => "resources/icons/1F4A3_color.png",
    DEAD => "resources/icons/1F635_color.png",
    FIRE => "resources/icons/1F525_color.png",
    FLAG => "resources/icons/E091_color.png",
    HAPPY => "resources/icons/1F600_color.png",
    PARTY => "resources/icons/1F973_color.png",
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Icon {
    Bomb,
    Dead,
    Fire,
    Flag,
    Happy,
    Party,
}

#[derive(Debug, Clone)]
pub struct Icons {
    icons: HashMap<Icon, Image>,
}

fn k_v() -> Vec<(Icon, &'static [u8])> {
    vec![
        (Icon::Bomb, *BOMB),
        (Icon::Dead, *DEAD),
        (Icon::Fire, *FIRE),
        (Icon::Flag, *FLAG),
        (Icon::Happy, *HAPPY),
        (Icon::Party, *PARTY),
    ]
}

impl Icons {
    pub fn new() -> Self {
        let mut icons = HashMap::new();

        for (icon, data) in k_v() {
            let bytes = glib::Bytes::from(&data.to_vec());
            let stream = MemoryInputStream::from_bytes(&bytes);
            let pixbuf = Pixbuf::from_stream(&stream, NONE_CANCELLABLE).unwrap();
            let scaled = &pixbuf.scale_simple(20, 20, InterpType::Bilinear).unwrap();

            let image = Image::from_pixbuf(Some(scaled));

            icons.insert(Icon::Flag, image);
        }

        Self { icons }
    }

    pub fn get(&self, icon: &Icon) -> Option<&Image> {
        self.icons.get(icon)
    }
}
