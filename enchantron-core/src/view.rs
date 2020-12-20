pub use self::base_view::BaseView;
pub use self::entity_view::EntityView;
pub use self::game_view::GameView;
pub use self::loading_view::LoadingView;
pub use self::loading_view_impl::LoadingViewImpl;
pub use self::main_menu_view::MainMenuView;
pub use self::player_view::*;
pub use self::view_impl_macro::*;

mod base_view;
mod entity_view;
mod game_view;
mod loading_view;
mod loading_view_impl;
mod main_menu_view;
mod player_view;

#[macro_use]
mod view_impl_macro;
