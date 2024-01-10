use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct Props {
    pub name: String,
    pub handle_onchange: Callback<String>,
    pub classes: String,
    pub input_type: String,
    pub required: bool,
    pub disabled: bool,
}

#[function_component(Input)]
pub fn text_input(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        handle_onchange.emit(value);
    });
    html! {
      <input
        class={props.classes.clone()}
        type={props.input_type.clone()}
        required={props.required}
        disabled={props.disabled}
        name={props.name.clone()}
        onchange={onchange}
        placeholder={props.name.clone()}
      />
    }
}
