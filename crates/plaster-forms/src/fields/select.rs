use plaster::prelude::*;

/// An autocompleting search select field
pub struct Select {
    label: String,
    search: String,
    searching: bool,
    value: Option<String>,
    value_label: String,
    inline: bool,
    options: Vec<(String, String)>,
    on_change: Option<Callback<Option<String>>>,
    on_blur: Option<Callback<()>>,
}

pub enum Msg {
    Change(InputData),
    Select(String),
    Focus,
    Blur,
    Noop,
}

#[derive(Default, Clone, PartialEq)]
pub struct Props {
    /// The input label
    pub label: String,
    /// The controlled value of the input
    pub value: Option<String>,
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
            props.options.iter().find(|x| &x.0 == value).map(|x| x.1.to_owned()).unwrap_or(String::new())
        } else {
            String::new()
        };

        Select {
            label: props.label,
            search: String::new(),
            searching: false,
            value: props.value,
            value_label,
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
                props.options.iter().find(|x| &x.0 == value).map(|x| x.1.to_owned()).unwrap_or(String::new())
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

        self.on_change = props.on_change;
        self.on_blur = props.on_blur;

        updated
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(data) => {
                self.search = data.value;
            }
            Msg::Select(value) => {
                self.value_label = self.options.iter().find(|x| x.0 == value).map(|x| x.1.to_owned()).unwrap_or(String::new());
                self.value = Some(value);
                self.searching = false;

                if let Some(ref callback) = self.on_change {
                    callback.emit(self.value.clone());
                }
            }
            Msg::Focus => {
                self.searching = true;
            }
            Msg::Blur => {
                self.searching = false;

                if let Some(ref callback) = self.on_blur {
                    callback.emit(());
                }
            }
            Msg::Noop => (),
        };

        true
    }
}

impl Renderable<Select> for Select {
    fn view(&self) -> Html<Self> {
        let label = html! { <label style="margin-right: 10px",>{&self.label}</label> };

        let style = if self.inline { "display: inline-block; width: 190px;" } else { "width: 190px;" };

        let value = if self.searching {
            &self.search
        } else {
            &self.value_label
        };

        let search_list = if self.searching {
            let options = self.options.iter().map(|o| {
                let value = o.0.to_owned();

                html! {
                    <a
                        href="",
                        onmousedown=|e| { e.prevent_default(); Msg::Noop },
                        onclick=|e| { e.prevent_default(); Msg::Select(value.clone()) },
                    >{&o.1}</a>
                }
            });

            html! {
                <div style="position: absolute; margin: 0; height: 100px; width: 190px; background-color: white;",>
                    {for options}
                </div>
            }
        } else {
            html! {
                <span />
            }
        };

        html! {
            <div style=style,>
                {label}
                <input
                    type="text",
                    style=style,
                    value=value,
                    oninput=|data| Msg::Change(data),
                    onfocus=|_| Msg::Focus,
                    onblur=|_| Msg::Blur,
                />
                {search_list}
            </div>
        }
    }
}
