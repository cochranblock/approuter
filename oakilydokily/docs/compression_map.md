# OakilyDokily Compression Map

Tokenization for traceability. Aligns with kova fN/tN convention.

## Functions (fN)

| Token | Human name | Module |
|-------|------------|--------|
| f1 | router | web/router |
| f30 | run_tests | tests |
| f70 | head | web/pages, web/waiver |
| f71 | html_escape | web/waiver |
| f72 | client_ip | web/waiver |
| f73 | hero_bg | web/pages |
| f74 | get_waiver | web/waiver |
| f75 | post_waiver | web/waiver |
| f76 | verify_turnstile | web/waiver |
| f77 | validate_waiver_input | waiver |
| f78 | send_waiver_confirmation | web/email |
| f78 | user_get | d1_auth |
| f79 | html_escape (email) | web/email |
| f79 | user_create | d1_auth |
| f80 | ga4_script | web/head |
| f80_from_env | d1_client_from_env | d1_auth |
| f81 | html_escape_attr | web/head |
| f81 | shard_for_email | d1_auth |
| f82 | google_redirect | web/auth |
| f82 | d1_query | d1_auth |
| f83 | google_callback | web/auth |
| f84 | logout | web/auth |
| f85 | oauth_state | web/auth |
| f86 | session_cookie_value | web/auth |
| f87 | parse_session | web/auth |
| f88 | get_session | web/auth |
| f89 | nav_auth_link | web/head |
| f90 | nav | web/head |
| f91 | apple_redirect | web/auth |
| f92 | apple_callback | web/auth |
| f93 | decode_apple_id_token | web/auth |
| f94 | is_https | web/auth |
| f95 | safe_redirect_path | web/auth |
| f95 | sitemap | web/pages |
| f96 | book_call_link | web/head |
| f96 | redirect_after_login | web/auth |
| f98 | facebook_redirect | web/auth |
| f99 | facebook_callback | web/auth |
| f100 | login_page | web/auth |
| f101 | login_post | web/auth |
| f102 | signup_page | web/auth |
| f103 | signup_post | web/auth |
| f104 | home | web/pages |
| f105 | about | web/pages |
| f106 | contact | web/pages |
| f107 | health | web/pages |
| f108 | mural | web/pages |
| f109 | serve | web/assets |
| f110 | waiver_confirmed | web/waiver |
| f112 | terms_hash | waiver |
| f113 | init_pool | waiver |
| f114 | insert | waiver |
| f115 | terms_text | waiver |
| f116 | user_create | waiver |
| f117 | user_get | waiver |
| f118 | hash_email | web/auth |
| f119 | window_conf | mural-wasm/main |
| f120 | landscape_load | mural-wasm/landscape |
| f121 | SpriteSheet::load | mural-wasm/sprites |
| f122 | TextureAtlas::from_sheet | mural-wasm/sprites |
| f123 | SpriteSheet::cell_rect | mural-wasm/sprites |
| f124 | TextureAtlas::frame | mural-wasm/sprites |
| f125 | TextureAtlas::kiss_frame | mural-wasm/sprites |
| f126 | TextureAtlas::texture | mural-wasm/sprites |
| f127 | mural_set_scroll_x | mural-wasm/bridge |
| f128 | mural_set_scroll_y | mural-wasm/bridge |
| f129 | mural_set_mouse | mural-wasm/bridge |
| f130 | get_scroll_x | mural-wasm/bridge |
| f131 | get_scroll_y | mural-wasm/bridge |
| f132 | get_mouse_pos | mural-wasm/bridge |
| f133 | Pet::new | mural-wasm/pet |
| f134 | Pet::trigger_kiss | mural-wasm/pet |
| f135 | Pet::enter_exodus | mural-wasm/pet |
| f136 | Pet::update | mural-wasm/pet |
| f137 | Pet::draw | mural-wasm/pet |
| f138 | SceneState::update | mural-wasm/scenes |
| f139 | SceneState::draw | mural-wasm/scenes |

## Types (tN)

| Token | Human name |
|-------|------------|
| t0 | AppState |
| t78 | D1AuthClient |
| t82 | GoogleUser |
| t83 | OAuth callback query |
| t119 | Species | mural-wasm/sprites |
| t120 | Animation | mural-wasm/sprites |
| t121 | SpriteSheet | mural-wasm/sprites |
| t122 | TextureAtlas | mural-wasm/sprites |
| t123 | Pet | mural-wasm/pet |
| t124 | PetState | mural-wasm/pet |
| t125 | HeartParticle | mural-wasm/pet |
| t126 | SceneState | mural-wasm/scenes |

## Struct fields (sN)

| Token | Type | Field |
|-------|------|-------|
| s73 | SceneState | cozy_nook_visible |
| s74 | SceneState | cozy_nook_x |
| s75 | SceneState | tubing_visible |
| s76 | SceneState | tubing_y |
| s77 | SceneState | tubing_vel |
| s78 | SceneState | doggy_door_triggered |

## Test traceability

- `/// f30=run_tests` — tests/mod.rs
- `//! f70=oakilydokily_test` — bin/oakilydokily-test.rs (TRIPLE SIMS via exopack)

Run `rg '/// f[0-9]+=' oakilydokily/src oakilydokily/mural-wasm/src` to list coverage.
