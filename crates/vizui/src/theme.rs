use std::any::{type_name, Any, TypeId};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::sync::Arc;

use gpui::*;
use refineable::Refineable;
use serde::Deserialize;

/// A set of strongly-typed setting values defined via multiple JSON files.
pub struct SettingsStore {
    setting_values: HashMap<TypeId, Box<dyn AnySettingValue>>,
    raw_default_settings: serde_json::Value,
    raw_user_settings: serde_json::Value,
    raw_local_settings: BTreeMap<(usize, Arc<Path>), serde_json::Value>,
    tab_size_callback: Option<(
        TypeId,
        Box<dyn Fn(&dyn Any) -> Option<usize> + Send + Sync + 'static>,
    )>,
}

impl SettingsStore {
    /// Get the value of a setting.
    ///
    /// Panics if the given setting type has not been registered, or if there is no
    /// value for this setting.
    pub fn get<T: Settings>(&self, path: Option<(usize, &Path)>) -> &T {
        self.setting_values
            .get(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("unregistered setting type {}", type_name::<T>()))
            .value_for_path(path)
            .downcast_ref::<T>()
            .expect("no default value for setting type")
    }
}

impl Global for SettingsStore {}

struct DeserializedSetting(Box<dyn Any>);

trait AnySettingValue: 'static + Send + Sync {
    fn key(&self) -> Option<&'static str>;
    fn setting_type_name(&self) -> &'static str;
    fn deserialize_setting(&self, json: &serde_json::Value) -> Result<DeserializedSetting>;
    fn load_setting(
        &self,
        default_value: &DeserializedSetting,
        custom: &[DeserializedSetting],
        cx: &mut AppContext,
    ) -> Result<Box<dyn Any>>;
    fn value_for_path(&self, path: Option<(usize, &Path)>) -> &dyn Any;
    fn set_global_value(&mut self, value: Box<dyn Any>);
    fn set_local_value(&mut self, root_id: usize, path: Arc<Path>, value: Box<dyn Any>);
    // fn json_schema(
    //     &self,
    //     generator: &mut SchemaGenerator,
    //     _: &SettingsJsonSchemaParams,
    //     cx: &AppContext,
    // ) -> RootSchema;
}

pub trait Settings: 'static + Send + Sync {
    #[track_caller]
    fn get_global(cx: &AppContext) -> &Self
    where
        Self: Sized,
    {
        cx.global::<SettingsStore>().get(None)
    }
}

pub trait ActiveTheme {
    fn theme(&self) -> &Arc<Theme>;
}

impl ActiveTheme for AppContext {
    fn theme(&self) -> &Arc<Theme> {
        &ThemeSettings::get_global(self).active_theme
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

#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
pub enum Appearance {
    Light,
    Dark,
}

impl Appearance {
    pub fn is_light(&self) -> bool {
        match self {
            Self::Light => true,
            Self::Dark => false,
        }
    }
}

impl From<WindowAppearance> for Appearance {
    fn from(value: WindowAppearance) -> Self {
        match value {
            WindowAppearance::Dark | WindowAppearance::VibrantDark => Self::Dark,
            WindowAppearance::Light | WindowAppearance::VibrantLight => Self::Light,
        }
    }
}

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
