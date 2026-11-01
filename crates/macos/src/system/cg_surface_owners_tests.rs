use super::*;
use core_foundation::{
    base::{CFType, TCFType},
    boolean::CFBoolean,
    dictionary::CFDictionary,
    number::CFNumber,
    string::CFString,
};

struct Window {
    layer: i32,
    pid: i32,
    name: Option<&'static str>,
    onscreen: Option<bool>,
}

fn dictionary(window: &Window) -> WindowDictionary {
    let mut pairs: Vec<(CFString, CFType)> = vec![
        (
            CFString::new("kCGWindowLayer"),
            CFNumber::from(window.layer).as_CFType(),
        ),
        (
            CFString::new("kCGWindowOwnerPID"),
            CFNumber::from(window.pid).as_CFType(),
        ),
        (
            CFString::new("kCGWindowNumber"),
            CFNumber::from(1).as_CFType(),
        ),
    ];
    if let Some(name) = window.name {
        pairs.push((
            CFString::new("kCGWindowOwnerName"),
            CFString::new(name).as_CFType(),
        ));
    }
    if let Some(onscreen) = window.onscreen {
        pairs.push((
            CFString::new("kCGWindowIsOnscreen"),
            CFBoolean::from(onscreen).as_CFType(),
        ));
    }
    CFDictionary::from_CFType_pairs(&pairs)
}

fn owners(windows: &[Window]) -> Vec<SurfaceOwner> {
    let dictionaries = windows.iter().map(dictionary).collect::<Vec<_>>();
    owners_from_dictionaries(&dictionaries)
}

fn bar(pid: i32) -> Window {
    Window {
        layer: 3,
        pid,
        name: Some("sketchybar"),
        onscreen: Some(true),
    }
}

#[test]
fn an_onscreen_floating_layer_window_names_its_owner() {
    assert_eq!(
        owners(&[bar(50)]),
        [SurfaceOwner {
            pid: 50,
            name: "sketchybar".into()
        }]
    );
}

#[test]
fn normal_layer_windows_name_their_owner_too() {
    let window = Window {
        layer: 0,
        ..bar(51)
    };

    assert_eq!(owners(&[window]).len(), 1);
}

#[test]
fn offscreen_helper_windows_do_not_make_an_owner() {
    let hidden = Window {
        onscreen: Some(false),
        ..bar(52)
    };
    let unreported = Window {
        onscreen: None,
        ..bar(53)
    };

    assert!(owners(&[hidden, unreported]).is_empty());
}

#[test]
fn system_chrome_layers_do_not_make_an_owner() {
    let menu_bar = Window {
        layer: 24,
        ..bar(54)
    };

    assert!(owners(&[menu_bar]).is_empty());
}

#[test]
fn unnamed_or_pidless_windows_do_not_make_an_owner() {
    let unnamed = Window {
        name: None,
        ..bar(55)
    };
    let empty = Window {
        name: Some(""),
        ..bar(56)
    };
    let kernel = Window { pid: 0, ..bar(0) };

    assert!(owners(&[unnamed, empty, kernel]).is_empty());
}

#[test]
fn many_windows_of_one_process_list_it_once_in_pid_order() {
    let secondary = Window {
        name: Some("secondary_bar"),
        ..bar(40)
    };

    let found = owners(&[bar(50), secondary, bar(50), bar(50)]);

    assert_eq!(
        found.iter().map(|owner| owner.pid).collect::<Vec<_>>(),
        [40, 50]
    );
    assert_eq!(found[0].name, "secondary_bar");
}

#[test]
fn expired_deadline_is_rejected_before_native_reads() {
    let error = visible_surface_owners_until(Instant::now()).unwrap_err();

    assert_eq!(error.code.as_str(), "TIMEOUT");
}
