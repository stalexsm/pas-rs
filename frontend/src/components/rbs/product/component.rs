use gloo::storage::{LocalStorage, Storage};
use gloo_net::http;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use yew::prelude::*;
use yew_router::hooks::{use_location, use_navigator};

use crate::{
    components::{
        elements::{
            modal::ModalDelete,
            paginate::{Paginate, Q},
        },
        footer::Footer,
        header::component::HeaderComponent,
        rbs::product::Product,
        rbs::product::{list::ProductList, modal::Modal},
        PER_PAGE,
    },
    AppContext, ResponseId, ResponseItems, ResponseMsg, Route, User,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestData {
    measure_unit_id: i64,
    name: String,
}

#[function_component(ProductComponent)]
pub fn product() -> Html {
    // Компонент домашней страницы

    let ctx = use_context::<AppContext>();
    let current_user: Option<User> = ctx.and_then(|ctx| ctx.0.clone());

    let location = use_location().unwrap();
    let page = location.query::<Q>().map(|it| it.page).unwrap_or(1);

    let rendered = use_state_eq(|| false);
    let is_visible = use_state_eq(|| false);
    let is_visible_del = use_state_eq(|| false);

    let item: UseStateHandle<Option<Product>> = use_state_eq(|| None);
    let items: UseStateHandle<ResponseItems<Product>> = use_state(|| ResponseItems {
        cnt: 0,
        items: vec![],
    });
    {
        let items = items.clone();
        use_effect_with((page, rendered.clone()), move |(page, rendered)| {
            let items = items.clone();
            let page = *page;
            let cloned_rendered = rendered.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut header_bearer = String::from("Bearer ");
                let token: Option<String> = LocalStorage::get("token").unwrap_or(None);
                if let Some(t) = token.clone() {
                    header_bearer.push_str(&t);
                }

                let response = http::Request::get("/api/products")
                    .header("Content-Type", "application/json")
                    .header("Authorization", &header_bearer)
                    .query([
                        ("page", page.clone().to_string().as_str()),
                        ("per_page", PER_PAGE.to_string().as_str()),
                    ])
                    .send()
                    .await
                    .unwrap()
                    .json::<ResponseItems<Product>>()
                    .await
                    .unwrap();

                items.set(response);
                cloned_rendered.set(false);
            });
        });
    }

    let cloned_is_visible = is_visible.clone();
    let toggle_modal = {
        let cloned_item = item.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            cloned_is_visible.set(!*cloned_is_visible);
            cloned_item.set(None); //Сбросим state для Item редактирование
        })
    };

    let cloned_is_visible_del = is_visible_del.clone();
    let toggle_modal_del = {
        let cloned_item = item.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            cloned_is_visible_del.set(!*cloned_is_visible_del);
            cloned_item.set(None); //Сбросим state для Item редактирование
        })
    };

    let on_delete_modal = {
        let cloned_item = item.clone();
        let cloned_is_visible_del = is_visible_del.clone();
        Callback::from(move |item: Product| {
            cloned_item.set(Some(item));
            // Toggle modal
            cloned_is_visible_del.set(!*cloned_is_visible_del);
        })
    };

    let on_edit = {
        let cloned_item = item.clone();
        let cloned_is_visible = is_visible.clone();
        Callback::from(move |item: Product| {
            cloned_item.set(Some(item));
            // Toggle modal
            cloned_is_visible.set(!*cloned_is_visible);
        })
    };

    let on_save = {
        // todo
        let cloned_is_visible = is_visible.clone();
        let cloned_item = item.clone();
        let cloned_rendered = rendered.clone();
        let navigator = use_navigator();
        Callback::from(move |(measure_unit_id, name)| {
            let mut header_bearer = String::from("Bearer ");
            let token: Option<String> = LocalStorage::get("token").unwrap_or(None);
            if let Some(t) = token.clone() {
                header_bearer.push_str(&t);
            }

            let cloned_is_visible = cloned_is_visible.clone();
            let cloned_item = cloned_item.clone();
            let cloned_rendered = cloned_rendered.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let req_data = RequestData {
                    measure_unit_id,
                    name,
                };
                // Хак для Home
                let path = "/api/products";

                if let Some(item) = (*cloned_item).clone() {
                    let _: ResponseMsg = http::Request::patch(&format!("{}/{}", path, item.id))
                        .header("Content-Type", "application/json")
                        .header("Authorization", &header_bearer)
                        .json(&req_data)
                        .unwrap()
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                } else {
                    let _: ResponseId = http::Request::post(path)
                        .header("Content-Type", "application/json")
                        .header("Authorization", &header_bearer)
                        .json(&req_data)
                        .unwrap()
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                }

                cloned_is_visible.set(!*cloned_is_visible);
                cloned_rendered.set(true); // для перерисовки списка после действий.

                if let Some(navigator) = navigator {
                    navigator.push(&Route::Product);
                }
            });
        })
    };

    let on_delete = {
        // todo
        let cloned_is_visible_del = is_visible_del.clone();
        let cloned_item = item.clone();
        let cloned_rendered = rendered.clone();
        let navigator = use_navigator();
        Callback::from(move |_| {
            let mut header_bearer = String::from("Bearer ");
            let token: Option<String> = LocalStorage::get("token").unwrap_or(None);
            if let Some(t) = token.clone() {
                header_bearer.push_str(&t);
            }

            let cloned_is_visible_del = cloned_is_visible_del.clone();
            let cloned_item = cloned_item.clone();
            let cloned_rendered = cloned_rendered.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let path = "/api/products";

                if let Some(item) = (*cloned_item).clone() {
                    let _: ResponseMsg = http::Request::delete(&format!("{}/{}", path, item.id))
                        .header("Content-Type", "application/json")
                        .header("Authorization", &header_bearer)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                }

                cloned_is_visible_del.set(!*cloned_is_visible_del);
                cloned_rendered.set(true); // для перерисовки списка после действий.

                if let Some(navigator) = navigator {
                    navigator.push(&Route::Product);
                }
            });
        })
    };

    html! {
        <>
        <HeaderComponent />

        <div class="flex justify-end mb-5">
            <button
                onclick={toggle_modal.clone()}
                class="px-4 py-2 bg-blue-500 text-white rounded-md mr-5 mt-5 hover:bg-blue-700">
                {"Добавить"}
            </button>
        </div>
        <div class="overflow-auto rounded-lg border border-gray-200 shadow-md m-5 max-h-[68%]">
            <table class="w-full border-collapse bg-white text-left text-sm text-gray-500 table-auto">
                <thead class="bg-gray-50 sticky top-0">
                    <tr>
                    <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"#"}</th>
                    <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"Название"}</th>
                    <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"Ед. измерения"}</th>
                    <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"Дата создания"}</th>
                    <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase"></th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-gray-100 border-t border-gray-100">
                    <ProductList
                        items={items.deref().items.clone()}
                        current_user={current_user}
                        {on_edit}
                        on_delete={on_delete_modal}
                    />
                </tbody>
            </table>
        </div>

        // Paginate
        if items.cnt > 0 {
            <Paginate
                cnt={items.cnt}
                path={Route::Product}
                page={page}
                per_page={PER_PAGE}
            />
        }

        <ModalDelete
            is_visible={*is_visible_del}
            toggle={toggle_modal_del}
            {on_delete}
        />

        <Modal
            is_visible={*is_visible}
            item={(*item).clone()}
            {toggle_modal}
            {on_save}
        />

        <Footer />

        </>
    }
}
