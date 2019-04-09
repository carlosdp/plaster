use crate::fields::text::TextField;
use plaster::prelude::*;
use std::collections::HashMap;

/// A key/value field
pub struct KeyValue {
    label: Option<String>,
    value: Vec<(String, String)>,
    on_change: Option<Callback<HashMap<String, String>>>,
}

pub enum Msg {
    ChangeKey(usize, String),
    ChangeValue(usize, String),
    AddKey,
}

#[derive(Default, Clone, PartialEq)]
pub struct Props {
    /// The input label
    pub label: Option<String>,
    /// The controlled value of the input
    pub value: Option<HashMap<String, String>>,
    /// A callback that is fired when the user changes the input value
    pub on_change: Option<Callback<HashMap<String, String>>>,
}

impl Component for KeyValue {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: ComponentLink<Self>) -> Self {
        let initial_value = props
            .value
            .map(|h| h.into_iter().collect())
            .unwrap_or(Vec::new());

        KeyValue {
            label: props.label,
            value: initial_value,
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
            let mut existing_keys = Vec::new();

            self.value = self
                .value
                .clone()
                .into_iter()
                .filter_map(|(k, v)| {
                    if let Some(val) = value.get(&k) {
                        existing_keys.push(k.clone());

                        if val != &v {
                            updated = true;
                            Some((k, val.to_string()))
                        } else {
                            Some((k, v))
                        }
                    } else {
                        None
                    }
                })
                .collect();

            value.into_iter().for_each(|(k, v)| {
                if !existing_keys.contains(&k) {
                    self.value.push((k, v));
                }
            });
        }

        if props.label != self.label {
            self.label = props.label;
            updated = true;
        }

        updated
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeKey(i, value) => {
                if self.value.len() > i {
                    self.value.get_mut(i).unwrap().0 = value;

                    if let Some(ref callback) = self.on_change {
                        callback.emit(self.value.clone().into_iter().collect());
                    }
                }
            }
            Msg::ChangeValue(i, value) => {
                if self.value.len() > i {
                    self.value.get_mut(i).unwrap().1 = value;

                    if let Some(ref callback) = self.on_change {
                        callback.emit(self.value.clone().into_iter().collect());
                    }
                }
            }
            Msg::AddKey => {
                self.value.push((String::new(), String::new()));
            }
        };

        true
    }
}

impl Renderable<KeyValue> for KeyValue {
    fn view(&self) -> Html<Self> {
        let label = self
            .label
            .as_ref()
            .map(|l| html! { <label>{l}</label> })
            // todo: render nothing instead
            .unwrap_or(html! { <span /> });

        let items = self.value.iter().enumerate().map(|(i, (k, v))| {
            html! {
                <div>
                    <TextField:
                        value=Some(k.to_string()),
                        on_change=move |s| Msg::ChangeKey(i, s),
                    />
                    <TextField:
                        value=Some(v.to_string()),
                        on_change=move |s| Msg::ChangeValue(i, s),
                    />
                </div>
            }
        });

        html! {
            <div>
                {label}
                {for items}
                <div>
                    <a
                        href="",
                        onclick=|e| { e.prevent_default(); Msg::AddKey },
                    >{"+"}</a>
                </div>
            </div>
        }
    }
}
