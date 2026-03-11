# Folder Token Map (dN)

Tokenization for source directories. Reduces path length and standardizes layout across Rust binary projects.

## Convention

- **dN** = directory under `src/` or project root
- **Preserved:** `src`, `bin` (Rust standard; do not rename)
- **Flatten:** Prefer `src/dN` over `src/group/dN` — one level under src when possible

## Global tokens (shared across projects)

| Token | Human name | Use |
|-------|------------|-----|
| — | src | Preserved (Rust) |
| — | bin | Preserved (Rust binaries) |
| d0 | core | Core logic, no I/O |
| d1 | web | Web/HTTP layer |
| d2 | pipeline | Code gen pipeline |
| d3 | tests | Test helpers |
| d4 | detect | Detection modules |
| d5 | export | Export/output |
| d6 | ux | UX/UI helpers |
| d7 | config | Config (if dir) |
| d8 | auth | Auth module |
| d9 | crypto | Crypto module |
| d10 | db | Database module |
| d11 | dns | DNS module |

## Per-project mapping

### kova
- src/pipeline → src/d2
- (bin preserved)

### cochranblock
- src/core → src/d0
- src/core/auth → src/d8 (flatten)
- src/core/crypto → src/d9 (flatten)
- src/core/db → src/d10 (flatten)
- src/core/dns → src/d11 (flatten)
- src/web → src/d1
- src/tests → src/d3
- src/ux → src/d6

### oakilydokily
- src/web → src/d1
- src/tests → src/d3

### whyyoulying
- src/detect → src/d4
- src/export → src/d5

### wowasticker
- (flat: src + bin only)

### approuter
- (flat: src + bin)

## Migration

1. Add `#![allow(non_snake_case)]` for dN module names
2. Rename dirs, update `mod` declarations
3. Update `use` paths
4. Extend this map when adding new dirs
