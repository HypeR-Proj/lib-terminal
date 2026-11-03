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
