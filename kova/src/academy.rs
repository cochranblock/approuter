// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Recursive Academy. Explain trace via model.
//! fN=explain_trace

use std::path::Path;

use crate::trace::LastTrace;

/// f115=explain_trace. Explain a pipeline trace in plain English. Uses Coder model + Cursor prompts + DDI reference.
pub async fn explain_trace(trace: &LastTrace, model_path: &Path) -> Result<String, String> {
    let cursor = crate::cursor_prompts::load_cursor_prompts(
        &crate::default_project(),
    );
    let ddi_note = "Fix loop loses effectiveness after 2–3 attempts (DDI). We cap retries to avoid worse output.";
    let system = if cursor.is_empty() {
        format!(
            "You are Recursive Academy. Explain this Kova execution trace. \
             What did the user want? What failed? Why? How would a user fix it? Be concise.\n\n{}",
            ddi_note
        )
    } else {
        format!(
            "You are Recursive Academy. Explain this Kova execution trace. \
             What did the user want? What failed? Why? How would a user fix it? Be concise. \
             Reference Cursor rules (tokenization, compression_map) when relevant.\n\n--- Cursor rules ---\n{}\n\n{}",
            cursor, ddi_note
        )
    };

    let user_msg = format!(
        "Intent: {}\nUser: {}\nStage: {}\nOutcome: {}\nRetries: {}\nStderr:\n```\n{}\n```\nChain: {}",
        trace.intent,
        trace.user_msg,
        trace.stage,
        trace.outcome,
        trace.retry_count,
        trace.stderr,
        trace.chain.join(" → ")
    );

    crate::inference::f80(model_path, &system, &user_msg)
        .await
        .map_err(|e| format!("Explain failed: {}", e))
}
