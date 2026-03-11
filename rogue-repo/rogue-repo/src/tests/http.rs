// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! f51 = HTTP tests: real server, real requests

use std::time::Instant;

use axum::{routing::get, Router};
use tokio::net::TcpListener;

use crate::pwa;
use crate::routes;
use crate::tests::t24;

pub async fn f51() -> Vec<t24> {
    let mut out = Vec::new();

    let listener = match TcpListener::bind("127.0.0.1:0").await {
        Ok(l) => l,
        Err(e) => {
            return vec![t24 {
                name: "http_bind".into(),
                passed: false,
                duration_ms: 0,
                message: Some(format!("bind: {}", e)),
            }];
        }
    };
    let addr = listener.local_addr().unwrap();
    let p0 = routes::t0 { s0: None };
    let app = Router::new()
        .route("/", get(routes::f4))
        .route("/login", get(pwa::f102))
        .route("/register", get(pwa::f103))
        .route("/manifest.json", get(pwa::f92))
        .route("/sw.js", get(pwa::f93))
        .route("/assets/*path", get(pwa::f91))
        .route("/apps/rogue-runner", get(pwa::f94))
        .route("/apps/rogue-runner-wasm", get(pwa::f95))
        .route("/apps/null-terminal", get(pwa::f117))
        .route("/health", get(health))
        .route("/buy-bucks", axum::routing::post(routes::f87))
        .route("/provision-app", axum::routing::post(routes::f88))
        .route("/add-device", axum::routing::post(routes::f89))
        .with_state(p0);

    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let base = format!("http://{}", addr);
    let client = reqwest::Client::new();

    out.push(get_index_200(&client, &base).await);
    out.push(get_health_200(&client, &base).await);
    out.push(get_index_contains_economy(&client, &base).await);
    out.push(get_manifest_200(&client, &base).await);
    out.push(get_manifest_valid_json(&client, &base).await);
    out.push(get_sw_200(&client, &base).await);
    out.push(get_icon_200(&client, &base).await);
    out.push(get_app_image_200(&client, &base).await);
    out.push(get_rogue_runner_200(&client, &base).await);
    out.push(get_rogue_runner_wasm_200(&client, &base).await);
    out.push(get_not_found_404(&client, &base).await);
    out.push(get_health_json_ok(&client, &base).await);
    out.push(get_asset_404(&client, &base).await);
    out.push(post_buy_bucks_200(&client, &base).await);
    out.push(post_provision_app_200(&client, &base).await);
    out.push(post_add_device_200(&client, &base).await);
    out.push(post_invalid_json_returns_422(&client, &base).await);
    out.push(post_buy_bucks_missing_user_id_422(&client, &base).await);

    out
}

async fn health() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({"ok": true}))
}

async fn get_index_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/", base)).send().await;
    let ok = match r {
        Ok(res) => {
            res.status() == 200
                && res
                    .text()
                    .await
                    .map(|t| t.contains("Rogue Repo"))
                    .unwrap_or(false)
        }
        Err(_) => false,
    };
    t24 {
        name: "get_index_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET / 200 + Rogue Repo".into())
        },
    }
}

async fn get_health_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/health", base)).send().await;
    let ok = match r {
        Ok(res) => res.status() == 200,
        Err(_) => false,
    };
    t24 {
        name: "get_health_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET /health 200".into())
        },
    }
}

async fn post_buy_bucks_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let body = serde_json::json!({
        "user_id": "00000000-0000-0000-0000-000000000001",
        "pan_encrypted": []
    });
    let r = client
        .post(format!("{}/buy-bucks", base))
        .json(&body)
        .send()
        .await;
    let ok = match r {
        Ok(res) => {
            let status = res.status() == 200;
            let text = res.text().await.unwrap_or_default();
            status && (text.contains("\"ok\":true") || text.contains("\"ok\": true"))
        }
        Err(_) => false,
    };
    t24 {
        name: "post_buy_bucks_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("POST /buy-bucks 200 + ok:true".into())
        },
    }
}

async fn post_provision_app_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let body = serde_json::json!({
        "user_id": "00000000-0000-0000-0000-000000000001",
        "game_id": "game-1"
    });
    let r = client
        .post(format!("{}/provision-app", base))
        .json(&body)
        .send()
        .await;
    let ok = match r {
        Ok(res) => {
            let status = res.status() == 200;
            let text = res.text().await.unwrap_or_default();
            status && (text.contains("\"ok\":true") || text.contains("\"ok\": true"))
        }
        Err(_) => false,
    };
    t24 {
        name: "post_provision_app_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("POST /provision-app 200 + ok:true".into())
        },
    }
}

async fn post_add_device_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let body = serde_json::json!({
        "user_id": "00000000-0000-0000-0000-000000000001",
        "hardware_fingerprint": "fp-abc123"
    });
    let r = client
        .post(format!("{}/add-device", base))
        .json(&body)
        .send()
        .await;
    let ok = match r {
        Ok(res) => {
            let status = res.status() == 200;
            let text = res.text().await.unwrap_or_default();
            status && (text.contains("\"ok\":true") || text.contains("\"ok\": true"))
        }
        Err(_) => false,
    };
    t24 {
        name: "post_add_device_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("POST /add-device 200 + ok:true".into())
        },
    }
}

async fn get_manifest_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/manifest.json", base)).send().await;
    let ok = match r {
        Ok(res) => {
            let status = res.status() == 200;
            let ct = res
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");
            status && (ct.contains("manifest") || ct.contains("json"))
        }
        Err(_) => false,
    };
    t24 {
        name: "get_manifest_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET /manifest.json 200 + manifest content-type".into())
        },
    }
}

async fn get_manifest_valid_json(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/manifest.json", base)).send().await;
    let ok = match r {
        Ok(res) => {
            let text = res.text().await.unwrap_or_default();
            let parsed: Result<serde_json::Value, _> = serde_json::from_str(&text);
            let has_name = parsed
                .as_ref()
                .ok()
                .and_then(|v| v.get("name"))
                .map(|v| v.is_string())
                .unwrap_or(false);
            let has_icons = parsed
                .as_ref()
                .ok()
                .and_then(|v| v.get("icons"))
                .map(|v| v.is_array())
                .unwrap_or(false);
            parsed.is_ok() && has_name && has_icons
        }
        Err(_) => false,
    };
    t24 {
        name: "get_manifest_valid_json".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("manifest must be valid JSON with name and icons".into())
        },
    }
}

async fn get_not_found_404(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client
        .get(format!("{}/nonexistent-path-xyz", base))
        .send()
        .await;
    let ok = match r {
        Ok(res) => res.status().as_u16() == 404,
        Err(_) => false,
    };
    t24 {
        name: "get_not_found_404".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("unknown path must return 404".into())
        },
    }
}

async fn get_health_json_ok(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/health", base)).send().await;
    let ok = match r {
        Ok(res) => {
            let text = res.text().await.unwrap_or_default();
            let parsed: Result<serde_json::Value, _> = serde_json::from_str(&text);
            let ok_field = parsed
                .as_ref()
                .ok()
                .and_then(|v| v.get("ok"))
                .and_then(|v| v.as_bool());
            parsed.is_ok() && ok_field == Some(true)
        }
        Err(_) => false,
    };
    t24 {
        name: "get_health_json_ok".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("health must return JSON with ok:true".into())
        },
    }
}

async fn get_asset_404(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client
        .get(format!("{}/assets/nonexistent-file.xyz", base))
        .send()
        .await;
    let ok = match r {
        Ok(res) => res.status().as_u16() == 404,
        Err(_) => false,
    };
    t24 {
        name: "get_asset_404".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("nonexistent asset must return 404".into())
        },
    }
}

async fn get_sw_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/sw.js", base)).send().await;
    let ok = match r {
        Ok(res) => res.status() == 200,
        Err(_) => false,
    };
    t24 {
        name: "get_sw_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET /sw.js 200".into())
        },
    }
}

async fn get_icon_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client
        .get(format!("{}/assets/icon-192.svg", base))
        .send()
        .await;
    let ok = match r {
        Ok(res) => res.status() == 200,
        Err(_) => false,
    };
    t24 {
        name: "get_icon_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET /assets/icon-192.svg 200".into())
        },
    }
}

async fn get_app_image_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client
        .get(format!("{}/assets/apps/rogue-runner.png", base))
        .send()
        .await;
    let ok = match r {
        Ok(res) => {
            let status = res.status() == 200;
            let ct = res
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");
            status && (ct.contains("image") || ct.contains("png"))
        }
        Err(_) => false,
    };
    t24 {
        name: "get_app_image_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET /assets/apps/rogue-runner.png 200 + image content-type".into())
        },
    }
}

async fn get_rogue_runner_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client
        .get(format!("{}/apps/rogue-runner", base))
        .send()
        .await;
    let ok = match r {
        Ok(res) => {
            let status = res.status() == 200;
            let ct: String = res
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .into();
            let body = res.text().await.unwrap_or_default();
            status
                && (ct.contains("text/html") || ct.contains("html"))
                && body.contains("Rogue Runner")
        }
        Err(_) => false,
    };
    t24 {
        name: "get_rogue_runner_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET /apps/rogue-runner 200 + HTML content-type".into())
        },
    }
}

async fn get_rogue_runner_wasm_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client
        .get(format!("{}/apps/rogue-runner-wasm", base))
        .send()
        .await;
    let ok = match r {
        Ok(res) => {
            let status = res.status() == 200;
            let ct = res
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_string();
            let body = res.text().await.unwrap_or_default();
            status && (ct.contains("text/html") || ct.contains("html")) && body.contains("glcanvas")
        }
        Err(_) => false,
    };
    t24 {
        name: "get_rogue_runner_wasm_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET /apps/rogue-runner-wasm 200 + HTML".into())
        },
    }
}

async fn get_index_contains_economy(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/", base)).send().await;
    let ok = match r {
        Ok(res) => {
            let t = res.text().await.unwrap_or_default();
            t.contains("100 Rogue Bucks") && t.contains("420")
        }
        Err(_) => false,
    };
    t24 {
        name: "get_index_contains_economy".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET / must show economy table".into())
        },
    }
}

async fn post_invalid_json_returns_422(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client
        .post(format!("{}/buy-bucks", base))
        .header("Content-Type", "application/json")
        .body("{ invalid json }")
        .send()
        .await;
    let ok = match r {
        Ok(res) => {
            let s = res.status().as_u16();
            s == 400 || s == 422
        }
        Err(_) => false,
    };
    t24 {
        name: "post_invalid_json_returns_4xx".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("invalid JSON must return 400 or 422".into())
        },
    }
}

async fn post_buy_bucks_missing_user_id_422(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let body = serde_json::json!({ "pan_encrypted": [] });
    let r = client
        .post(format!("{}/buy-bucks", base))
        .json(&body)
        .send()
        .await;
    let ok = match r {
        Ok(res) => res.status().as_u16() == 422,
        Err(_) => false,
    };
    t24 {
        name: "post_buy_bucks_missing_user_id_422".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("missing user_id must return 422".into())
        },
    }
}
