// SPDX-FileCopyrightText: 2026 roolrz
// SPDX-License-Identifier: Apache-2.0

//! Coalesced terminal events returned by the service to its clients.

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EventFlags(pub u32);

impl EventFlags {
    pub const INPUT_LOSS: u32 = 1;
    pub const ECHO_LOSS: u32 = 2;
    pub const SETTINGS_CHANGED: u32 = 4;
    pub const OUTPUT_RESUMED: u32 = 8;
    pub const SIZE_CHANGED: u32 = 16;
    pub const FOREGROUND_CHANGED: u32 = 32;
    /// Foreground-only request; never broadcast to background leases.
    pub const TERMINATE: u32 = 64;
}

/// A job-scoped request. Lease IDs never repeat within a terminal instance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminationEvent {
    pub lease: u64,
    pub sequence: u64,
}

impl TerminationEvent {
    pub const BYTES: usize = 16;

    #[must_use]
    pub fn encode(self) -> [u8; Self::BYTES] {
        let mut bytes = [0; Self::BYTES];
        bytes[..8].copy_from_slice(&self.lease.to_le_bytes());
        bytes[8..].copy_from_slice(&self.sequence.to_le_bytes());
        bytes
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != Self::BYTES {
            return None;
        }
        let event = Self {
            lease: u64::from_le_bytes(bytes[..8].try_into().ok()?),
            sequence: u64::from_le_bytes(bytes[8..].try_into().ok()?),
        };
        (event.lease != 0 && event.sequence != 0).then_some(event)
    }
}
