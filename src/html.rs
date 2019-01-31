//! The main module which contents aliases to necessary items
//! to create a template and implement `update` and `view` functions.
//! Also this module contains declaration of `Component` trait which used
//! to create own UI-components.

use callback::Callback;
use js_sys::Function;
use scheduler::{scheduler, Runnable};
use std::cell::RefCell;
use std::rc::Rc;
use virtual_dom::{Listener, VDiff, VNode};
use wasm_bindgen::prelude::*;
use web_sys::{Element, EventTarget, HtmlSelectElement, Node};
use Shared;

/// A handle to an event listener
pub struct EventListenerHandle {
    event_target: EventTarget,
    function: Function,
    type_: String,
}

impl EventListenerHandle {
    /// Create a new EventListenerHandle with the target Element, the converted Closure, and the
    /// event type (ie. "onclick").
    pub fn new(target: &EventTarget, function: &Function, type_: &str) -> EventListenerHandle {
        target
            .add_event_listener_with_callback(type_, function)
            .expect("could not add event listener to element");

        EventListenerHandle {
            event_target: target.clone(),
            function: function.clone(),
            type_: type_.to_string(),
        }
    }

    /// Remove the event listener from the target Element.
    pub fn remove(&self) {
        self.event_target
            .remove_event_listener_with_callback(&self.type_, &self.function)
            .expect("could not remove event listener");
    }
}

/// This type indicates that component should be rendered again.
pub type ShouldRender = bool;

/// An interface of a UI-component. Uses `self` as a model.
pub trait Component: Sized + 'static {
    /// Control message type which `update` loop get.
    type Message: 'static;
    /// Properties type of component implementation.
    /// It sould be serializable because it's sent to dynamicaly created
    /// component (layed under `VComp`) and must be restored for a component
    /// with unknown type.
    type Properties: Clone + PartialEq + Default;
    /// Initialization routine which could use a context.
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self;
    /// Called everytime when a messages of `Msg` type received. It also takes a
    /// reference to a context.
    fn update(&mut self, msg: Self::Message) -> ShouldRender;
    /// This method called when properties changes, and once when component created.
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        unimplemented!("you should implement `change` method for a component with properties")
    }
    /// Called for finalization on the final point of the component's lifetime.
    fn destroy(&mut self) {} // TODO Replace with `Drop`
}

/// Should be rendered relative to context and component environment.
pub trait Renderable<COMP: Component> {
    /// Called by rendering loop.
    fn view(&self) -> Html<COMP>;
}

/// Update message for a `Components` instance. Used by scope sender.
pub(crate) enum ComponentUpdate<COMP: Component> {
    /// Creating an instance of the component
    Create(ComponentLink<COMP>),
    /// Wraps messages for a component.
    Message(COMP::Message),
    /// Wraps properties for a component.
    Properties(COMP::Properties),
    /// Removes the component
    Destroy,
}

/// Link to component's scope for creating callbacks.
pub struct ComponentLink<COMP: Component> {
    scope: Scope<COMP>,
}

impl<COMP> ComponentLink<COMP>
where
    COMP: Component + Renderable<COMP>,
{
    /// Create link for a scope.
    fn connect(scope: &Scope<COMP>) -> Self {
        ComponentLink {
            scope: scope.clone(),
        }
    }

    /// This method sends messages back to the component's loop.
    pub fn send_back<F, IN>(&mut self, function: F) -> Callback<IN>
    where
        F: Fn(IN) -> COMP::Message + 'static,
    {
        let scope = self.scope.clone();
        let closure = move |input| {
            let output = function(input);
            scope.clone().send_message(output);
        };
        closure.into()
    }

    /// This method sends a message to this component immediately.
    pub fn send_self(&mut self, msg: COMP::Message) {
        self.scope.send_message(msg);
    }
}

/// A context which contains a bridge to send a messages to a loop.
/// Mostly services uses it.
pub struct Scope<COMP: Component> {
    shared_component: Shared<Option<ComponentRunnable<COMP>>>,
}

impl<COMP: Component> Clone for Scope<COMP> {
    fn clone(&self) -> Self {
        Scope {
            shared_component: self.shared_component.clone(),
        }
    }
}

impl<COMP> Scope<COMP>
where
    COMP: Component + Renderable<COMP>,
{
    /// Send the message and schedule an update.
    pub(crate) fn send(&mut self, update: ComponentUpdate<COMP>) {
        let envelope = ComponentEnvelope {
            shared_component: self.shared_component.clone(),
            message: Some(update),
        };
        let runnable: Box<dyn Runnable> = Box::new(envelope);
        scheduler().put_and_try_run(runnable);
    }

    /// Send message to a component.
    pub fn send_message(&mut self, message: COMP::Message) {
        let update = ComponentUpdate::Message(message);
        self.send(update);
    }
}

/// Holder for the element.
pub type NodeCell = Rc<RefCell<Option<Node>>>;

impl<COMP> Scope<COMP>
where
    COMP: Component + Renderable<COMP>,
{
    pub(crate) fn new() -> Self {
        let shared_component = Rc::new(RefCell::new(None));
        Scope { shared_component }
    }

    // TODO Consider to use &Node instead of Element as parent
    /// Mounts elements in place of previous node (ancestor).
    pub(crate) fn mount_in_place(
        self,
        element: Element,
        ancestor: Option<VNode<COMP>>,
        occupied: Option<NodeCell>,
        init_props: Option<COMP::Properties>,
    ) -> Scope<COMP> {
        let runnable = ComponentRunnable {
            env: self.clone(),
            component: None,
            last_frame: None,
            element,
            ancestor,
            occupied,
            init_props,
            destroyed: false,
        };
        let mut scope = self.clone();
        *scope.shared_component.borrow_mut() = Some(runnable);
        let link = ComponentLink::connect(&scope);
        scope.send(ComponentUpdate::Create(link));
        scope
    }
}

struct ComponentRunnable<COMP: Component> {
    env: Scope<COMP>,
    component: Option<COMP>,
    last_frame: Option<VNode<COMP>>,
    element: Element,
    ancestor: Option<VNode<COMP>>,
    occupied: Option<NodeCell>,
    init_props: Option<COMP::Properties>,
    destroyed: bool,
}

/// Wraps a component reference and a message to hide it under `Runnable` trait.
/// It's necessary to schedule a processing of a message.
struct ComponentEnvelope<COMP>
where
    COMP: Component,
{
    shared_component: Shared<Option<ComponentRunnable<COMP>>>,
    message: Option<ComponentUpdate<COMP>>,
}

impl<COMP> Runnable for ComponentEnvelope<COMP>
where
    COMP: Component + Renderable<COMP>,
{
    fn run(&mut self) {
        let mut component = self.shared_component.borrow_mut();
        let this = component.as_mut().expect("shared component not set");
        if this.destroyed {
            return;
        }
        let mut should_update = false;
        let upd = self
            .message
            .take()
            .expect("component's envelope called twice");
        // This loop pops one item, because the following
        // updates could try to borrow the same cell
        // Important! Don't use `while let` here, because it
        // won't free the lock.
        let env = this.env.clone();
        match upd {
            ComponentUpdate::Create(link) => {
                let props = this.init_props.take().unwrap_or_default();
                this.component = Some(COMP::create(props, link));
                // No messages at start
                let current_frame = this.component.as_ref().unwrap().view();
                this.last_frame = Some(current_frame);
                // First-time rendering the tree
                let node = this.last_frame.as_mut().unwrap().apply(
                    &this.element,
                    None,
                    this.ancestor.take(),
                    &env,
                );
                if let Some(ref mut cell) = this.occupied {
                    *cell.borrow_mut() = node;
                }
            }
            ComponentUpdate::Message(msg) => {
                should_update |= this
                    .component
                    .as_mut()
                    .expect("component was not created to process messages")
                    .update(msg);
            }
            ComponentUpdate::Properties(props) => {
                should_update |= this
                    .component
                    .as_mut()
                    .expect("component was not created to process properties")
                    .change(props);
            }
            ComponentUpdate::Destroy => {
                // TODO this.component.take() instead of destroyed
                this.component.as_mut().unwrap().destroy();
                this.destroyed = true;
            }
        }
        if should_update {
            let mut next_frame = this.component.as_ref().unwrap().view();
            // Re-rendering the tree
            let node = next_frame.apply(&this.element, None, this.last_frame.take(), &env);
            if let Some(ref mut cell) = this.occupied {
                *cell.borrow_mut() = node;
            }
            this.last_frame = Some(next_frame);
        }
    }
}

/// A type which expected as a result of `view` function implementation.
pub type Html<MSG> = VNode<MSG>;

macro_rules! impl_action {
    ($($action:ident($event:ident : $type:ident) -> $ret:ty => $convert:expr)*) => {$(
        /// An abstract implementation of a listener.
        pub mod $action {
            use web_sys::Element;
            #[allow(unused)]
            use wasm_bindgen::JsCast;
            use web_sys::$type;
            use super::*;

            /// A wrapper for a callback.
            /// Listener extracted from here when attached.
            pub struct Wrapper<F>(Option<F>);

            /// And event type which keeps the returned type.
            pub type EventTy = $ret;

            impl<F, MSG> From<F> for Wrapper<F>
            where
                MSG: 'static,
                F: Fn($ret) -> MSG + 'static,
            {
                fn from(handler: F) -> Self {
                    Wrapper(Some(handler))
                }
            }

            impl<T, COMP> Listener<COMP> for Wrapper<T>
            where
                T: Fn($ret) -> COMP::Message + 'static,
                COMP: Component + Renderable<COMP>,
            {
                fn kind(&self) -> &'static str {
                    stringify!($action)
                }

                fn attach(&mut self, element: &Element, mut activator: Scope<COMP>)
                    -> EventListenerHandle {
                    let handler = self.0.take().expect("tried to attach listener twice");
                    let this = element.clone();
                    let listener = Closure::wrap(Box::new(move |event: $type| {
                        debug!("Event handler: {}", stringify!($type));
                        event.stop_propagation();
                        let handy_event: $ret = $convert(&this, event);
                        let msg = handler(handy_event);
                        activator.send_message(msg);
                    }) as Box<dyn FnMut($type)>);
                    EventListenerHandle::new(element, listener.as_ref().unchecked_ref(), stringify!($action))
                }
            }
        }
    )*};
}

// Inspired by: http://package.elm-lang.org/packages/elm-lang/html/2.0.0/Html-Events
impl_action! {
    onclick(event: MouseEvent) -> MouseEvent => |_, event| { event }
    ondoubleclick(event: MouseEvent) -> MouseEvent => |_, event| { event }
    onkeypress(event: KeyboardEvent) -> KeyboardEvent => |_, event| { event }
    onkeydown(event: KeyboardEvent) -> KeyboardEvent => |_, event| { event }
    onkeyup(event: KeyboardEvent) -> KeyboardEvent => |_, event| { event }
    onmousemove(event: MouseEvent) -> MouseEvent => |_, event| { event }
    onmousedown(event: MouseEvent) -> MouseEvent => |_, event| { event }
    onmouseup(event: MouseEvent) -> MouseEvent => |_, event| { event }
    onmouseover(event: MouseEvent) -> MouseEvent => |_, event| { event }
    onmouseout(event: MouseEvent) -> MouseEvent => |_, event| { event }
    onmouseenter(event: MouseEvent) -> MouseEvent => |_, event| { event }
    onmouseleave(event: MouseEvent) -> MouseEvent => |_, event| { event }
    onmousewheel(event: MouseEvent) -> MouseEvent => |_, event| { event }
    ongotpointercapture(event: PointerEvent) -> PointerEvent => |_, event| { event }
    onlostpointercapture(event: PointerEvent) -> PointerEvent => |_, event| { event }
    onpointercancel(event: PointerEvent) -> PointerEvent => |_, event| { event }
    onpointerdown(event: PointerEvent) -> PointerEvent => |_, event| { event }
    onpointerenter(event: PointerEvent) -> PointerEvent => |_, event| { event }
    onpointerleave(event: PointerEvent) -> PointerEvent => |_, event| { event }
    onpointermove(event: PointerEvent) -> PointerEvent => |_, event| { event }
    onpointerout(event: PointerEvent) -> PointerEvent => |_, event| { event }
    onpointerover(event: PointerEvent) -> PointerEvent => |_, event| { event }
    onpointerup(event: PointerEvent) -> PointerEvent => |_, event| { event }
    onscroll(event: MouseScrollEvent) -> MouseScrollEvent => |_, event| { event }
    onblur(event: FocusEvent) -> FocusEvent => |_, event| { event }
    onfocus(event: FocusEvent) -> FocusEvent => |_, event| { event }
    onsubmit(event: Event) -> Event => |_, event| { event }
    ondragstart(event: DragEvent) -> DragEvent => |_, event| { event }
    ondrag(event: DragEvent) -> DragEvent => |_, event| { event }
    ondragend(event: DragEvent) -> DragEvent => |_, event| { event }
    ondragenter(event: DragEvent) -> DragEvent => |_, event| { event }
    ondragleave(event: DragEvent) -> DragEvent => |_, event| { event }
    ondragover(event: DragEvent) -> DragEvent => |_, event| { event }
    ondragexit(event: DragEvent) -> DragEvent => |_, event| { event }
    ondrop(event: DragEvent) -> DragEvent => |_, event| { event }
    oncontextmenu(event: MouseEvent) -> MouseEvent => |_, event| { event }
    oninput(event: InputEvent) -> InputData => |this: &Element, _| {
        use web_sys::{HtmlInputElement, HtmlTextAreaElement};
        let value = match this.clone().dyn_into() {
            Ok(input) => {
                let input: HtmlInputElement = input;
                input.value()
            }
            Err(_) => {
                match this.clone().dyn_into() {
                    Ok(tae) => {
                        let tae: HtmlTextAreaElement = tae;
                        tae.value()
                    }
                    Err(_) => {
                        panic!("only an HtmlInputElement or HtmlTextAreaElement can have an oninput event listener");
                    }
                }
            }
        };
        InputData { value }
    }
    onchange(event: Event) -> ChangeData => |this: &Element, _| {
        use web_sys::{HtmlInputElement, HtmlTextAreaElement, HtmlSelectElement};
        match this.node_name().as_ref() {
            "INPUT" => {
                let input: HtmlInputElement = this.clone().dyn_into().unwrap();
                ChangeData::Value(input.value())
            }
            "TEXTAREA" => {
                let tae: HtmlTextAreaElement = this.clone().dyn_into().unwrap();
                ChangeData::Value(tae.value())
            }
            "SELECT" => {
                let se: HtmlSelectElement = this.clone().dyn_into().unwrap();
                ChangeData::Select(se)
            }
            _ => {
                panic!("only an InputElement, TextAreaElement or SelectElement can have an onchange event listener");
            }
        }
    }
}

/// A type representing data from `oninput` event.
#[derive(Debug)]
pub struct InputData {
    /// Inserted characters. Contains value from
    /// [InputEvent](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/data).
    pub value: String,
}

// There is no '.../Web/API/ChangeEvent/data' (for onchange) similar to
// https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/data (for oninput).
// ChangeData actually contains the value of the InputElement/TextAreaElement
// after `change` event occured or contains the SelectElement (see more at the
// variant ChangeData::Select)

/// A type representing change of value(s) of an element after committed by user
/// ([onchange event](https://developer.mozilla.org/en-US/docs/Web/Events/change)).
#[derive(Debug)]
pub enum ChangeData {
    /// Value of the element in cases of `<input>`, `<textarea>`
    Value(String),
    /// SelectElement in case of `<select>` element. You can use one of methods of SelectElement
    /// to collect your required data such as: `value`, `selected_index`, `selected_indices` or
    /// `selected_values`. You can also iterate throught `selected_options` yourself.
    Select(HtmlSelectElement),
}

/// A bridging type for checking `href` attribute value.
#[derive(Debug)]
pub struct Href {
    link: String,
}

impl From<String> for Href {
    fn from(link: String) -> Self {
        Href { link }
    }
}

impl<'a> From<&'a str> for Href {
    fn from(link: &'a str) -> Self {
        Href {
            link: link.to_owned(),
        }
    }
}

impl ToString for Href {
    fn to_string(&self) -> String {
        self.link.to_owned()
    }
}
