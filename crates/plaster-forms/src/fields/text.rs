use crate::fields::ValidationFn;
use plaster::prelude::*;

/// An <input type="text" /> field
pub struct TextField {
    label: String,
    value: String,
    password: bool,
    class: String,
    validate: ValidationFn<String>,
    validation_error: Option<String>,
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
    /// HTML class
    pub class: String,
    /// A function that returns a validation error
    pub validate: ValidationFn<String>,
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
            class: props.class,
            validate: props.validate,
            validation_error: None,
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

        if props.class != self.class {
            self.class = props.class;
            updated = true;
        }

        self.validate = props.validate;
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

                self.validation_error = self.validate.validate(self.value.clone());
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
        let ty = if self.password { "password" } else { "text" };

        let (class, error) = if let Some(ref err) = self.validation_error {
            (
                format!("{} error", &self.class),
                html! {
                    <div class="input-error",>
                        {err}
                    </div>
                },
            )
        } else {
            (self.class.clone(), html!(<span />))
        };

        #[cfg(not(feature = "ionic"))]
        {
            html! {
                <div class=class,>
                    <input
                        type=ty,
                        placeholder=&self.label,
                        value=&self.value,
                        oninput=|data| Msg::Change(data),
                        onblur=|_| Msg::Blur,
                    />
                    {error}
                </div>
            }
        }

        #[cfg(feature = "ionic")]
        {
            use wasm_bindgen::JsCast;

            #[derive(serde_derive::Deserialize)]
            struct Detail {
                value: String,
            }

            html! {
                <ion_item>
                    <ion_input
                        type=ty,
                        placeholder=&self.label,
                        value=&self.value,
                        [ionChange]=|event: Event| {
                            let c: web_sys::CustomEvent = event.dyn_into().expect("is not custom event");
                            let detail: Detail = c.detail().into_serde().unwrap();
                            Msg::Change(InputData { value: detail.value })
                        },
                    />
                    {error}
                </ion_item>
            }
        }
    }
}
