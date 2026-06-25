pub mod components {
    pub mod cosmic_entity;
    pub mod debug_text;
    pub mod game_menu;
    pub mod player;
    pub mod tilemap;
}

pub use components::{cosmic_entity::*, debug_text::*, game_menu::*, player::*, tilemap::*};
