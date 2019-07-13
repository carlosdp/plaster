use plaster::prelude::*;

/// An <input type="text" /> field
pub struct File {
    label: String,
    value: Vec<web_sys::File>,
    class: String,
    on_change: Option<Callback<Vec<web_sys::File>>>,
}

pub enum Msg {
    Change(ChangeData),
}

#[derive(Default, Clone, PartialEq)]
pub struct Props {
    /// The input label
    pub label: String,
    /// HTML class
    pub class: String,
    /// A callback that is fired when the user changes the input value
    pub on_change: Option<Callback<Vec<web_sys::File>>>,
}

impl Component for File {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _context: ComponentLink<Self>) -> Self {
        File {
            label: props.label,
            value: Vec::new(),
            class: props.class,
            on_change: props.on_change,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let mut updated = false;

        if props.label != self.label {
            self.label = props.label;
            updated = true;
        }

        if props.class != self.class {
            self.class = props.class;
            updated = true;
        }

        self.on_change = props.on_change;

        updated
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(data) => {
                if let ChangeData::Files(list) = data {
                    self.value.clear();

                    if let Some(files) = list {
                        for i in 0..files.length() {
                            self.value.push(files.get(i).unwrap());
                        }
                    }

                    if let Some(ref callback) = self.on_change {
                        callback.emit(self.value.clone());
                    }
                }
            }
        };

        false
    }
}

impl Renderable<File> for File {
    fn view(&self) -> Html<Self> {
        html! {
            <div class=&self.class,>
                <input
                    type="file",
                    onchange=|data| Msg::Change(data),
                />
            </div>
        }
    }
}
