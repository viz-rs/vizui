use std::sync::Arc;

use gpui::{AppContext, Font, Global, Pixels, Result, SharedString};

mod appearance;
pub use appearance::Appearance;

mod settings;
pub use settings::Settings;

mod styles;
pub use styles::{ThemeColors, ThemeStyles};

pub trait ActiveTheme {
    fn theme(&self) -> &Arc<Theme>;
}

impl ActiveTheme for AppContext {
    fn theme(&self) -> &Arc<Theme> {
        &ThemeSettings::get_global(self).active_theme
    }
}

#[derive(Clone)]
pub struct Theme {
    pub id: String,
    pub name: SharedString,
    pub appearance: Appearance,
    pub styles: ThemeStyles,
}

impl Theme {
    /// Returns the [`ThemeColors`] for the theme.
    #[inline(always)]
    pub fn colors(&self) -> &ThemeColors {
        &self.styles.colors
    }

    /// Returns the [`Appearance`] for the theme.
    #[inline(always)]
    pub fn appearance(&self) -> Appearance {
        self.appearance
    }
}

#[derive(Clone)]
pub struct ThemeSettings {
    pub ui_font_size: Pixels,
    pub ui_font: Font,
    pub buffer_font: Font,
    pub active_theme: Arc<Theme>,
}

impl Settings for ThemeSettings {}
