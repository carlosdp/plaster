#[macro_use]
extern crate plaster;

pub mod fields;

pub mod prelude {
    pub use crate::fields::{
        big_checkbox::BigCheckbox, checkbox::Checkbox, file::File, key_value::KeyValue,
        select::Select, text::TextField, ValidationFn,
    };
}

use plaster::prelude::*;

pub trait Form {
    type Value: Clone;

    fn value(&self) -> Self::Value;
}

#[derive(Clone, Default, PartialEq)]
pub struct TestValue {
    name: String,
}

pub struct TestForm {
    value: TestValue,
    submit_label: Option<String>,
    on_change: Option<Callback<TestValue>>,
    on_submit: Option<Callback<TestValue>>,
}

#[derive(Clone, Default, PartialEq)]
pub struct TestFormProps {
    default_value: Option<TestValue>,
    submit_label: Option<String>,
    on_change: Option<Callback<TestValue>>,
    on_submit: Option<Callback<TestValue>>,
}

pub enum TestFormMessage {
    UpdateName(String),
    Submit,
}

impl Component for TestForm {
    type Message = TestFormMessage;
    type Properties = TestFormProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> TestForm {
        TestForm {
            value: props.default_value.unwrap_or(TestValue::default()),
            submit_label: props.submit_label,
            on_change: props.on_change,
            on_submit: props.on_submit,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TestFormMessage::UpdateName(value) => {
                self.value.name = value;

                if let Some(ref callback) = self.on_change {
                    callback.emit(self.value.clone());
                }

                true
            }
            TestFormMessage::Submit => {
                if let Some(ref callback) = self.on_submit {
                    callback.emit(self.value.clone());
                }

                false
            }
        }
    }
}

impl Renderable<TestForm> for TestForm {
    fn view(&self) -> Html<Self> {
        html! {
            <form onsubmit=|e| { e.prevent_default(); TestFormMessage::Submit },>
                <fields::text::TextField:
                    label="Name",
                    on_change=|v| TestFormMessage::UpdateName(v),
                />
                <button>{self.submit_label.as_ref().map(|s| s.as_str()).unwrap_or("Submit")}</button>
            </form>
        }
    }
}

impl Form for TestForm {
    type Value = TestValue;

    fn value(&self) -> Self::Value {
        self.value.clone()
    }
}
