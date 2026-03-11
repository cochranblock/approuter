# Kova Compression Map

Tokenization for traceability. Source: workspace tokenization rules.

## Functions (fN)

| Token | Human name | Module | Tests |
|-------|------------|--------|-------|
| f14 | plan | plan | integration: full_pipeline_*, tunnel_update_*, setup_roguerepo_* |
| f15 | execute | compute | integration: full_pipeline_* |
| f20 | full_pipeline | intent | intent::f62_full_pipeline, integration |
| f21 | tunnel_update | intent | intent::f62_tunnel_update, integration |
| f22 | setup_roguerepo | intent | intent::f62_setup_roguerepo, integration |
| f25 | load_backlog | backlog | backlog::f25_roundtrip, integration::load_backlog_roundtrip |
| f62 | parse_intent | intent | intent::f62_* |
| f73 | store_message | context | (via storage roundtrip) |
| f74 | load_messages | context | (via storage roundtrip) |
| f76 | stream | inference | — |
| f78 | model_path_for_role | config | config::model_role_default_filename |
| f79 | classify | router | router::parse_*, router::clarification_* |
| f80 | chat_complete | inference | — |
| f60 | triple_sims_run | exopack | — |
| f61 | run_cargo_test_n | exopack | — |
| f81 | run_code_gen_pipeline | pipeline | pipeline::cargo_check_clippy_test_* |
| f82 | load_project_context | context_loader | context_loader::f82_* |
| f86 | recent_changes_snapshot | recent_changes | recent_changes::f86_* |
| f87 | format_recent_changes | recent_changes | recent_changes::f87_* |
| f91 | cargo_check | pipeline | — |
| f92 | cargo_clippy | pipeline | — |
| f93 | cargo_test | pipeline | — |
| f90 | kova_test | bin/kova-test | clippy, TRIPLE SIMS (f61), release build, bootstrap smoke, serve smoke (exopack http_client) |
| f83 | target_file_hint | context_loader | context_loader::f83_target_hint |
| f84 | format_diff | output | output::f84_diff |
| f85 | resolve_target_path | output | output::f85_resolve_target |
| f39 | open | storage | storage::store_* |
| f40 | put_compressed | storage | storage::store_put_get_compressed_roundtrip |
| f41 | get_compressed | storage | storage::store_* |
| f42 | put_raw | storage | storage::store_put_raw_get_raw |
| f43 | get_raw | storage | storage::store_put_raw_get_raw |
| f77 | model_install | model | — |
| f94 | default_project | config | — |
| f95 | discover_projects | config | — |
| f96 | projects_root | config | (internal) |
| f97 | home | config | — |
| f98 | kova_dir | config | — |
| f99 | prompts_dir | config | — |
| f100 | sled_path | config | — |
| f101 | models_dir | config | — |
| f102 | inference_model_path | config | — |
| f103 | backlog_path | config | — |
| f104 | workspace_root | config | — |
| f105 | load_build_preset | config | — |
| f106 | all_build_presets | config | — |
| f107 | infer_preset_name | config | — |
| f108 | bind_addr | config | — |
| f109 | bootstrap | config | — |
| f110 | load_prompt | config | — |
| f111 | load_cursor_prompts | cursor_prompts | cursor_prompts::* |
| f112 | format_context | context_loader | context_loader::* |
| f113 | gui_run | gui | — |
| f114 | serve_run | serve | — |
| f115 | explain_trace | academy | — |
| f116 | fix_and_retry | pipeline | — |
| f117 | run_cargo | pipeline/compilation | — |
| f118 | categorize | pipeline/error_kind | — |
| f119 | kova_c2_run | c2 | CLI orchestration (kova c2 run) |
| f120 | kova_c2_broadcast | c2 | SSH broadcast to workers |
| f121 | autopilot_run | autopilot | Type prompt into Cursor composer |

## Types (tN)

| Token | Human name |
|-------|------------|
| t0 | Intent |
| t1 | IntentKind |
| t3 | Plan |
| t5 | ActionKind |
| t6 | Executor |
| t12 | Store |
| t86 | RecentChange |
| t88 | BuildPreset |
| t89 | ModelRole |
| t90 | ProjectContext |
| t91 | Message |
| t92 | AppState |
| t93 | LastTrace |
| t94 | RouterResult |
| t95 | ErrorKind |

## Struct fields (sN) — plan t3

| Token | Human name | Type |
|-------|------------|------|
| s4 | project | PathBuf |
| s5 | approuter_dir | Option<PathBuf> |
| s7 | project_hint | Option<String> |

## Test traceability

Each test has `/// fN=human_name` doc comment linking to the function under test.
Run `rg '/// f[0-9]+=' kova/src` to list coverage.

### Macros

- **kova_test!(fN, test_name, { body })** — Adds `#[test]` and `/// fN=traceability` doc. Use for unit tests.
- **assert_matches!** (dev-dep) — Pattern assertion; cleaner than `match { X => ..., _ => panic!() }`.

### recent_changes (f86/f87)

**Stay on task.** Polls project for files modified in last N minutes. Tokenized output for LLM context.

- **CLI:** `kova recent [--project DIR] [--minutes 30]`
- **HTTP:** `GET /context/recent?minutes=30` (serve mode)
- **Pipeline:** f82_with_recent injects recent changes into code-gen prompt (30 min default)

### exopack (critical)

**kova-test** binary (f90) depends on exopack for TRIPLE SIMS via f61. Main binary uses plain `cargo test` (f93); no exopack in main.

### Robustness

- **f62_table_driven** — Single table of (input, expected); add rows to extend coverage.
- **f62_precedence_*** — Explicit precedence tests (full vs test, cloudflare vs cache).
- **extract_rust_block** — Edge cases: multiple blocks, empty, non-rust, unclosed.
- **context_loader** — src vs root, nonexistent file, underscores in filename.
- **Integration** — tunnel/setup use temp dirs (no ~/approuter). full_pipeline_on_rogue_repo is `#[ignore]`.
