# Wowasticker Compression Map

Tokenization for traceability. Aligns with kova fN/tN convention.

## Functions (fN)

| Token | Human name | Module |
|-------|------------|--------|
| f119 | transcribe_audio | ai |
| f120 | parse_sticker_from_transcription | ai |
| f121 | db_open | db |
| f122 | db_init | db |
| f123 | ensure_default_schedule | db |
| f124 | list_blocks | db |
| f125 | get_sticker | db |
| f126 | set_sticker | db |
| f127 | get_sticker_today | db |
| f128 | set_sticker_today | db |
| f129 | capture_audio | audio |
| f130 | resample_to_16k | audio |
| f131 | wowasticker_test | bin/wowasticker-test |
| f132 | run_dictation_flow | ui |
| f133 | App | ui |
| f134 | extract_behavior | ai |
| f135 | set_sticker_today_with_note | db |
| f136 | set_sticker_with_note | db |
| f137 | transcribe_audio_sync | ai (candle) |
| f138 | extract_tags | ai |
| f139 | ScheduleCard | ui |

## Types (tN)

| Token | Human name |
|-------|------------|
| t119 | StickerValue |
| t120 | ScheduleBlock |
| t121 | StickerRecord |
| t122 | Student |
| t123 | Db |
| t124 | BehaviorResult |

## Struct fields (sN)

| Token | Type | Field |
|-------|------|-------|
| s0 | ScheduleBlock | id |
| s1 | ScheduleBlock | name |
| s2 | ScheduleBlock | sort_order |
| s3 | StickerRecord | block_id |
| s4 | StickerRecord | date |
| s5 | StickerRecord | value |
| s6 | Student | id |
| s7 | Student | name |
| s8 | Student | goal_stickers |
| s9 | StickerRecord | note |
| s10 | BehaviorResult | score |
| s11 | BehaviorResult | note |
| s12 | BehaviorResult | tags |

## Test traceability

- `/// f131=wowasticker_test` — bin/wowasticker-test.rs (TRIPLE SIMS via exopack f61_with_args)
- Run `rg '/// f[0-9]+=' wowasticker/src` to list coverage.
