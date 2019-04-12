use plaster::prelude::*;

/// An <input type="text" /> field
pub struct TextField {
    label: String,
    value: String,
    password: bool,
    inline: bool,
    on_change: Option<Callback<String>>,
    on_blur: Option<Callback<()>>,
}

pub enum Msg {
    Change(InputData),
    Blur,
}

#[derive(Default, Clone, PartialEq)]
pub struct Props {
    /// The input label
    pub label: String,
    /// The controlled value of the input
    pub value: Option<String>,
    /// Whether or not this is a password field
    pub password: bool,
    /// Whether or not the field should be inline
    pub inline: bool,
    /// A callback that is fired when the user changes the input value
    pub on_change: Option<Callback<String>>,
    /// A callback that is fired when the field loses focus
    pub on_blur: Option<Callback<()>>,
}

impl Component for TextField {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: ComponentLink<Self>) -> Self {
        TextField {
            label: props.label,
            value: props.value.unwrap_or(String::new()),
            password: props.password,
            inline: props.inline,
            on_change: props.on_change,
            on_blur: props.on_blur,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let mut updated = false;

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

        self.on_change = props.on_change;
        self.on_blur = props.on_blur;

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
            Msg::Blur => {
                if let Some(ref callback) = self.on_blur {
                    callback.emit(());
                }
            }
        };

        true
    }
}

impl Renderable<TextField> for TextField {
    fn view(&self) -> Html<Self> {
        let label = html! { <label style="margin-right: 10px",>{&self.label}</label> };

        let ty = if self.password { "password" } else { "text" };

        let style = if self.inline { "display: inline" } else { "" };

        html! {
            <div style=style,>
                {label}
                <input
                    type=ty,
                    style=style,
                    value=&self.value,
                    oninput=|data| Msg::Change(data),
                    onblur=|_| Msg::Blur,
                />
            </div>
        }
    }
}
