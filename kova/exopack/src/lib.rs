// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! exopack — testing augmentation: screenshot, video, interfaces, API mocks, triple sims.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

#[cfg(feature = "interface")]
pub mod interface;

#[cfg(feature = "mock")]
pub mod mock;

pub mod video;

#[cfg(feature = "screenshot")]
pub mod screenshot;

#[cfg(feature = "triple_sims")]
pub mod triple_sims;

#[cfg(feature = "devtools")]
pub mod devtools;

#[cfg(feature = "demo")]
pub mod demo;

#[cfg(feature = "baked_demo")]
pub mod baked_demo;
