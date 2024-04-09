use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub url: String,
    pub token: String,
    pub project_id: String,
    pub window: Size,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn set_width(&mut self, w: u32) {
        self.width = w;
    }

    pub fn set_height(&mut self, h: u32) {
        self.height = h;
    }
}
