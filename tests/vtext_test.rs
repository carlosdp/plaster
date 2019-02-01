#[macro_use]
extern crate plaster;
#[macro_use]
extern crate wasm_bindgen_test;

use plaster::prelude::*;
use plaster::virtual_dom::VNode;

struct Comp;

impl Component for Comp {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Comp
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!();
    }
}

impl Renderable<Comp> for Comp {
    fn view(&self) -> Html<Self> {
        unimplemented!();
    }
}

#[wasm_bindgen_test]
fn text_as_root() {
    let _: VNode<Comp> = html! {
        { "Text Node As Root" }
    };
}
