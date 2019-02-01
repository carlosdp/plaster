//! # Plaster Framework - API Documentation
//!
//! Plaster is a framework for web-client apps created with
//! a modern Rust-to-Wasm compilation feature.
//! This framework was highly inspired by
//! [Elm](http://elm-lang.org/) and [React](https://reactjs.org/).
//! Forked originally from [Yew](https://github.com/DenisKolodin/yew).
//!
//! Minimal example:
//!
//! ```rust
//! #[macro_use]
//! extern crate plaster;
//! use plaster::prelude::*;
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
//! #[wasm_bindgen(start)]
//! fn main() {
//!     App::<Model>::new().mount_to_body();
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

#[macro_use]
extern crate log;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

#[macro_use]
pub mod macros;
// todo: figure out what to do with this
// pub mod agent;
pub mod app;
pub mod callback;
pub mod components;
pub mod html;
pub mod prelude;
pub mod scheduler;
pub mod virtual_dom;

use std::cell::RefCell;
use std::rc::Rc;

type Shared<T> = Rc<RefCell<T>>;

struct Hidden;
