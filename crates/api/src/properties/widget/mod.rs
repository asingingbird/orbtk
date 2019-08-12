// Widget related properties.
pub use self::count::*;
pub use self::font_icon::*;
pub use self::image::*;
pub use self::index::*;
pub use self::name::*;
pub use self::resizeable::*;
pub use self::selection_mode::*;
pub use self::text::*;
pub use self::text_selection::*;
pub use self::title::*;
pub use self::water_mark::*;

mod count;
mod font_icon;
mod image;
mod index;
mod name;
mod resizeable;
mod selection_mode;
mod text;
mod text_selection;
mod title;
mod water_mark;

#[cfg(test)]
mod tests;
