<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Rogue Repo Compression Map
# Token-Optimized Code Representation
# Sovereign ISO 8583 payment engine + Rogue Bucks economy

---

## L1: IDENTIFIER MAP

### Functions (f)
```
f0  = main
f1  = buy_bucks (POST /buy-bucks)
f2  = provision_app (POST /provision-app)
f3  = add_device (POST /add-device)
f4  = serve_index (GET /)
f5  = health (GET /health)
f94 = serve_rogue_runner (GET /apps/rogue-runner)
f95 = serve_rogue_runner_wasm (GET /apps/rogue-runner-wasm)
f97 = register (POST /register)
f98 = login (POST /login)
f99 = (reserved)
f100 = verify_email (GET /verify-email)
f101 = logout (POST /logout)
f102 = serve_login (GET /login)
f103 = serve_register (GET /register)
f10 = encrypt_pan
f11 = decrypt_pan
f12 = build_0200 (ISO 8583 MTI 0200 pack)
f13 = send_to_bank (TCP)
f14 = add_device_ledger (deduct 420, insert fingerprint)
f15 = provision_entitlement (42 bucks, insert entitlement)
f16 = credit_bucks (420 for entry buy-in)
f20 = init_db
f21 = run_migrations
f30 = run_tests
f31 = rogue_repo_test (binary, TRIPLE SIMS via exopack f60)
```

### Types/Structs (t)
```
t0  = AppState
t1  = Vault (AES-256-GCM)
t2  = PurchaseRequest (ISO 8583)
t3  = Iso8583Message
t4  = Ledger (repo-ledger)
t5  = BuyBucksRequest
t6  = ProvisionAppRequest
t7  = AddDeviceRequest
t8  = User
t9  = Device
t10 = Entitlement
t11 = Transaction
t12 = AppError (variants E0–E9)
t83 = BuyBucksReq
t84 = BuyBucksRes / generic ok response
t86 = AddDeviceReq
t97 = RegisterForm
t98 = LoginForm
t99 = AuthRes (ok, message, user_id)
```

### Parameters (p)
```
p0 = state (AppState)
p1 = req (Request)
p2 = body / payload
p3 = vault (Vault)
p4 = pan (encrypted bytes)
p5 = amount_cents
p6 = user_id
p7 = hardware_fingerprint
p8 = game_id
p9 = pool (PgPool)
```

### Struct Fields (s)
```
s0 = db_pool
s1 = vault
s2 = ledger
s3 = switch_config
s4 = encryption_key
s5 = rogue_bucks_balance
s6 = hardware_fingerprint
s7 = is_authorized
s8 = user_id
s9 = game_id
s10 = amount_cents
```

### Error Variants (e)
```
e0 = Unauthorized
e1 = InsufficientBucks
e2 = DeviceAlreadyAuthorized
e3 = VaultError
e4 = SwitchError
e5 = LedgerError
e6 = DbError
e7 = InvalidRequest
```

### Constants (c)
```
c0 = ENTRY_BUY_IN_CENTS (420 = $4.20)
c1 = ENTRY_BUY_IN_BUCKS (420)
c2 = ASSET_COST_BUCKS (42)
c3 = ADD_DEVICE_FEE_BUCKS (420)
c4 = BUCKS_PER_USD (100)
```

### Rogue Runner (rogue-runner binary + lib)
```
f95 = mulberry32 (lib PRNG)
f96 = generate_level (lib)
f105 = load_progress
f106 = save_progress
f107 = start_game
f108 = jump
f109 = game_over
f110 = level_complete
f111 = update
f112 = draw
f113 = loop (HTML rAF)
f114 = resize
f115 = rogue_runner_test (binary, TRIPLE SIMS via exopack f61)
t95 = Obstacle (lib)
t96 = LevelData (lib)
t88 = GameState
s88 = state (GameState)
s89 = level
s90 = score
s91 = player_y
s92 = vy
s93 = level_data
s94 = obstacle_idx
s95 = saved_level
c90 = MAX_LEVEL (1000)
c91 = GRAVITY
c92 = JUMP
c93 = PLAYER_H
c94 = PLAYER_W
c95 = GROUND
```

---

## Docs (d)

```
d1  = Sim1 (User Story Analysis)
d2  = Sim2 (Feature Gap Analysis)
d3  = Sim3 (UI/UX Analysis)
d4  = Implementation Summary
d5  = TRIPLE_SIMS_ROGUEREPO.md (PWA, app store)
d6  = TRIPLE_SIMS_ROGUE_REPO.md (tests, API)
d7  = TRIPLE_SIMS_ROGUEREPO_AUTH.md (f97–f103)
```

---

## PRESERVED (not compressed)

Rust std, tokio, axum, sqlx, aes_gcm, bitvec, ed25519_dalek, bytes, serde.
