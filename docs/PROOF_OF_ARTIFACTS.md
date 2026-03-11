# Proof of Artifacts — Convention

Standard for adding wire diagrams, screenshots, and UI demos to project READMEs. Makes it easy for reviewers to assess each product from the README page.

---

## Section Template

```markdown
## Proof of Artifacts

*Wire diagrams, screenshots, and demos for quick review.*

### Wire / Architecture

[Diagram: Mermaid flowchart or ASCII]

### Screenshots

| View | Description |
|------|-------------|
| ![Name](docs/artifacts/screenshot-name.png) | Brief caption |

### Demo

[GIF or MP4 embed for UI products]
```

Projects without a UI (CLI, API, daemon) use only the Wire/Architecture subsection.

---

## Asset Layout

| Location | Purpose |
|----------|---------|
| `docs/artifacts/` | Per-project; screenshots, GIFs, MP4s |
| `docs/artifacts/wire.*` | Wire diagram (Mermaid in .md or exported SVG) |

**Naming:** `screenshot-{view}.png`, `demo-{flow}.gif`, `demo-{flow}.mp4`

---

## Wire Diagram Format

- **Primary:** Mermaid (renders on GitHub)
- **Fallback:** ASCII for projects that already have it
- **Export:** Optional SVG in `docs/artifacts/` for high-res

---

## Video / GIF Guidelines

| Format | Use case | Max size |
|--------|----------|----------|
| GIF | Quick UI preview (5–15s) | ~5 MB |
| MP4 | Longer demos, optional sound | ~10 MB |

**Tools:** OBS, QuickTime, or `ffmpeg` to capture; `ffmpeg` to convert.
