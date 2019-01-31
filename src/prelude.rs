//! The Yew Prelude
//!
//! The purpose of this module is to alleviate imports of many common types:
//!
//! ```
//! # #![allow(unused_imports)]
//! use yew::prelude::*;
//! ```
pub use html::{
    ChangeData, Component, ComponentLink, Href, Html, InputData, Renderable, ShouldRender,
};

pub use app::App;

pub use callback::Callback;

pub use web_sys::{
    DragEvent, Event, FocusEvent, InputEvent, KeyEvent, KeyboardEvent, MouseEvent,
    MouseScrollEvent, PointerEvent,
};

// todo: figure out what to do with this
// pub use agent::{Bridge, Bridged, Threaded};

// /// Prelude module for creating worker.
// pub mod worker {
//     pub use agent::{
//         Agent, AgentLink, Bridge, Bridged, Context, Global, HandlerId, Job, Private, Public,
//         Transferable,
//     };
// }
