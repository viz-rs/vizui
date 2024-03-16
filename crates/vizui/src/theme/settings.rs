use std::any::{type_name, Any, TypeId};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::sync::Arc;

use gpui::{AppContext, Global, Result};

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

pub trait Settings: 'static + Send + Sync {
    #[track_caller]
    fn get_global(cx: &AppContext) -> &Self
    where
        Self: Sized,
    {
        cx.global::<SettingsStore>().get(None)
    }
}

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
