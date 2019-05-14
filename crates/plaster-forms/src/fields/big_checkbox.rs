use plaster::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct CheckboxOption {
    pub key: String,
    pub label: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub disabled: bool,
}

/// A big enum checkbox field
pub struct BigCheckbox {
    label: String,
    value: Vec<String>,
    options: Vec<CheckboxOption>,
    radio: bool,
    on_change: Option<Callback<Vec<String>>>,
}

pub enum Msg {
    Click(String, bool),
}

#[derive(Default, Clone, PartialEq)]
pub struct Props {
    /// The input label
    pub label: String,
    /// The controlled value of the input
    pub value: Vec<String>,
    /// The options for the checkboxes
    pub options: Vec<CheckboxOption>,
    /// Whether this should be a radio button
    pub radio: bool,
    /// A callback that is fired when the user changes the input value
    pub on_change: Option<Callback<Vec<String>>>,
}

impl Component for BigCheckbox {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: ComponentLink<Self>) -> Self {
        BigCheckbox {
            label: props.label,
            value: props.value,
            options: props.options,
            radio: props.radio,
            on_change: props.on_change,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.label = props.label;
        self.value = props.value;
        self.options = props.options;
        self.radio = props.radio;
        self.on_change = props.on_change;

        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(key, disabled) => {
                if !disabled {
                    if self.radio {
                        self.value = vec![key];
                    } else {
                        if self.value.contains(&key) {
                            self.value.retain(|x| x != &key);
                        } else {
                            self.value.push(key);
                        }
                    }

                    if let Some(ref callback) = self.on_change {
                        callback.emit(self.value.clone());
                    }
                }
            }
        };

        true
    }
}

impl Renderable<BigCheckbox> for BigCheckbox {
    fn view(&self) -> Html<Self> {
        let options = self.options.iter().map(|option| {
            let icon = if let Some(ref icon) = option.icon {
                html! {
                    <img class="big-enum-icon", src=icon, />
                }
            } else {
                html!(<span />)
            };

            let checked = if self.value.contains(&option.key) {
                html!(<img class="big-enum-select-indicator", src="/small-check-true.svg", />)
            } else {
                html!(<img class="big-enum-select-indicator", src="/small-check-false.svg", />)
            };
            let class = if option.disabled {
                "big-enum-select-item disabled"
            } else {
                "big-enum-select-item"
            };
            let key = option.key.clone();
            let disabled = option.disabled;

            html! {
                <div class=class, onclick=|_| Msg::Click(key.clone(), disabled),>
                    {checked}
                    {icon}
                    <div class="big-enum-title",>{&option.label}</div>
                    <div class="big-enum-content",>{option.description.as_ref().map(|x| x.as_str()).unwrap_or("")}</div>
                </div>
            }
        });

        html! {
            <div class="big-enum-select",>
                {for options}
            </div>
        }
    }
}
