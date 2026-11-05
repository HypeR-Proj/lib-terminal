<!--
SPDX-FileCopyrightText: 2026 roolrz
SPDX-License-Identifier: Apache-2.0
-->

# lib-terminal

Formal Native terminal client API and wire protocol.

Applications use Terminal and protocol/settings records to configure terminals and
acquire foreground leases. Ordinary byte-stream clients can use std::io without
depending on this library.

See the [implementation entry](src/lib.rs) and [component declaration](component.json)
for the public interfaces, build dependencies and delivered artifacts.

## Boundaries

Queues and line discipline belong to terminal-service, not the client library. Bootstrap
byte streams are independent, and terminal control requires its separately delegated
capability.

## Foreground termination requests

`Settings::signals` enables the configurable `controls.terminate` character
(Ctrl+C by default), independently of canonical input. `Settings::raw()` disables
it and preserves every input byte. Recognized termination characters become
events rather than ordinary input; literal-next quotes the character. Recognition
discards input still buffered before that character, retains subsequent input,
and does not discard output or data already committed in a READ reply.

A launcher uses `Terminal::prepare_job()` and retains the returned `JobEvents`,
delegates only `PreparedJob::client`, then activates the lease with `bind_job()`
and a WAIT-only observer of the application's presentation owner.
The supervisor receives bounded, coalescible `TerminationEvent` records carrying
the lease ID and a monotonically increasing sequence. It decides which processes
to notify and how each process handles the request. Terminal-service has no
process termination rights and does not implement default signal dispositions.

Keep `JobEvents` alive through foreground supervision. Closing it or explicitly
calling `release_child()` revokes the job lease; an individual pipeline member's
exit does not. Presentation-owner exit restores the default settings and display
baseline through the normal configuration transaction; it also ends that owner's
configuration authority. Old records belong only to their original endpoint and lease.
The existing `prepare_child()` / `bind_child()` API still creates Process-owned
leases for session management. Such a foreground lease receives its own
`EventFlags::TERMINATE`; the flag is never broadcast to background leases.
For these leases, uncommitted reads are cancelled until `take_events()`; that
acknowledgement also fences previously queued reads. Coordinate input readers
while taking the event, apply cancellation locally, then resume reads. Already
committed replies remain valid. Job leases leave application read policy to the
supervisor.

## Build and test

Use this checkout inside the [repo-managed workspace](https://github.com/HypeR-Proj/HypeR).
From this repository directory:

```sh
make
make test
```

An absolute `OUT=/path/to/out` selects a shared output directory. The module
entry delegates to build/system, which prepares the installed SDK and declared
dependencies. Native binaries require HypeR; host module checks cover the tests
selected by component.json. A standalone clone does not contain the workspace’s
other source repositories or Native SDK.
