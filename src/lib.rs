pub mod components {
    pub mod game_menu;
    pub mod player;
    pub mod tilemap;
    pub mod debug_text;
}

pub use components::{game_menu::*, player::*, debug_text::*, tilemap::*};
