use serde::{Deserialize, Serialize};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::Node;
use yew::prelude::*;

pub mod analitic;
pub mod auth;
pub mod elements;
pub mod footer;
pub mod header;
pub mod home;
pub mod not_found;
pub mod organization;
pub mod rbs;
pub mod user;

// Для пагинации
const PER_PAGE: i64 = 8;

pub trait SelectableItem {
    // Для возможности использовать объект для select

    fn id(&self) -> i64;
    fn name(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseError {
    pub detail: String,
}

#[hook]
fn use_outside_click(node_ref: NodeRef, callback: Callback<MouseEvent>) {
    // Функция для отслеживания клика вне элемента

    use_effect_with(node_ref, move |node_ref| {
        let document = web_sys::window().unwrap().document().unwrap();

        let element = node_ref.get().unwrap();
        let listener = {
            let cloned_element = element.clone();
            Closure::new(Box::new(move |event: web_sys::MouseEvent| {
                let e = event.target().unwrap().dyn_into::<Node>().ok();
                if !cloned_element.contains(e.as_ref()) {
                    log::debug!("Run!!! use_outside_click");
                    callback.emit(event);
                }
            }) as Box<dyn FnMut(_)>)
        };

        let closure = listener.as_ref().clone();
        document
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .unwrap();

        move || {
            document
                .remove_event_listener_with_callback("mousedown", listener.as_ref().unchecked_ref())
                .unwrap();
        }
    });
}
