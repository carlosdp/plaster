use plaster::prelude::*;

/// An <input type="text" /> field
pub struct Checkbox {
    label: String,
    value: bool,
    on_change: Option<Callback<bool>>,
}

pub enum Msg {
    Click,
}

#[derive(Default, Clone, PartialEq)]
pub struct Props {
    /// The input label
    pub label: String,
    /// The controlled value of the input
    pub value: bool,
    /// A callback that is fired when the user changes the input value
    pub on_change: Option<Callback<bool>>,
}

impl Component for Checkbox {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: ComponentLink<Self>) -> Self {
        Checkbox {
            label: props.label,
            value: props.value,
            on_change: props.on_change,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.label = props.label;
        self.value = props.value;
        self.on_change = props.on_change;

        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.value = !self.value;

                if let Some(ref callback) = self.on_change {
                    callback.emit(self.value);
                }
            }
        };

        true
    }
}

impl Renderable<Checkbox> for Checkbox {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="checkbox",>
                <input
                    type="checkbox",
                    checked=self.value,
                    onclick=|_| Msg::Click,
                />
                <div class="checkbox-label",>{&self.label}</div>
            </div>
        }
    }
}
