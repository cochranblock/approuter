// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Router model. Intent classification. f79=classify.

use std::path::Path;
use std::sync::{mpsc, Arc};
use std::thread;

use kalosm::language::{ChatModelExt, Parse};

const CLASSIFY_PROMPT: &str = r#"Classify the user's message as exactly one of: code_gen, refactor, explain, fix, run, custom, needs_clarification.
If needs_clarification, include a "question" field with a short clarification question and optionally "choices" (array of 2-5 options).
Reply with valid JSON only: {"classification": "..."} or {"classification": "needs_clarification", "question": "Which file?", "choices": ["compute.rs", "plan.rs"]}"#;

/// Structured output for grammar-constrained Router. Maps to RouterResult.
#[derive(Clone, Debug, kalosm::language::Parse, kalosm::language::Schema)]
struct RouterOutput {
    classification: String,
    question: Option<String>,
    choices: Option<Vec<String>>,
}

/// Router classification result.
#[derive(Debug, Clone, PartialEq, Eq)]
/// t94=RouterResult. Router classification output.
pub enum RouterResult {
    CodeGen,
    Refactor,
    Explain,
    Fix,
    Run,
    Custom,
    NeedsClarification {
        question: Option<String>,
        choices: Option<Vec<String>>,
    },
    Error(String),
}

impl RouterResult {
    /// Suggested question when NeedsClarification. Canned fallback if model didn't provide one.
    /// Uses elicitor::format_question when choices are available.
    pub fn clarification_question(&self, original_msg: &str) -> String {
        match self {
            RouterResult::NeedsClarification { question, choices } => {
                let q = question
                    .as_deref()
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| {
                        let lower = original_msg.to_lowercase();
                        if lower.contains("fix") || lower.contains("bug") {
                            "Which file?"
                        } else if lower.contains("add") || lower.contains("implement") {
                            "Which file should I add this to?"
                        } else {
                            "Could you clarify what you need?"
                        }
                    });
                if let Some(ref ch) = choices {
                    if !ch.is_empty() {
                        return crate::elicitor::format_question(q, Some(ch));
                    }
                }
                // Canned choices for fix/add when model didn't provide any
                let lower = original_msg.to_lowercase();
                if lower.contains("fix") || lower.contains("bug") {
                    let canned = vec![
                        "compute.rs".into(),
                        "plan.rs".into(),
                        "lib.rs".into(),
                        "other".into(),
                    ];
                    return crate::elicitor::format_question(q, Some(&canned));
                }
                if lower.contains("add") || lower.contains("implement") {
                    let canned = vec!["lib.rs".into(), "main.rs".into(), "other".into()];
                    return crate::elicitor::format_question(q, Some(&canned));
                }
                q.to_string()
            }
            _ => "Could you clarify what you need?".into(),
        }
    }

    /// Choices when NeedsClarification. None if freeform only.
    pub fn clarification_choices(&self) -> Option<&[String]> {
        match self {
            RouterResult::NeedsClarification { choices, .. } => choices.as_deref(),
            _ => None,
        }
    }

    /// Use coder model for response (code_gen, refactor, explain, fix, custom).
    pub fn use_coder(&self) -> bool {
        matches!(
            self,
            RouterResult::CodeGen
                | RouterResult::Refactor
                | RouterResult::Explain
                | RouterResult::Fix
                | RouterResult::Custom
        )
    }

    fn parse(s: &str) -> Self {
        let lower = s.to_lowercase().trim().to_string();
        let t = lower.as_str();
        if t.contains("needs_clarification") || t.contains("clarification") {
            let question = t
                .split('|')
                .nth(1)
                .map(|q| q.trim().to_string())
                .filter(|q| !q.is_empty());
            return RouterResult::NeedsClarification {
                question,
                choices: None,
            };
        }
        if t.contains("code_gen") || t.contains("code gen") {
            return RouterResult::CodeGen;
        }
        if t.contains("refactor") {
            return RouterResult::Refactor;
        }
        if t.contains("explain") {
            return RouterResult::Explain;
        }
        if t.contains("fix") {
            return RouterResult::Fix;
        }
        if t.contains("run") {
            return RouterResult::Run;
        }
        if t.contains("custom") {
            return RouterResult::Custom;
        }
        RouterResult::CodeGen
    }
}

/// f79=classify. Spawn router inference. Returns receiver for single RouterResult.
pub fn f79(model_path: &Path, user_input: &str) -> mpsc::Receiver<RouterResult> {
    let (tx, rx) = mpsc::channel();
    let path = model_path.to_path_buf();
    let input = user_input.to_string();

    thread::spawn(move || {
        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(e) => {
                let _ = tx.send(RouterResult::Error(format!("tokio: {}", e)));
                return;
            }
        };
        rt.block_on(async {
            let result = run_classify(&path, &input).await;
            let _ = tx.send(result);
        });
    });

    rx
}

async fn run_classify(model_path: &Path, user_input: &str) -> RouterResult {
    use kalosm::language::StreamExt;

    let model = match crate::inference::get_or_load_model(model_path).await {
        Ok(m) => m,
        Err(e) => return RouterResult::Error(format!("model load: {}", e)),
    };

    if crate::config::router_structured() {
        if let Ok(result) = run_classify_structured(&model, user_input).await {
            return result;
        }
    }

    let mut chat = model.chat().with_system_prompt(CLASSIFY_PROMPT);
    let mut response = chat(user_input);

    let mut out = String::new();
    while let Some(token) = response.next().await {
        out.push_str(&token.to_string());
    }

    RouterResult::parse(&out)
}

async fn run_classify_structured(
    model: &kalosm::language::Llama,
    user_input: &str,
) -> Result<RouterResult, ()> {
    let task = model
        .task(CLASSIFY_PROMPT)
        .with_constraints(Arc::new(RouterOutput::new_parser()));

    let stream = task.run(user_input);
    let output = stream.await.map_err(|_| ())?;

    Ok(router_output_to_result(&output))
}

fn router_output_to_result(out: &RouterOutput) -> RouterResult {
    let c = out.classification.to_lowercase().trim().to_string();
    let t = c.as_str();
    if t.contains("needs_clarification") || t.contains("clarification") {
        return RouterResult::NeedsClarification {
            question: out.question.clone(),
            choices: out.choices.clone(),
        };
    }
    if t.contains("code_gen") || t.contains("code gen") {
        return RouterResult::CodeGen;
    }
    if t.contains("refactor") {
        return RouterResult::Refactor;
    }
    if t.contains("explain") {
        return RouterResult::Explain;
    }
    if t.contains("fix") {
        return RouterResult::Fix;
    }
    if t.contains("run") {
        return RouterResult::Run;
    }
    if t.contains("custom") {
        return RouterResult::Custom;
    }
    RouterResult::CodeGen
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use crate::kova_test;

    kova_test!(f79, parse_needs_clarification_with_question, {
        let r = RouterResult::parse("needs_clarification|Which file?");
        assert_matches!(r, RouterResult::NeedsClarification { question: Some(q), .. } => {
            assert_eq!(q, "which file?");
        });
    });

    kova_test!(f79, parse_needs_clarification_without_question, {
        let r = RouterResult::parse("needs_clarification");
        assert_matches!(r, RouterResult::NeedsClarification { question: None, choices: None });
    });

    kova_test!(f79, clarification_question_canned, {
        let r = RouterResult::NeedsClarification {
            question: None,
            choices: None,
        };
        assert!(r.clarification_question("fix the bug").contains("file"));
        assert!(r.clarification_question("add a retry").contains("file"));
        assert_eq!(
            r.clarification_question("something vague"),
            "Could you clarify what you need?"
        );
    });
}
