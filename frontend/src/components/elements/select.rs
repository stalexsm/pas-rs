use crate::components::SelectableItem;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default, Clone)]
pub struct Props<T>
where
    T: SelectableItem + 'static + PartialEq + Clone,
{
    pub name: String,
    pub handle_onchange: Callback<String>,
    pub classes: String,
    pub disabled: bool,
    pub items: Vec<T>,

    pub selected_id: Option<i64>,
}

#[function_component(Select)]
pub fn select<T>(props: &Props<T>) -> Html
where
    T: SelectableItem + 'static + PartialEq + Clone,
{
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlSelectElement>()
            .value();

        handle_onchange.emit(value);
    });
    html! {
      <select
        disabled={props.disabled}
        class={props.classes.clone()}
        name={props.name.clone()}
        onchange={onchange}
        placeholder={props.name.clone()}
      >
        { for props.items.iter().map(|item| html!{
            <option
                selected={props.selected_id.map_or(false, |s| s == item.id())}
                value={item.id().to_string()}>
                { item.name() }
            </option>})
        }
      </ select>
    }
}
