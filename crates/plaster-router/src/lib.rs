use js_sys::Function;
use plaster::callback::Callback;
use route_recognizer::{Params, Router as RecRouter};
use serde_derive::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, CustomEvent, CustomEventInit};

use log::trace;
pub use plaster_router_macro::Routes;

pub struct Router<T> {
    routes: Vec<fn(Params) -> T>,
    index_router: RecRouter<usize>,
    current_path: Arc<Mutex<String>>,
    listener: Closure<dyn FnMut(CustomEvent)>,
    callback: Callback<()>,
}

impl<T> Router<T> {
    pub fn new(callback: Callback<()>) -> Router<T> {
        let win = window().expect("need a window context");
        let path = if cfg!(not(feature = "mobile")) {
            win.location().pathname().unwrap_or("/".to_string())
        } else {
            "/".to_string()
        };
        trace!("initial route: {}", &path);
        let current_path = Arc::new(Mutex::new(path));
        let current_path_c = current_path.clone();
        let callback_c = callback.clone();

        let listener_callback = Closure::wrap(Box::new(move |e: CustomEvent| {
            let ev: RouteEvent = e
                .detail()
                .into_serde()
                .expect("could not deserialize route event");
            trace!("route change: {}", &ev.route);
            *current_path_c.lock().unwrap() = ev.route;
            callback_c.emit(());
        }) as Box<dyn FnMut(_)>);

        let listener_function: &Function = listener_callback.as_ref().unchecked_ref();

        win.add_event_listener_with_callback("plasterroutechange", listener_function)
            .expect("could not attach global event listener");

        if cfg!(not(feature = "mobile")) {
            win.add_event_listener_with_callback("popstate", listener_function)
                .expect("could not attach popstate event listener");
        }

        Router {
            routes: Vec::new(),
            index_router: RecRouter::new(),
            current_path: current_path,
            listener: listener_callback,
            callback: callback,
        }
    }

    pub fn add_route(&mut self, route: &str, closure: fn(Params) -> T) {
        trace!("added route: {}", route);
        let index = self.routes.len();
        self.routes.push(closure);
        self.index_router.add(route, index);
    }

    pub fn navigate(&mut self, path: &str) {
        *self.current_path.lock().unwrap() = path.to_string();
        if cfg!(not(feature = "mobile")) {
            self.push_state();
        }
        self.callback.emit(());
    }

    pub fn resolve(&self) -> Option<T> {
        let route_match = self
            .index_router
            .recognize(&self.current_path.lock().unwrap())
            .ok();
        route_match.map(|m| self.routes.get(m.handler.clone()).unwrap()(m.params))
    }

    pub fn current_route(&self) -> String {
        self.current_path.lock().unwrap().clone()
    }

    pub fn set_route(&self, path: &str) {
        *self.current_path.lock().unwrap() = path.to_string();
    }

    fn push_state(&self) {
        match window().expect("need a window context").history() {
            Ok(history) => {
                history
                    .push_state_with_url(
                        &JsValue::NULL,
                        "",
                        Some(&self.current_path.lock().unwrap()),
                    )
                    .expect("could not pushState");
            }
            Err(_) => (),
        }
    }
}

impl<T> Drop for Router<T> {
    fn drop(&mut self) {
        window()
            .expect("need window context")
            .remove_event_listener_with_callback(
                "plasterroutechange",
                self.listener.as_ref().unchecked_ref(),
            )
            .expect("could not remove event listener");
    }
}

pub trait Routes<T> {
    fn router(callback: Callback<()>) -> Router<T>;
}

pub fn route_to(path: &str) {
    let win = window().expect("need window context");

    if cfg!(not(feature = "mobile")) {
        win.history()
            .expect("history API unavailable")
            .push_state_with_url(&JsValue::NULL, "", Some(path))
            .expect("could not pushState");
    }

    let mut init = CustomEventInit::new();
    init.detail(
        &JsValue::from_serde(&RouteEvent {
            route: path.to_owned(),
        })
        .unwrap(),
    );
    let event = CustomEvent::new_with_event_init_dict("plasterroutechange", &init)
        .expect("could not create CustomEvent");
    win.dispatch_event(&event)
        .expect("could not dispatch route change");
}

#[derive(Serialize, Deserialize)]
struct RouteEvent {
    route: String,
}
