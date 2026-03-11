// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Categorize cargo check stderr for agentic fix loop. f118=categorize, t95=ErrorKind.

/// t95=ErrorKind. Error category for Mechanic (Fixer) model context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Syntax,
    BorrowChecker,
    Lifetime,
    Type,
    Other,
}

/// f118=categorize. Categorize stderr from cargo check. Feeds specific context to the Mechanic.
pub fn categorize(stderr: &str) -> ErrorKind {
    let lower = stderr.to_lowercase();
    if lower.contains("expected one of") || lower.contains("unexpected end of")
        || lower.contains("expected `;`") || lower.contains("expected `,`")
        || lower.contains("expected `)`") || lower.contains("expected `{`")
        || lower.contains("missing semicolon") || lower.contains("unclosed delimiter")
        || lower.contains("expected expression")
    {
        return ErrorKind::Syntax;
    }
    if lower.contains("borrow") || lower.contains("cannot borrow")
        || lower.contains("borrowed value") || lower.contains("move")
        || lower.contains("use of moved value")
    {
        return ErrorKind::BorrowChecker;
    }
    if lower.contains("lifetime") || lower.contains("outlives")
        || lower.contains("does not live long enough") || lower.contains("'static")
    {
        return ErrorKind::Lifetime;
    }
    if lower.contains("expected") && (lower.contains("type") || lower.contains("found"))
        || lower.contains("mismatched types") || lower.contains("cannot infer")
    {
        return ErrorKind::Type;
    }
    ErrorKind::Other
}

fn context_hint(kind: ErrorKind) -> &'static str {
    match kind {
        ErrorKind::Syntax => "Fix the syntax error (missing semicolon, delimiter, etc).",
        ErrorKind::BorrowChecker => "Fix the borrow checker error (ownership, mutability).",
        ErrorKind::Lifetime => "Fix the lifetime error (references, scope).",
        ErrorKind::Type => "Fix the type mismatch.",
        ErrorKind::Other => "Fix the compilation error.",
    }
}

/// Build error block with categorized context for the Mechanic.
pub fn error_block_with_context(stage: &str, stderr: &str) -> String {
    let kind = categorize(stderr);
    let hint = context_hint(kind);
    format!(
        "{} error (category: {:?}). {}\n\nStderr:\n```\n{}\n```",
        stage, kind, hint, stderr
    )
}
