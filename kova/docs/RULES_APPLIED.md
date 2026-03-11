<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Rules Applied — Cochranblock Workspace

Rules from `.cursor/rules/` mapped to this workspace. Applied and verified.

---

## Active Rules

| Rule | Scope | How Applied |
|------|-------|-------------|
| **user-preferences** | All | Rust first, descriptive names (except tokenized code per compression_map), direct answers, never cover Cursor |
| **build-plans-first** | All | Plan before execute; wait for approval on 3+ steps, architectural, or destructive changes |
| **slow-is-fast** | All | Depth over speed; verify outputs; use capable models for complex work |
| **ultra-optimization** | All | Match model to task; offload to Gemini for research/commentary; minimize @ context |
| **self-hosted-gitlab** | Git | When GitLab is primary: `origin` → localhost:8929; `git push origin HEAD`. (Current: origin = GitHub.) |
| **gemini-cli-keep-files** | CLI | Advise "yes" when CLI prompts for file writes and paths are expected |
| **kova-gitlab-blueprint** | red-team-recon, gitlab-config, docs | WBS by project; pipeline = validation; artifacts = CDRLs |
| **expert-teams-hotswap** | red-team-recon, gitlab-config, docs | Expert teams; inherit, critique, improve; beat-the-prime |

---

## Workspace Mapping

- **portfolio/** — Rust web server. Tokenization (v0, p0, f14, etc.) follows `kova/docs/compression_map.md`; exempt from "descriptive variables" (that rule targets math/science code).
- **kova/docs/** — Plans, protocol map, compression map, architecture. CDRL-like when Kova structure exists.
- **red-team-recon/, gitlab-config/** — Not present yet; rules apply when added.

---

## Protocol

1. **Before edits**: State plan (build-plans-first).
2. **Complex work**: Use capable model; verify (slow-is-fast).
3. **Git**: Push to `origin` (GitLab).
4. **Tokenized code**: Keep v/p/f/t identifiers per compression map.
