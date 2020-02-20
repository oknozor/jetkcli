pub mod global;
pub mod local;

use global::GlobalSettings;
use local::ProjectSettings;
use local::ProjectSettingsShared;

lazy_static! {
    pub static ref GLOBAL_SETTINGS: GlobalSettings = { GlobalSettings::get().unwrap() };
    pub static ref PROJECT_SETTINGS_SHARED: ProjectSettingsShared =
        { ProjectSettingsShared::get().unwrap() };
    pub static ref PROJECT_SETTINGS: ProjectSettings = { ProjectSettings::get().unwrap() };
}
