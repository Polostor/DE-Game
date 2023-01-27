//! This crate implements a plugin group and various events and system
//! parameters used as building blocks for 2D in-game and menu UI across the
//! game.

use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};
pub use button::ButtonCommands;
use button::ButtonPlugin;
pub use commands::GuiCommands;
use focus::FocusPlugin;
pub use focus::SetFocusEvent;
pub use label::LabelCommands;
pub use style::OuterStyle;
use text::TextPlugin;
use textbox::TextBoxPlugin;
pub use textbox::{TextBoxCommands, TextBoxQuery};
use toast::ToastPlugin;
pub use toast::{ToastEvent, ToastLabel};

mod button;
mod commands;
mod focus;
mod label;
mod style;
mod text;
mod textbox;
mod toast;

pub struct GuiPluginGroup;

pub use text::TextProps;

impl PluginGroup for GuiPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(FocusPlugin)
            .add(TextPlugin)
            .add(ButtonPlugin)
            .add(TextBoxPlugin)
            .add(ToastPlugin)
    }
}
