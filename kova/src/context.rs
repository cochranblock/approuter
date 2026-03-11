// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Conversation persistence. conv:default:messages in sled.

use crate::storage;
use serde::{Deserialize, Serialize};

const KEY: &[u8] = b"conv:default:messages";
const CAP: usize = 20;

/// t91=Message. Chat message with role and content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// f73 = store_message. Append role+content, cap at CAP.
pub fn f73(store: &storage::t12, role: &str, content: &str) -> Result<(), storage::E0> {
    let mut msgs = f74(store).unwrap_or_default();
    msgs.push(Message {
        role: role.into(),
        content: content.into(),
    });
    if msgs.len() > CAP {
        msgs.drain(0..(msgs.len() - CAP));
    }
    store.f40(KEY, &msgs)
}

/// f74 = load_messages. Load conversation for chat.
pub fn f74(store: &storage::t12) -> Result<Vec<Message>, storage::E0> {
    Ok(store.f41(KEY)?.unwrap_or_default())
}
