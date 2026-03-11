# Wowasticker UI/UX Analysis

## Overview

Luka's Sticker Chart is a mobile-first behavioral tracking app: parents/educators dictate observations, AI parses sentiment, and stickers are assigned to schedule blocks. Target: one-handed use during school pickup, in-the-moment capture.

---

## Strengths

### 1. **Thumb-zone layout**
- Primary action (Dictate) is bottom-anchored with `flex-shrink: 0` — reachable without stretching
- `padding_bottom: env(safe-area-inset-bottom, 20px)` respects notches/home indicators
- Scrollable block list above keeps content in reach

### 2. **Clear hierarchy**
- Title → Goal → Blocks → Action. Linear, predictable
- Schedule cards use weight (600) and color (#666) to separate label from value
- Sticker states (○ ● ●●) are simple and scannable

### 3. **Selection feedback**
- Selected card: `#e3f2fd` background, `2px solid #007AFF` border — visible but not loud
- Transparent border when unselected avoids layout shift

### 4. **Status communication**
- Status text updates through flow: "Recording...", "Transcribing...", "Parsing...", "Done" / "Error"
- User knows where they are in a 10s+ async flow

### 5. **Disabled state during processing**
- Button `disabled: processing()` prevents double-tap and accidental re-recording

---

## Gaps & Risks

### 1. **No loading / progress for 10s recording**
- "Recording 10 seconds..." is static. No countdown or progress bar
- **Risk:** User may think it’s stuck, tap again, or put phone away
- **Suggestion:** Countdown (10…9…8…) or progress bar; consider haptic at start/end

### 2. **Selection affordance is weak**
- Cards look tappable but there’s no explicit "tap to select" hint
- New users may not realize they must select a block before dictating
- **Suggestion:** First-run hint: "Tap a block, then dictate" or subtle selection tutorial

### 3. **Goal is static**
- "Goal: 15 Stickers" is hardcoded; no per-student or per-day context
- **Suggestion:** Pull from `Student.goal_stickers` or make configurable

### 4. **Error recovery is minimal**
- `status.set(format!("Error: {}", e))` — error text only, no retry
- **Risk:** User has to understand the error and try again manually
- **Suggestion:** "Retry" button on error; optionally log/surface common causes (e.g. mic permission)

### 5. **Empty state is placeholder-only**
- When DB isn’t ready, `DEFAULT_BLOCKS` renders gray boxes with no interactivity
- **Risk:** Looks like real data; user may try to dictate into a non-selected block
- **Suggestion:** Skeleton or "Loading schedule..." instead of fake blocks

### 6. **No confirmation of what was saved**
- "Done. Sticker updated." doesn’t show which block or what score
- **Suggestion:** "Math: ●● (great job!)" — block name + score + snippet of note

### 7. **Accessibility**
- No `aria-label` or `role` on button/cards
- Sticker symbols (○ ● ●●) may need `aria-label` or text alternative for screen readers
- **Suggestion:** Add `aria-label="Dictate observation for selected block"` and semantic labels for sticker states

### 8. **Touch target size**
- Button padding (20px) is good
- Card tap area is full card — good
- Ensure minimum 44×44pt touch targets for all interactive elements

### 9. **No undo**
- Mis-dictation or wrong block → no way to revert
- **Suggestion:** Undo snackbar after "Done" or edit flow for last entry

### 10. **Transcription opacity**
- User never sees what was transcribed or how it was interpreted
- **Risk:** Trust issues if score feels wrong
- **Suggestion:** Expandable "What we heard" / "Why this score" after save (optional, for power users)

---

## Summary Table

| Area           | Status | Priority |
|----------------|--------|----------|
| Thumb-zone     | ✅     | —        |
| Selection UX   | ⚠️     | Medium   |
| Recording feedback | ❌  | High     |
| Error recovery| ❌     | High     |
| Goal display   | ⚠️     | Low      |
| Empty/loading  | ⚠️     | Medium   |
| Confirmation   | ⚠️     | Medium   |
| Accessibility  | ❌     | High     |
| Undo           | ❌     | Medium   |

---

## Quick Wins

1. **Recording countdown** — biggest UX win for a 10s wait
2. **Retry on error** — one extra button, much better recovery
3. **Richer "Done" message** — block name + score in status
4. **First-tap hint** — "Tap a block, then dictate" when no block selected
