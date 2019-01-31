//! # Yew Framework - API Documentation
//!
//! Yew is a framework for web-client apps created with
//! a modern Rust-to-Wasm compilation feature.
//! This framework was highly inspired by
//! [Elm](http://elm-lang.org/) and [React](https://reactjs.org/).
//!
//! Minimal example:
//!
//! ```rust
//! #[macro_use]
//! extern crate yew;
//! use yew::prelude::*;
//!
//! struct Model {
//!     value: i64,
//! }
//!
//! enum Msg {
//!     DoIt,
//! }
//!
//! impl Component for Model {
//!     type Message = Msg;
//!     type Properties = ();
//!     fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
//!         Self {
//!             value: 0,
//!         }
//!     }
//!
//!     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//!         match msg {
//!             Msg::DoIt => self.value = self.value + 1
//!         }
//!         true
//!     }
//! }
//!
//! impl Renderable<Model> for Model {
//!     fn view(&self) -> Html<Self> {
//!         html! {
//!             <div>
//!                <button onclick=|_| Msg::DoIt,>{ "+1" }</button>
//!                 <p>{ self.value }</p>
//!             </div>
//!         }
//!     }
//! }
//!
//! fn main() {
//!     yew::initialize();
//!     App::<Model>::new().mount_to_body();
//!     yew::run_loop();
//! }
//! ```
//!

#![deny(
    missing_docs,
    bare_trait_objects,
    anonymous_parameters,
    elided_lifetimes_in_paths
)]
#![recursion_limit = "512"]
#![feature(try_from)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate http;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate anymap;
extern crate bincode;
extern crate js_sys;
#[cfg(feature = "msgpack")]
extern crate rmp_serde;
#[cfg(feature = "cbor")]
extern crate serde_cbor;
extern crate serde_json;
#[cfg(feature = "yaml")]
extern crate serde_yaml;
extern crate slab;
#[cfg(feature = "toml")]
extern crate toml;
extern crate wasm_bindgen;
extern crate web_sys;

#[macro_use]
pub mod macros;
// todo: figure out what to do with this
// pub mod agent;
pub mod app;
pub mod callback;
pub mod components;
pub mod format;
pub mod html;
pub mod prelude;
pub mod scheduler;
pub mod virtual_dom;

use std::cell::RefCell;
use std::rc::Rc;

type Shared<T> = Rc<RefCell<T>>;

struct Hidden;
