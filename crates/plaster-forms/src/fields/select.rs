use plaster::prelude::*;
use crate::fields::ValidationFn;

/// An autocompleting search select field
pub struct Select {
    label: String,
    search: String,
    searching: bool,
    selected_option: i32,
    value: Option<String>,
    value_label: String,
    validate: ValidationFn<Option<String>>,
    validation_error: Option<String>,
    inline: bool,
    options: Vec<(String, String)>,
    on_change: Option<Callback<Option<String>>>,
    on_blur: Option<Callback<()>>,
}

pub enum Msg {
    Change(InputData),
    SoftSelect(usize),
    Select(String),
    Focus,
    Blur,
    KeyDown(KeyboardEvent),
    Noop,
}

#[derive(Default, Clone, PartialEq)]
pub struct Props {
    /// The input label
    pub label: String,
    /// The controlled value of the input
    pub value: Option<String>,
    /// Validation function
    pub validate: ValidationFn<Option<String>>,
    /// Whether or not the field should be inline
    pub inline: bool,
    /// An array of options, (value, label)
    pub options: Vec<(String, String)>,
    /// A callback that is fired when the user changes the input value
    pub on_change: Option<Callback<Option<String>>>,
    /// A callback that is fired when the select loses focus
    pub on_blur: Option<Callback<()>>,
}

impl Component for Select {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: ComponentLink<Self>) -> Self {
        let value_label = if let Some(ref value) = props.value {
            props
                .options
                .iter()
                .find(|x| &x.0 == value)
                .map(|x| x.1.to_owned())
                .unwrap_or(String::new())
        } else {
            String::new()
        };

        Select {
            label: props.label,
            search: String::new(),
            searching: false,
            selected_option: -1,
            value: props.value,
            value_label,
            validate: props.validate,
            validation_error: None,
            inline: props.inline,
            options: props.options,
            on_change: props.on_change,
            on_blur: props.on_blur,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let mut updated = false;

        if props.value != self.value {
            self.value = props.value;
            self.value_label = if let Some(ref value) = self.value {
                props
                    .options
                    .iter()
                    .find(|x| &x.0 == value)
                    .map(|x| x.1.to_owned())
                    .unwrap_or(String::new())
            } else {
                String::new()
            };
            updated = true;
        }

        if props.options != self.options {
            self.options = props.options;
            updated = true;
        }

        if props.label != self.label {
            self.label = props.label;
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
                self.search = data.value;
                self.searching = true;

                let top_option = self.filtered_options().next();
                if let Some((i, _)) = top_option {
                    self.selected_option = i as i32;
                }
            }
            Msg::SoftSelect(i) => {
                self.selected_option = i as i32;
            }
            Msg::Select(value) => {
                self.value_label = self
                    .options
                    .iter()
                    .find(|x| x.0 == value)
                    .map(|x| x.1.to_owned())
                    .unwrap_or(String::new());
                self.value = Some(value);
                self.searching = false;

                if let Some(ref callback) = self.on_change {
                    callback.emit(self.value.clone());
                }

                self.validation_error = self.validate.validate(self.value.clone());
            }
            Msg::Focus => {
                self.searching = true;
            }
            Msg::Blur => {
                self.searching = false;

                if let Some(ref callback) = self.on_blur {
                    callback.emit(());
                }

                self.validation_error = self.validate.validate(self.value.clone());
            }
            Msg::KeyDown(e) => match e.key().as_str() {
                "ArrowUp" => {
                    if self.selected_option > 0 {
                        self.selected_option -= 1;
                    }
                }
                "ArrowDown" => {
                    if self.selected_option < self.options.len() as i32 {
                        self.selected_option += 1;
                    }
                }
                "Enter" => {
                    if self.selected_option >= 0 {
                        let selected = self.options.get(self.selected_option as usize).unwrap();
                        self.value_label = selected.1.clone();
                        self.value = Some(selected.0.clone());
                        self.searching = false;

                        if let Some(ref callback) = self.on_change {
                            callback.emit(self.value.clone());
                        }
                    }
                }
                _ => (),
            },
            Msg::Noop => (),
        };

        true
    }
}

impl Select {
    fn filtered_options<'a>(&'a self) -> impl Iterator<Item=(usize, &'a (String, String))> + 'a {
        let search_term = self.search.to_lowercase();

        self.options
            .iter()
            .enumerate()
            .filter(move |(_, o)| o.1.to_lowercase().contains(&search_term))
    }
}

impl Renderable<Select> for Select {
    fn view(&self) -> Html<Self> {
        let class = if self.inline {
            "select-inline"
        } else {
            "select"
        };

        let value = if self.searching {
            &self.search
        } else {
            &self.value_label
        };

        let search_list = if self.searching {
            let options = self.filtered_options()
                .map(|(i, o)| {
                    let value = o.0.to_owned();

                    let class = if (i as i32) == self.selected_option {
                        "selected"
                    } else {
                        ""
                    };

                    html! {
                        <a
                            href="",
                            class=class,
                            onmousedown=|e| { e.prevent_default(); Msg::Noop },
                            onmouseenter=|_| Msg::SoftSelect(i),
                            onclick=|e| { e.prevent_default(); Msg::Select(value.clone()) },
                        >{&o.1}</a>
                    }
                });

            html! {
                <div class="select-drop",>
                    {for options}
                </div>
            }
        } else {
            html! {
                <span />
            }
        };

        let (class, error) = if let Some(ref err) = self.validation_error {
            if !self.searching {
                (
                    format!("{} error", class),
                    html! {
                        <div class="input-error",>
                            {err}
                        </div>
                    }
                )
            } else {
                (class.to_owned(), html!(<span />))
            }
        } else {
            (class.to_owned(), html!(<span />))
        };

        html! {
            <div class="select-wrapper",>
                <input
                    type="text",
                    class=class,
                    value=value,
                    name="search",
                    placeholder=&self.label,
                    oninput=|data| Msg::Change(data),
                    onfocus=|_| Msg::Focus,
                    onblur=|_| Msg::Blur,
                    onkeydown=|e| Msg::KeyDown(e),
                />
                {error}
                {search_list}
            </div>
        }
    }
}
