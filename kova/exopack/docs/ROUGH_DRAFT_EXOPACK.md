<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Rough Draft → Exopack → Release Pattern

## Overview

Projects under this workspace follow a two-binary model:

1. **Rough draft** (`*-test`) — Built first. Contains functionality, tests, and exopack for quantifiable quality measurement.
2. **Release** (`*`) — Strips tests and exopack. Pure deployment binary.

## Process

```
Build rough draft first
    │
    ├─► *-test binary: functionality + tests + exopack
    │   - Screenshot, video, mocks, interfaces
    │   - Self-reflection tools for quality
    │   - exopack stays forever in test binary
    │
    └─► Release binary: functionality only
        - No tests
        - No exopack
        - Stripped for deployment
```

## Projects

| Project | Test Binary | Release Binary |
|---------|-------------|----------------|
| cochranblock | cochranblock-test | cochranblock |
| oakilydokily | oakilydokily-test | oakilydokily |
| approuter | approuter-test | approuter |
| approuter | approuter-test | approuter |
| kova | kova-test | kova |

## Rule

See `.cursor/rules/rough-draft-exopack.mdc` (§21).
