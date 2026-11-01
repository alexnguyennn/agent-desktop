use super::{ActivationPolicy, WindowOwner, WindowOwnerSnapshot, ensure_before_deadline};
use crate::system::cg_surface_owners::{self, SurfaceOwner};
use agent_desktop_core::AdapterError;
use std::time::Instant;

/// The AppKit window owners plus every other process that currently has an
/// on-screen window. Bundle-less status-bar daemons such as SketchyBar are
/// absent from AppKit's eligible set but still own capturable windows.
pub(crate) fn window_owner_snapshot_with_surfaces_until(
    deadline: Instant,
) -> Result<WindowOwnerSnapshot, AdapterError> {
    let snapshot = super::window_owner_snapshot_until(deadline)?;
    ensure_before_deadline(deadline)?;
    let candidates = cg_surface_owners::visible_surface_owners_until(deadline)?;
    ensure_before_deadline(deadline)?;
    snapshot.with_surface_owners(
        candidates,
        i32::try_from(std::process::id()).ok(),
        crate::system::process_identity::token_for_pid,
    )
}

impl WindowOwnerSnapshot {
    pub(super) fn with_surface_owners(
        mut self,
        candidates: Vec<SurfaceOwner>,
        own_pid: Option<i32>,
        mut resolve: impl FnMut(i32) -> Result<Option<String>, AdapterError>,
    ) -> Result<Self, AdapterError> {
        for candidate in candidates {
            if Some(candidate.pid) == own_pid
                || self.owner(candidate.pid).is_some()
                || self.skipped_pid(candidate.pid)
            {
                continue;
            }
            match resolve(candidate.pid) {
                Ok(Some(_)) => self.owners.push(WindowOwner {
                    pid: candidate.pid,
                    name: candidate.name,
                    bundle_id: None,
                    launch_time: None,
                    activation_policy: ActivationPolicy::Prohibited,
                }),
                Ok(None) => {}
                Err(error) if crate::system::process_apps::is_cross_uid_identity_error(&error) => {}
                Err(error) => return Err(error),
            }
        }
        self.owners.sort_unstable_by_key(|owner| owner.pid);
        Ok(self)
    }

    fn skipped_pid(&self, pid: i32) -> bool {
        self.skipped
            .iter()
            .any(|record| record["pid"].as_i64() == Some(i64::from(pid)))
    }
}

#[cfg(test)]
#[path = "workspace_apps_surfaces_tests.rs"]
mod tests;
