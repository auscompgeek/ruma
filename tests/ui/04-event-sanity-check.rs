// rustc overflows when compiling this see:
// https://github.com/rust-lang/rust/issues/55779
extern crate serde;

use ruma_events::{RawEventContent, StateEventContent};
use ruma_events_macros::Event;

/// State event.
#[derive(Clone, Debug, Event)]
pub struct StateEvent<C: StateEventContent>
where
    C::Raw: RawEventContent,
{
    pub content: C,
    pub state_key: String,
    pub prev_content: Option<C>,
}

fn main() {}
