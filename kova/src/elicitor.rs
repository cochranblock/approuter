// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Elicitor module. Format questions, parse short replies, build restatements.
//! Used by GUI and serve for clarification flow.

/// Parsed user reply to a clarification question.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElicitorReply {
    /// User picked choice by index: (a) → 0, (b) → 1, etc.
    Choice(usize),
    /// User typed freeform text (e.g. "compute.rs").
    Freeform(String),
    /// User confirmed (y/yes) or declined (n/no).
    Confirm(bool),
    /// User cancelled: cancel, n, no, stop.
    Cancel,
}

/// Format a question with optional choices as "(a) X (b) Y (c) Z".
pub fn format_question(question: &str, choices: Option<&[String]>) -> String {
    let q = question.trim();
    let Some(ch) = choices else {
        return q.to_string();
    };
    if ch.is_empty() {
        return q.to_string();
    }
    let letters = [b'a', b'b', b'c', b'd', b'e'];
    let parts: Vec<String> = ch
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let letter = letters.get(i).map(|&c| c as char).unwrap_or('?');
            format!("({}) {}", letter, s)
        })
        .collect();
    format!("{} {}", q, parts.join(" "))
}

/// Parse user input into ElicitorReply.
/// - "y", "yes" → Confirm(true)
/// - "n", "no", "cancel", "stop" → Cancel (or Confirm(false) for n/no when not in clarification)
/// - "a", "1" when num_choices given → Choice(0)
/// - "b", "2" → Choice(1), etc.
/// - Exact match of a choice string → Choice(index)
/// - Otherwise → Freeform(trimmed)
pub fn parse_reply(input: &str, num_choices: Option<usize>) -> ElicitorReply {
    let s = input.trim().to_lowercase();
    if s.is_empty() {
        return ElicitorReply::Cancel;
    }
    // Cancel keywords
    if matches!(s.as_str(), "cancel" | "stop" | "abort") {
        return ElicitorReply::Cancel;
    }
    // Confirm
    if matches!(s.as_str(), "y" | "yes") {
        return ElicitorReply::Confirm(true);
    }
    if matches!(s.as_str(), "n" | "no") {
        return ElicitorReply::Confirm(false);
    }
    // Choice by letter: a=0, b=1, c=2, d=3, e=4
    if s.len() == 1 {
        let c = s.chars().next().unwrap();
        if let Some(idx) = (b'a'..=b'e').position(|x| x as char == c) {
            if num_choices.is_none_or(|n| idx < n) {
                return ElicitorReply::Choice(idx);
            }
        }
        if let Some(idx) = (b'1'..=b'5').position(|x| (x - b'0') as char == c) {
            if num_choices.is_none_or(|n| idx < n) {
                return ElicitorReply::Choice(idx);
            }
        }
    }
    // Choice by number "1", "2", etc.
    if let Ok(n) = s.parse::<usize>() {
        if n >= 1 {
            let idx = n - 1;
            if num_choices.is_none_or(|max| idx < max) {
                return ElicitorReply::Choice(idx);
            }
        }
    }
    ElicitorReply::Freeform(input.trim().to_string())
}

/// Build restatement: "I'll add X to Y. Proceed? (y/n)"
pub fn build_restatement(action: &str, target: &str) -> String {
    let a = action.trim();
    let t = target.trim();
    if t.is_empty() {
        format!("{}. Proceed? (y/n)", a)
    } else {
        format!("I'll {} in {}. Proceed? (y/n)", a, t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_question_no_choices() {
        assert_eq!(format_question("Which file?", None), "Which file?");
    }

    #[test]
    fn format_question_with_choices() {
        let ch = ["compute.rs".into(), "plan.rs".into(), "lib.rs".into()];
        assert_eq!(
            format_question("Which file?", Some(&ch)),
            "Which file? (a) compute.rs (b) plan.rs (c) lib.rs"
        );
    }

    #[test]
    fn parse_reply_confirm() {
        assert_eq!(parse_reply("y", None), ElicitorReply::Confirm(true));
        assert_eq!(parse_reply("yes", None), ElicitorReply::Confirm(true));
        assert_eq!(parse_reply("Y", None), ElicitorReply::Confirm(true));
    }

    #[test]
    fn parse_reply_cancel() {
        assert_eq!(parse_reply("n", None), ElicitorReply::Confirm(false));
        assert_eq!(parse_reply("no", None), ElicitorReply::Confirm(false));
        assert_eq!(parse_reply("cancel", None), ElicitorReply::Cancel);
        assert_eq!(parse_reply("stop", None), ElicitorReply::Cancel);
    }

    #[test]
    fn parse_reply_choice_letter() {
        assert_eq!(parse_reply("a", Some(3)), ElicitorReply::Choice(0));
        assert_eq!(parse_reply("b", Some(3)), ElicitorReply::Choice(1));
        assert_eq!(parse_reply("c", Some(3)), ElicitorReply::Choice(2));
    }

    #[test]
    fn parse_reply_choice_number() {
        assert_eq!(parse_reply("1", Some(3)), ElicitorReply::Choice(0));
        assert_eq!(parse_reply("2", Some(3)), ElicitorReply::Choice(1));
    }

    #[test]
    fn parse_reply_freeform() {
        assert_eq!(
            parse_reply("compute.rs", Some(3)),
            ElicitorReply::Freeform("compute.rs".into())
        );
    }

    #[test]
    fn build_restatement_basic() {
        assert_eq!(
            build_restatement("add retry helper", "compute.rs"),
            "I'll add retry helper in compute.rs. Proceed? (y/n)"
        );
    }
}
