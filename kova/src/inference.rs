// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Local LLM inference. Kalosm + GGUF. Streams tokens.
//! f76=chat_stream

use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc};
use std::thread;

use kalosm::language::{ChatModelExt, Parse};

#[cfg(feature = "inference")]
use once_cell::sync::Lazy;
#[cfg(feature = "inference")]
use std::num::NonZeroUsize;
#[cfg(feature = "inference")]
use std::sync::Mutex;

#[cfg(feature = "inference")]
type CachedModel = Arc<kalosm::language::Llama>;

#[cfg(feature = "inference")]
static MODEL_CACHE: Lazy<Mutex<lru::LruCache<PathBuf, CachedModel>>> = Lazy::new(|| {
    let cap = NonZeroUsize::new(crate::config::model_cache_size()).unwrap_or(NonZeroUsize::MIN);
    Mutex::new(lru::LruCache::new(cap))
});

/// Get or load model from cache. Returns Arc<Llama> for inference.
#[cfg(feature = "inference")]
pub(crate) async fn get_or_load_model(model_path: &Path) -> anyhow::Result<CachedModel> {
    use kalosm::language::*;

    let path_buf = model_path.to_path_buf();

    {
        let mut cache = MODEL_CACHE.lock().unwrap();
        if let Some(cached) = cache.get(&path_buf) {
            return Ok(Arc::clone(cached));
        }
    }

    let source = LlamaSource::new(FileSource::Local(model_path.to_path_buf()));
    let model = Llama::builder()
        .with_source(source)
        .build()
        .await
        .map_err(|e| anyhow::anyhow!("model load: {}", e))?;

    let model = Arc::new(model);
    {
        let mut cache = MODEL_CACHE.lock().unwrap();
        cache.put(path_buf, Arc::clone(&model));
    }
    Ok(model)
}

/// f76=chat_stream. Spawn inference in a thread. Returns receiver for streamed tokens.
/// Uses Arc<str> for zero-copy handoff; receiver can use &* without clone.
/// When sender is dropped, inference is done.
pub fn f76(
    model_path: &Path,
    system_prompt: &str,
    _history: &[(String, String)],
    user_input: &str,
) -> mpsc::Receiver<Arc<str>> {
    let (tx, rx) = mpsc::channel();
    let path = model_path.to_path_buf();
    let system = system_prompt.to_string();
    let input = user_input.to_string();

    thread::spawn(move || {
        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(e) => {
                let _ = tx.send(Arc::from(format!("Error: tokio runtime: {}", e)));
                return;
            }
        };
        rt.block_on(async {
            let _ = run_inference(&path, &system, &input, tx).await;
        });
    });

    rx
}

async fn run_inference(
    model_path: &Path,
    system_prompt: &str,
    user_input: &str,
    tx: mpsc::Sender<Arc<str>>,
) -> anyhow::Result<()> {
    use kalosm::language::StreamExt;

    let model = get_or_load_model(model_path).await?;

    let mut chat = model.chat().with_system_prompt(system_prompt);
    let mut response = chat(user_input);

    while let Some(token) = response.next().await {
        let _ = tx.send(Arc::from(token.to_string()));
    }

    Ok(())
}

/// f80_code_gen_structured. Experimental. Run inference with JSON schema, return rust_block directly.
/// Returns Ok(rust_block) on success, Err on failure. Use when code_gen_structured config is true.
#[cfg(feature = "inference")]
pub async fn f80_code_gen_structured(
    model_path: &Path,
    system_prompt: &str,
    user_input: &str,
) -> anyhow::Result<String> {
    use std::sync::Arc;

    let model = get_or_load_model(model_path).await?;

    const STRUCTURED_PROMPT: &str = "Generate Rust code only. Reply with valid JSON: {\"rust_block\": \"your code here\"}. Escape newlines as \\n and quotes as \\\".";
    let full_system = format!("{}\n\n{}", system_prompt, STRUCTURED_PROMPT);

    let task = model
        .task(&full_system)
        .with_constraints(Arc::new(CodeGenOutput::new_parser()));

    let stream = task.run(user_input);
    let output: CodeGenOutput = stream.await.map_err(|e| anyhow::anyhow!("structured code gen: {}", e))?;

    Ok(output.rust_block)
}

/// Structured output for code gen. Experimental.
#[cfg(feature = "inference")]
#[derive(Clone, Debug, kalosm::language::Parse, kalosm::language::Schema)]
struct CodeGenOutput {
    rust_block: String,
}

/// f80=chat_complete. Run inference, return full response. No streaming.
pub async fn f80(
    model_path: &Path,
    system_prompt: &str,
    user_input: &str,
) -> anyhow::Result<String> {
    use kalosm::language::StreamExt;

    let model = get_or_load_model(model_path).await?;

    let mut chat = model.chat().with_system_prompt(system_prompt);
    let mut response = chat(user_input);

    let mut out = String::new();
    while let Some(token) = response.next().await {
        out.push_str(&token.to_string());
    }
    Ok(out)
}
