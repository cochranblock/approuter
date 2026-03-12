// Unlicense — cochranblock.org
// Contributors: mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! approuter — reverse proxy + client for self-registration.
//! Use `approuter::{f116, RegisterConfig}` when client feature is enabled.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

pub mod setup;

#[cfg(feature = "client")]
pub use approuter_client::{f116, RegisterConfig};
