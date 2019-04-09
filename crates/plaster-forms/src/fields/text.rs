use plaster::prelude::*;

/// An <input type="text" /> field
pub struct TextField {
    label: String,
    value: String,
    password: bool,
    on_change: Option<Callback<String>>,
}

pub enum Msg {
    Change(InputData),
}

#[derive(Default, Clone, PartialEq)]
pub struct Props {
    /// The input label
    pub label: String,
    /// The controlled value of the input
    pub value: Option<String>,
    /// Whether or not this is a password field
    pub password: bool,
    /// A callback that is fired when the user changes the input value
    pub on_change: Option<Callback<String>>,
}

impl Component for TextField {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: ComponentLink<Self>) -> Self {
        TextField {
            label: props.label,
            value: props.value.unwrap_or(String::new()),
            password: props.password,
            on_change: props.on_change,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let mut updated = false;

        if props.on_change != self.on_change {
            self.on_change = props.on_change;
            updated = true;
        }

        if let Some(value) = props.value {
            if value != self.value {
                self.value = value;
                updated = true;
            }
        }

        if props.label != self.label {
            self.label = props.label;
            updated = true;
        }

        updated
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(data) => {
                if let Some(ref callback) = self.on_change {
                    callback.emit(data.value.clone());
                }

                self.value = data.value;
            }
        };

        true
    }
}

impl Renderable<TextField> for TextField {
    fn view(&self) -> Html<Self> {
        let label = html! { <label style="margin-right: 10px",>{&self.label}</label> };

        let ty = if self.password { "password" } else { "text" };

        html! {
            <div>
                {label}
                <input type=ty, oninput=|data| Msg::Change(data), value=&self.value,/>
            </div>
        }
    }
}
