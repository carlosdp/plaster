use plaster::prelude::*;
use crate::fields::ValidationFn;

/// An <input type="text" /> field
pub struct TextField {
    label: String,
    value: String,
    password: bool,
    class: String,
    inline: bool,
    options: Vec<String>,
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
    /// Whether or not the field should be inline
    pub inline: bool,
    /// If it's an autocomplete, an array of options
    pub options: Vec<String>,
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
            inline: props.inline,
            options: props.options,
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
                }
            )
        } else {
            (self.class.clone(), html!(<span />))
        };

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
}
