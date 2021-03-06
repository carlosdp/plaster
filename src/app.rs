//! This module contains `App` sctruct which used to bootstrap
//! a component in an isolated scope.

use html::{Component, Renderable, Scope};
use web_sys::{window, Element};

/// An application instance.
pub struct App<COMP: Component> {
    /// `Scope` holder
    scope: Scope<COMP>,
}

impl<COMP> App<COMP>
where
    COMP: Component + Renderable<COMP>,
{
    /// Creates a new `App` with a component in a context.
    pub fn new() -> Self {
        let scope = Scope::new();
        App { scope }
    }

    /// Alias to `mount("body", ...)`.
    pub fn mount_to_body(self) -> Scope<COMP> {
        // Bootstrap the component for `Window` environment only (not for `Worker`)
        let element = window()
            .expect("context needs a window")
            .document()
            .expect("window needs a document")
            .query_selector("body")
            .expect("can't get body node for rendering")
            .expect("can't unwrap body node");
        self.mount(element, None)
    }

    /// Alias to `mount()` that allows using a selector
    pub fn mount_to_selector(self, selector: &str) -> Scope<COMP> {
        let element = window()
            .expect("context needs a window")
            .document()
            .expect("window needs a document")
            .query_selector(selector)
            .expect("can't get node for rendering")
            .expect("can't unwrap body node");
        self.mount(element, None)
    }

    /// Alias to `mount()` that allows passing in initial props
    pub fn mount_with_props(self, element: Element, props: COMP::Properties) -> Scope<COMP> {
        self.mount(element, Some(props))
    }

    /// The main entrypoint of a yew program. It works similar as `program`
    /// function in Elm. You should provide an initial model, `update` function
    /// which will update the state of the model and a `view` function which
    /// will render the model to a virtual DOM tree.
    pub fn mount(self, element: Element, props: Option<COMP::Properties>) -> Scope<COMP> {
        clear_element(&element);
        self.scope.mount_in_place(element, None, None, props)
    }
}

/// Removes anything from the given element.
fn clear_element(element: &Element) {
    while let Some(child) = element.last_child() {
        element.remove_child(&child).expect("can't remove a child");
    }
}
