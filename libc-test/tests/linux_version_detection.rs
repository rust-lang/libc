//! Regression tests for kernel header version detection in the build script.

#[path = "../build/linux_version.rs"]
mod linux_version;

use linux_version::LinuxVersion;

fn detect(defines: &[(&str, &str)]) -> Option<(u32, u32)> {
    let mut version = LinuxVersion::default();
    for &(name, value) in defines {
        version.parse_macro(name, value);
    }
    version.get()
}

#[test]
fn encoded_only() {
    assert_eq!(detect(&[("LINUX_VERSION_CODE", "330240")]), Some((5, 10)));
    assert_eq!(detect(&[("LINUX_VERSION_CODE", "393471")]), Some((6, 0)));
}

#[test]
fn split_macros() {
    assert_eq!(
        detect(&[
            ("LINUX_VERSION_MAJOR", "6"),
            ("LINUX_VERSION_PATCHLEVEL", "12")
        ]),
        Some((6, 12)),
    );
}

#[test]
fn split_macros_take_precedence_in_any_order() {
    let defines = [
        ("LINUX_VERSION_CODE", "330240"),
        ("LINUX_VERSION_MAJOR", "6"),
        ("LINUX_VERSION_PATCHLEVEL", "12"),
    ];
    for order in [
        [0, 1, 2],
        [1, 0, 2],
        [1, 2, 0],
        [0, 2, 1],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        assert_eq!(detect(&order.map(|i| defines[i])), Some((6, 12)));
    }
}

#[test]
fn partial_split_macros_preserve_existing_behavior() {
    for (name, value, expected) in [
        ("LINUX_VERSION_MAJOR", "6", (6, 0)),
        ("LINUX_VERSION_PATCHLEVEL", "12", (0, 12)),
    ] {
        let split = (name, value);
        let encoded = ("LINUX_VERSION_CODE", "330240");
        assert_eq!(detect(&[split]), Some(expected));
        assert_eq!(detect(&[split, encoded]), Some(expected));
        assert_eq!(detect(&[encoded, split]), Some(expected));
    }
}

#[test]
fn missing_or_malformed_encoded_version() {
    assert_eq!(detect(&[]), None);
    for value in ["", "invalid", "-1", "4294967296"] {
        assert_eq!(detect(&[("LINUX_VERSION_CODE", value)]), None);
        assert_eq!(
            detect(&[("LINUX_VERSION_CODE", value), ("LINUX_VERSION_MAJOR", "6")]),
            Some((6, 0)),
        );
    }
}

#[test]
#[should_panic]
fn malformed_split_major_still_panics() {
    detect(&[
        ("LINUX_VERSION_CODE", "330240"),
        ("LINUX_VERSION_MAJOR", "invalid"),
    ]);
}

#[test]
#[should_panic]
fn malformed_split_minor_still_panics() {
    detect(&[
        ("LINUX_VERSION_CODE", "330240"),
        ("LINUX_VERSION_PATCHLEVEL", "invalid"),
    ]);
}
