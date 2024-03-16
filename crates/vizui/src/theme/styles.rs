use gpui::Hsla;
use refineable::Refineable;
use serde::Deserialize;

#[derive(Refineable, Clone, Debug)]
#[refineable(Debug, Deserialize)]
pub struct ThemeColors {
    /// Background Color. Used for the app background and blank panels or windows.
    pub background: Hsla,
    /// Text Color. Default text color used for most text.
    pub text: Hsla,
}

#[derive(Refineable, Clone)]
pub struct ThemeStyles {
    #[refineable]
    pub colors: ThemeColors,
}
