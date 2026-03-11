// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use crate::error::t18;

pub async fn f13(ipify_url: &str) -> Result<String, t18> {
    let res = reqwest::get(ipify_url)
        .await
        .map_err(|e| t18::E4(e.to_string()))?;
    let text = res.text().await.map_err(|e| t18::E4(e.to_string()))?;
    Ok(text.trim().to_string())
}
