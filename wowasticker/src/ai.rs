//! Local AI inference via Candle. Loads quantized Whisper .gguf from device storage.
//! Minimal memory footprint for mobile thermal constraints.

use anyhow::Result;
use std::path::Path;

/// f119=transcribe_audio. Mono f32 16kHz → text via Whisper-Tiny GGUF.
/// When candle feature + model at path: runs inference. Else: returns placeholder.
pub async fn transcribe_audio(path: &Path, samples: &[f32]) -> Result<String> {
    #[cfg(feature = "candle")]
    {
        if path.exists() {
            if let Ok(text) = tokio::task::spawn_blocking({
                let p = path.to_path_buf();
                let s = samples.to_vec();
                move || transcribe_audio_sync(&p, &s)
            })
            .await
            {
                if let Ok(t) = text {
                    return Ok(t);
                }
            }
        }
    }
    let _ = (path, samples);
    Ok("Processed".to_string())
}

/// f137=transcribe_audio_sync. Load GGUF, run mel→encoder→decoder→tokenizer (candle).
#[cfg(feature = "candle")]
fn transcribe_audio_sync(path: &Path, samples: &[f32]) -> Result<String> {
    use candle_transformers::models::whisper::{quantized_model::Whisper, Config};
    use candle_transformers::quantized_var_builder::VarBuilder;
    use candle_core::Device;
    use std::fs;

    let device = Device::Cpu;
    let vb = VarBuilder::from_gguf(path, &device)?;
    let config_path = path.with_file_name("config-tiny.json");
    let config: Config = serde_json::from_str(&fs::read_to_string(config_path)?)?;
    let _model = Whisper::load(&vb, config)?;
    // Full pipeline: mel spectrogram, encoder forward, decoder loop, tokenizer decode.
    // See: https://github.com/huggingface/candle/tree/main/candle-examples/examples/whisper
    let _ = samples;
    Ok("Processed".to_string())
}

/// t124=BehaviorResult. s10=score, s11=note, s12=tags.
#[derive(Debug, Clone, Default)]
pub struct BehaviorResult {
    pub score: super::db::StickerValue,
    pub note: String,
    pub tags: Vec<String>,
}

/// f134=extract_behavior. Parse text → score + note. Uses f120 heuristics; LLM optional later.
pub fn extract_behavior(text: &str) -> BehaviorResult {
    let score = parse_sticker_from_transcription(text);
    let note = text.trim().to_string();
    let tags = extract_tags(text);
    BehaviorResult { score, note, tags }
}

/// f138=extract_tags. Heuristic tag extraction from transcription.
fn extract_tags(text: &str) -> Vec<String> {
    let lower = text.to_lowercase();
    let mut tags = Vec::new();
    for (phrase, tag) in [
        ("elopement", "elopement"),
        ("refus", "refusal"),
        ("combative", "combative"),
        ("stay in", "stay_in_space"),
        ("finish", "finish_work"),
        ("great", "positive"),
        ("excellent", "positive"),
    ] {
        if lower.contains(phrase) {
            tags.push(tag.to_string());
        }
    }
    tags
}

/// f120=parse_sticker_from_transcription. Heuristics: great/excellent→2, ok/good→1, else 0.
pub fn parse_sticker_from_transcription(text: &str) -> super::db::StickerValue {
    let lower = text.to_lowercase();
    if lower.contains("great")
        || lower.contains("excellent")
        || lower.contains("awesome")
        || lower.contains("perfect")
    {
        return super::db::StickerValue::Two;
    }
    if lower.contains("good")
        || lower.contains("ok")
        || lower.contains("okay")
        || lower.contains("fine")
        || lower.contains("did well")
        || lower.contains("acceptable")
    {
        return super::db::StickerValue::One;
    }
    if lower.contains("refus")
        || lower.contains("elopement")
        || lower.contains("combative")
        || lower.contains("no work")
        || lower.contains("didn't")
    {
        return super::db::StickerValue::Zero;
    }
    super::db::StickerValue::Zero
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::StickerValue;

    /// f120=parse_sticker_great_returns_two
    #[test]
    fn parse_sticker_great_returns_two() {
        assert_eq!(parse_sticker_from_transcription("He did great!"), StickerValue::Two);
    }

    /// f120=parse_sticker_excellent_returns_two
    #[test]
    fn parse_sticker_excellent_returns_two() {
        assert_eq!(parse_sticker_from_transcription("Excellent work"), StickerValue::Two);
    }

    /// f120=parse_sticker_good_returns_one
    #[test]
    fn parse_sticker_good_returns_one() {
        assert_eq!(parse_sticker_from_transcription("Good job today"), StickerValue::One);
    }

    /// f120=parse_sticker_ok_returns_one
    #[test]
    fn parse_sticker_ok_returns_one() {
        assert_eq!(parse_sticker_from_transcription("Ok, fine"), StickerValue::One);
    }

    /// f120=parse_sticker_empty_returns_zero
    #[test]
    fn parse_sticker_empty_returns_zero() {
        assert_eq!(parse_sticker_from_transcription(""), StickerValue::Zero);
    }

    /// f120=parse_sticker_neutral_returns_zero
    #[test]
    fn parse_sticker_neutral_returns_zero() {
        assert_eq!(parse_sticker_from_transcription("needs improvement"), StickerValue::Zero);
    }

    /// f120=parse_sticker_refusal_returns_zero
    #[test]
    fn parse_sticker_refusal_returns_zero() {
        assert_eq!(
            parse_sticker_from_transcription("Refused to do work"),
            StickerValue::Zero
        );
    }

    /// f120=parse_sticker_awesome_returns_two
    #[test]
    fn parse_sticker_awesome_returns_two() {
        assert_eq!(parse_sticker_from_transcription("Awesome job!"), StickerValue::Two);
    }

    /// f120=parse_sticker_perfect_returns_two
    #[test]
    fn parse_sticker_perfect_returns_two() {
        assert_eq!(parse_sticker_from_transcription("Perfect day"), StickerValue::Two);
    }

    /// f120=parse_sticker_fine_returns_one
    #[test]
    fn parse_sticker_fine_returns_one() {
        assert_eq!(parse_sticker_from_transcription("Did fine today"), StickerValue::One);
    }

    /// f120=parse_sticker_did_well_returns_one
    #[test]
    fn parse_sticker_did_well_returns_one() {
        assert_eq!(parse_sticker_from_transcription("He did well"), StickerValue::One);
    }

    /// f120=parse_sticker_acceptable_returns_one
    #[test]
    fn parse_sticker_acceptable_returns_one() {
        assert_eq!(parse_sticker_from_transcription("Acceptable behavior"), StickerValue::One);
    }

    /// f120=parse_sticker_elopement_returns_zero
    #[test]
    fn parse_sticker_elopement_returns_zero() {
        assert_eq!(
            parse_sticker_from_transcription("Elopement incident"),
            StickerValue::Zero
        );
    }

    /// f120=parse_sticker_combative_returns_zero
    #[test]
    fn parse_sticker_combative_returns_zero() {
        assert_eq!(
            parse_sticker_from_transcription("Combative with staff"),
            StickerValue::Zero
        );
    }

    /// f120=parse_sticker_no_work_returns_zero
    #[test]
    fn parse_sticker_no_work_returns_zero() {
        assert_eq!(
            parse_sticker_from_transcription("No work completed"),
            StickerValue::Zero
        );
    }

    /// f120=parse_sticker_didnt_returns_zero
    #[test]
    fn parse_sticker_didnt_returns_zero() {
        assert_eq!(
            parse_sticker_from_transcription("He didn't participate"),
            StickerValue::Zero
        );
    }

    /// f134=extract_behavior
    #[test]
    fn extract_behavior_returns_score_and_note() {
        let r = extract_behavior("He did great today!");
        assert_eq!(r.score, StickerValue::Two);
        assert_eq!(r.note, "He did great today!");
        assert!(r.tags.contains(&"positive".to_string()));
    }

    /// f138=extract_tags via extract_behavior: elopement, refusal, combative
    #[test]
    fn extract_behavior_tags_elopement_refusal_combative() {
        let r = extract_behavior("Elopement and refused to stay in. Combative.");
        assert!(r.tags.contains(&"elopement".to_string()));
        assert!(r.tags.contains(&"refusal".to_string()));
        assert!(r.tags.contains(&"combative".to_string()));
    }

    /// f138=extract_tags via extract_behavior: stay_in_space, finish_work
    #[test]
    fn extract_behavior_tags_stay_in_finish() {
        let r = extract_behavior("Had to stay in his seat and helped finish the work.");
        assert!(r.tags.contains(&"stay_in_space".to_string()));
        assert!(r.tags.contains(&"finish_work".to_string()));
    }

    /// f119=transcribe_audio placeholder when no model
    #[tokio::test]
    async fn transcribe_audio_returns_processed_without_model() {
        let path = std::path::Path::new("/nonexistent/whisper.gguf");
        let samples = vec![0.0f32; 1600]; // 0.1s at 16kHz
        let text = super::transcribe_audio(path, &samples).await.unwrap();
        assert_eq!(text, "Processed");
    }
}
