# Kova

## Proof of Artifacts

*Wire diagrams, screenshots, and demos for quick review.*

### Wire / Architecture

```mermaid
flowchart TB
    User[User] --> GUI[GUI egui]
    User --> Web[Web Client]
    GUI --> Serve[serve API]
    Web --> Serve
    Serve --> Intent[Intent]
    Intent --> Plan[Plan]
    Plan --> Compute[Compute]
    Compute --> C2[c2 broadcast]
```

### Screenshots

| View | Description |
|------|-------------|
| ![GUI](docs/artifacts/screenshot-gui.png) | GUI window |
| ![Web](docs/artifacts/screenshot-web.png) | Web client |

### Demo

*Add `docs/artifacts/demo-gui.gif` for GUI or web flow.*

---

Augment engine. Hybrid: native egui GUI + web client, shared API. Tokenized orchestration (f18–f23), c2 broadcast, hive sync.

## Build

```bash
cargo build -p kova --features serve
cargo run -p kova
```

## Docs

- [docs/HIVE_BLAZING.md](docs/HIVE_BLAZING.md) — Parallel sync + broadcast
- [docs/compression_map.md](docs/compression_map.md) — Tokenization
