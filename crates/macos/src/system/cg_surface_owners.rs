use agent_desktop_core::AdapterError;
use rustc_hash::FxHashSet;
use std::time::Instant;

use super::cg_window::{self, WindowDictionary};

/// A process that owns at least one on-screen application-level window,
/// whether or not AppKit lists it as an application.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SurfaceOwner {
    pub(crate) pid: i32,
    pub(crate) name: String,
}

pub(crate) fn visible_surface_owners_until(
    deadline: Instant,
) -> Result<Vec<SurfaceOwner>, AdapterError> {
    cg_window::ensure_before_deadline(deadline)?;
    let owners = owners_from_dictionaries(&cg_window::window_dictionaries()?);
    cg_window::ensure_before_deadline(deadline)?;
    Ok(owners)
}

fn owners_from_dictionaries(dictionaries: &[WindowDictionary]) -> Vec<SurfaceOwner> {
    let mut seen = FxHashSet::default();
    let mut owners = dictionaries
        .iter()
        .filter_map(owner_of_visible_window)
        .filter(|owner| seen.insert(owner.pid))
        .collect::<Vec<_>>();
    owners.sort_unstable_by_key(|owner| owner.pid);
    owners
}

fn owner_of_visible_window(dictionary: &WindowDictionary) -> Option<SurfaceOwner> {
    let layer = cg_window::int_field(dictionary, "kCGWindowLayer")?;
    if !cg_window::is_application_window_layer(layer) {
        return None;
    }
    if !cg_window::bool_field(dictionary, "kCGWindowIsOnscreen").ok()? {
        return None;
    }
    let pid = i32::try_from(cg_window::int_field(dictionary, "kCGWindowOwnerPID")?).ok()?;
    let name = cg_window::string_field(dictionary, "kCGWindowOwnerName")?;
    (pid > 0 && !name.is_empty()).then_some(SurfaceOwner { pid, name })
}

#[cfg(test)]
#[path = "cg_surface_owners_tests.rs"]
mod tests;
