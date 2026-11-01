use super::super::window_owner_snapshot_from_json;
use super::*;
use agent_desktop_core::ErrorCode;

fn deadline() -> Instant {
    Instant::now() + std::time::Duration::from_secs(1)
}

fn appkit_snapshot() -> WindowOwnerSnapshot {
    let bytes = br#"{
        "applications":[
            {"name":"Mail","pid":10,"bundle_id":"com.apple.mail","launch_time":100.25,"activation_policy":"regular"},
            {"name":"Daemon","pid":12,"launch_time":102.75,"activation_policy":"prohibited"},
            {"name":"Broken","pid":13,"launch_time":-1.0,"activation_policy":"regular"}
        ],
        "frontmost_pid":10,
        "frontmost_launch_time":100.25
    }"#;
    window_owner_snapshot_from_json(bytes, deadline()).unwrap()
}

fn surface(pid: i32, name: &str) -> SurfaceOwner {
    SurfaceOwner {
        pid,
        name: name.into(),
    }
}

fn live(pid: i32) -> Result<Option<String>, AdapterError> {
    Ok(Some(format!("instance-{pid}")))
}

fn cross_uid_denial() -> AdapterError {
    AdapterError::new(ErrorCode::PermDenied, "denied").with_details(serde_json::json!({
        "kind": "process_identity_permission",
    }))
}

#[test]
fn bundle_less_window_owner_joins_the_appkit_owners_with_live_process_identity() {
    let merged = appkit_snapshot()
        .with_surface_owners(vec![surface(40, "sketchybar")], None, live)
        .unwrap();

    let bar = merged.owner(40).unwrap();
    assert_eq!(bar.name, "sketchybar");
    assert_eq!(bar.bundle_id, None);
    assert_eq!(bar.launch_time, None);
    assert!(merged.eligible_pids().contains(&10));
    assert!(merged.eligible_pids().contains(&40));
}

#[test]
fn appkit_prohibited_daemon_with_a_window_is_found_by_its_process_name() {
    let merged = appkit_snapshot()
        .with_surface_owners(vec![surface(12, "secondary_bar")], None, live)
        .unwrap();

    assert_eq!(
        merged.matching_pids("secondary_bar"),
        rustc_hash::FxHashSet::from_iter([12])
    );
    assert!(merged.matching_pids("SECONDARY_BAR").contains(&12));
    assert!(merged.matching_pids("sketchybar").is_empty());
}

#[test]
fn sandbox_lanes_sharing_one_executable_are_selected_by_their_own_names() {
    let merged = appkit_snapshot()
        .with_surface_owners(
            vec![
                surface(40, "sketchybar"),
                surface(41, "sbdev_a1"),
                surface(42, "sbdev_b2"),
            ],
            None,
            live,
        )
        .unwrap();

    assert_eq!(
        merged.matching_pids("sbdev_a1"),
        rustc_hash::FxHashSet::from_iter([41])
    );
    assert_eq!(
        merged.matching_pids("sketchybar"),
        rustc_hash::FxHashSet::from_iter([40])
    );
}

#[test]
fn appkit_known_own_and_skipped_processes_are_never_replaced_or_duplicated() {
    let merged = appkit_snapshot()
        .with_surface_owners(
            vec![
                surface(10, "Mail"),
                surface(77, "agent-desktop"),
                surface(13, "Broken"),
            ],
            Some(77),
            live,
        )
        .unwrap();

    assert_eq!(merged.eligible_pids().len(), 1);
    assert_eq!(merged.owner(10).unwrap().launch_time, Some(100.25));
    assert!(merged.owner(13).is_none());
    assert!(merged.owner(77).is_none());
}

#[test]
fn owners_that_exited_or_belong_to_another_uid_are_left_out() {
    let merged = appkit_snapshot()
        .with_surface_owners(
            vec![
                surface(40, "gone"),
                surface(41, "root-owned"),
                surface(42, "borders"),
            ],
            None,
            |pid| match pid {
                40 => Ok(None),
                41 => Err(cross_uid_denial()),
                _ => live(pid),
            },
        )
        .unwrap();

    assert_eq!(
        merged.eligible_pids(),
        rustc_hash::FxHashSet::from_iter([10, 42])
    );
}

#[test]
fn identity_failures_other_than_cross_uid_denial_fail_the_snapshot() {
    let error = appkit_snapshot()
        .with_surface_owners(vec![surface(40, "sketchybar")], None, |_| {
            Err(AdapterError::new(
                ErrorCode::AppUnresponsive,
                "libproc incomplete",
            ))
        })
        .unwrap_err();

    assert_eq!(error.code, ErrorCode::AppUnresponsive);
}

#[test]
fn a_new_window_owner_changes_the_snapshot_generation() {
    let before = appkit_snapshot();
    let after = appkit_snapshot()
        .with_surface_owners(vec![surface(40, "sketchybar")], None, live)
        .unwrap();

    assert!(before.same_generation(&before.clone()));
    assert!(!before.same_generation(&after));
}

#[test]
fn expired_deadline_is_rejected_before_any_native_read() {
    let error = window_owner_snapshot_with_surfaces_until(Instant::now()).unwrap_err();

    assert_eq!(error.code, ErrorCode::Timeout);
}
