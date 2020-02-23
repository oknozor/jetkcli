pub mod global;
pub mod internal;
pub mod private;
pub mod shared;

use global::GlobalSettings;
use private::ProjectSettings;
use shared::ProjectSettingsShared;

lazy_static! {
    pub static ref GLOBAL_SETTINGS: GlobalSettings = { GlobalSettings::get().unwrap() };
    pub static ref PROJECT_SETTINGS_SHARED: ProjectSettingsShared =
        { ProjectSettingsShared::get().unwrap() };
    pub static ref PROJECT_SETTINGS: ProjectSettings = { ProjectSettings::get().unwrap() };
}
