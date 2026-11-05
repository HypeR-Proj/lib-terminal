// SPDX-FileCopyrightText: 2026 roolrz
// SPDX-License-Identifier: Apache-2.0

use hyper_terminal::Settings;

#[test]
fn wire_roundtrip_rejects_reserved_fields_and_conflicting_controls() {
    use hyper_terminal::protocol::{decode_settings, encode_settings};
    for settings in [Settings::default(), Settings::raw()] {
        let bytes = encode_settings(settings);
        assert_eq!(decode_settings(&bytes), Some(settings));
        for length in 0..bytes.len() {
            assert!(decode_settings(&bytes[..length]).is_none());
        }
    }
    let mut bytes = encode_settings(Settings::default());
    bytes[36] = 1;
    assert!(decode_settings(&bytes).is_none());
    let mut settings = Settings::default();
    settings.controls.erase = settings.controls.kill;
    assert!(settings.validate().is_err());
}

#[test]
fn termination_settings_are_independent_of_canonical_input() {
    use hyper_terminal::protocol::{decode_settings, encode_settings};
    let mut settings = Settings::raw();
    settings.signals = true;
    settings.controls.terminate = Some(0);
    assert_eq!(decode_settings(&encode_settings(settings)), Some(settings));
    settings.controls.terminate = None;
    assert_eq!(decode_settings(&encode_settings(settings)), Some(settings));
    let mut bytes = encode_settings(settings);
    bytes[63] = 2;
    assert!(decode_settings(&bytes).is_none());
    settings = Settings::default();
    settings.controls.terminate = settings.controls.stop;
    assert!(settings.validate().is_err());
    settings.controls.terminate = settings.controls.erase;
    assert!(settings.validate().is_err());
}

#[test]
fn termination_event_requires_a_live_lease_and_sequence() {
    use hyper_terminal::TerminationEvent;
    let event = TerminationEvent {
        lease: 8,
        sequence: 19,
    };
    let bytes = event.encode();
    assert_eq!(TerminationEvent::decode(&bytes), Some(event));
    for length in 0..bytes.len() {
        assert!(TerminationEvent::decode(&bytes[..length]).is_none());
    }
    for event in [
        TerminationEvent {
            lease: 0,
            sequence: 1,
        },
        TerminationEvent {
            lease: 1,
            sequence: 0,
        },
    ] {
        assert!(TerminationEvent::decode(&event.encode()).is_none());
    }
}

#[test]
fn formatting_settings_roundtrip_and_reject_nonboolean_values() {
    use hyper_terminal::protocol::{decode_settings, encode_settings};
    for mask in 0..8 {
        let mut settings = Settings {
            visual_kill: mask & 1 != 0,
            echo_erased: mask & 2 != 0,
            ..Settings::default()
        };
        settings.output.uppercase = mask & 4 != 0;
        assert_eq!(decode_settings(&encode_settings(settings)), Some(settings));
    }
    for index in [13, 14, 15] {
        let mut bytes = encode_settings(Settings::default());
        bytes[index] = 2;
        assert!(decode_settings(&bytes).is_none());
    }
}
