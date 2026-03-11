# Kova Hive Mesh — Usage Gap Analysis

**Purpose:** Compare current usage vs. best-ability use of the kova hive mesh (c2-core + workers + hive), based on USER_STORY docs and plans.

**Date:** 2026-03-11

---

## 1. What the Hive Mesh Is

| Component | Role |
|-----------|------|
| **c2-core** | Mac mini — tunnel, approuter, kova GUI, LLM (Apple GPU) |
| **Workers** | lf, gd, bt, st — Linux machines for parallel builds, heavy cargo |
| **Hive** | ~/hive-vault on c2-core → /mnt/hive on workers (NFS from st) |
| **sync** | `kova c2 sync` — rsync workspace from c2-core to workers |

---

## 2. Capabilities (from c2.rs, inspect.rs, PLAN_GAP_ANALYSIS)

| Command | What it does |
|---------|--------------|
| `kova c2 nodes` | List workers (lf, gd, bt, st) |
| `kova c2 inspect` | CPU, RAM, disk, GPU on c2-core + all workers |
| `kova c2 inspect --recommend` | Placement tips + copy-paste commands |
| `kova c2 sync [--dry-run] <target>` | Rsync workspace to target (lf, gd, bt, st) |
| `kova c2 run f18/f20 --broadcast` | Run pipeline on all reachable workers |
| `kova c2 run f20 --broadcast --nodes lf,gd` | Restrict to specific workers |
| `kova c2 ssh-ca init` | Create SSH host CA |
| `kova c2 ssh-ca sign <node>` | Sign host cert for node |
| `kova c2 ssh-ca setup` | Sign all nodes |

**Tokens:** f18 (cargo check), f19 (clippy), f20 (full pipeline), f21 (tunnel update), f22 (setup roguerepo), f23 (local-only).

---

## 3. Prerequisites for Broadcast

1. **Project under ~/hive-vault** — `to_worker_path` maps ~/hive-vault → /mnt/hive. Broadcast fails otherwise.
2. **Hive synced on workers** — `kova c2 sync` must have run (or equivalent).
3. **SSH to workers** — lf, gd, bt, st reachable (sshallp, SSH config).
4. **Hive dir on workers** — `/mnt/hive/projects/workspace` and `/mnt/hive/projects/{ronin-sites,rogue-repo}` exist.

---

## 4. Sync vs. Broadcast Path Mismatch

| Operation | Source | Destination |
|-----------|--------|--------------|
| **Sync** | KOVA_ROOT or HOME (e.g. ~/) | target:/mnt/hive/projects/workspace/ |
| **Broadcast** | Project must be under ~/hive-vault | Maps to /mnt/hive on workers |

**Gap:** Sync uses `root` (HOME) directly. Broadcast requires `~/hive-vault`. So you need either:

- **Option A:** `~/hive-vault/projects/workspace` symlink to `~/` (or mirror), and `~/hive-vault/projects/rogue-repo` → `~/rogue-repo`.
- **Option B:** Set `KOVA_ROOT=~/hive-vault` and sync from there; then workspace lives under hive-vault.

---

## 5. USER_STORY Alignment

### USER_STORY_RECURSIVE_ACADEMY (Epics 7–10)

- Cursor prompts, trace/explain, DDI fix loop, Recursive Academy.
- **Hive mesh:** Not directly referenced. C2/broadcast is orthogonal.

### USER_STORY_PERFECT_RUST (Epics E1–E6)

- Elicitation, router, confirm-before-generate.
- **Hive mesh:** Not directly referenced. Broadcast is for parallel execution.

### PLAN_GAP_ANALYSIS_SIMULATION

- Sim 5: c2 — `kova c2 nodes`, `kova c2 run f20`, etc. All marked ✓.
- **Recommendation:** None. C2 is implemented.

---

## 6. Best-Ability Checklist

| Use case | Command | Status |
|----------|---------|--------|
| List workers | `kova c2 nodes` | ✓ |
| Inspect resources | `kova c2 inspect` | ✓ |
| Placement recommendations | `kova c2 inspect --recommend` | ✓ |
| Sync to one worker | `kova c2 sync lf` | ✓ (requires KOVA_ROOT or HOME) |
| Sync workspace to all | `for n in lf gd bt st; do kova c2 sync $n; done` | Manual |
| Full pipeline broadcast | `kova c2 run f20 --project ~/hive-vault/... --broadcast` | ✓ if hive-vault set up |
| Heavy build on best worker | `kova c2 inspect --recommend` → copy-paste ssh command | ✓ |
| Parallel build (all workers) | `sshallp "cd /mnt/hive/projects/workspace && cargo build --release"` | ✓ (manual) |
| SSH CA (no host key churn) | `kova c2 ssh-ca init` then `sign`/`setup` | ✓ |

---

## 7. Gaps to Address

### 7.1 Hive-vault setup

- **Gap:** Broadcast requires project under ~/hive-vault; sync uses HOME.
- **Fix:** Document or script: `ln -s ~ ~/hive-vault/projects/workspace` (or equivalent) so broadcast paths match sync.

### 7.2 Sync to all workers

- **Gap:** `kova c2 sync <target>` syncs to one target. No `--all` flag.
- **Fix:** Add `kova c2 sync --all` (or `sync` with no target = all) that loops over default_nodes().

### 7.3 KOVA_ROOT vs KOVA_PROJECT_PLACEMENT

- **Gap:** Sync uses KOVA_ROOT; workspace may be at ~/. Need clear convention.
- **Fix:** Document: KOVA_ROOT = workspace root; sync from there; hive-vault mirrors that structure for broadcast.

### 7.4 Node daemon (kova node)

- **Gap:** Phase 1 stub only. No Cap'n Proto listener yet.
- **Fix:** Per plan — defer; Phase 1 is schema + stub.

---

## 8. Recommended Next Steps

1. **Verify hive-vault:** `ls ~/hive-vault/projects/workspace` — if missing, create symlink or mirror.
2. **Run inspect:** `kova c2 inspect --recommend` — see placement tips and copy-paste commands.
3. **Sync once:** `kova c2 sync lf` (or each worker) — ensure workers have /mnt/hive populated.
4. **Test broadcast:** `kova c2 run f20 --project ~/hive-vault/projects/workspace/rogue-repo --broadcast` (if project under hive-vault).
5. **Consider sync --all:** Add convenience for syncing to all workers in one command.

---

## 9. One-Line Summary

**Hive mesh:** c2-core + workers (lf, gd, bt, st) + hive-vault → /mnt/hive. Use `c2 inspect --recommend`, `c2 sync`, `c2 run f20 --broadcast` once hive-vault is set up so broadcast paths match sync.
