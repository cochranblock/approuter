# OakilyDokily

Hero site with interactive mural embed.

## Proof of Artifacts

*Wire diagrams, screenshots, and demos for quick review.*

### Wire / Architecture

```mermaid
flowchart TB
    User[User] --> Hero[Hero]
    User --> Mural[Mural Section]
    Mural --> MuralWasm[mural-wasm]
```

### Screenshots

| View | Description |
|------|-------------|
| ![Hero](docs/artifacts/screenshot-hero.png) | Hero section |
| ![Mural](docs/artifacts/screenshot-mural.png) | Mural section |

### Demo

*Add `docs/artifacts/demo-scroll.gif` for scroll + mural interaction.*

## Build

See [mural-wasm/README.md](mural-wasm/README.md) for mural build. Oakilydokily is served via approuter.
