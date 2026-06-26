pub mod components {
    pub mod debug_text;
    pub mod game_menu;
    pub mod mouse;
    pub mod player;
    pub mod tilemap;
    pub mod cosmic_components {

        pub mod cosmic_entity;
        pub mod cosmic_resource;
        pub mod cosmic_type;
    }
}

pub use components::{cosmic_components::{cosmic_entity::*,cosmic_resource::*, cosmic_type::CosmicType}, debug_text::*, game_menu::*, mouse::*, player::*, tilemap::*};
