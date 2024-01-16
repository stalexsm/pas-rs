use std::ops::Deref;

use gloo::storage::{LocalStorage, Storage};
use gloo_net::http;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::hooks::{use_location, use_navigator};

use crate::{
    components::{
        elements::paginate::{Paginate, Q},
        footer::Footer,
        header::component::HeaderComponent,
        home::{list::ProducedGoodList, modal::Modal, ProducedGood},
        PER_PAGE,
    },
    AppContext, ResponseId, ResponseItems, ResponseMsg, Route, User,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestData {
    product_id: i64,
    cnt: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RequestDataAdj {
    cnt: i64,
}

#[function_component(HomeComponent)]
pub fn home() -> Html {
    // Компонент домашней страницы

    let ctx = use_context::<AppContext>();
    let current_user: Option<User> = ctx.and_then(|ctx| ctx.0.clone());

    let location = use_location().unwrap();
    let page = location.query::<Q>().map(|it| it.page).unwrap_or(1);
    let rendered = use_state_eq(|| false);

    let is_visible = use_state_eq(|| false);
    let is_adj = use_state_eq(|| false);

    let item: UseStateHandle<Option<ProducedGood>> = use_state_eq(|| None);
    let items: UseStateHandle<ResponseItems<ProducedGood>> = use_state_eq(|| ResponseItems {
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

                let response = http::Request::get("/api/produced-goods")
                    .header("Content-Type", "application/json")
                    .header("Authorization", &header_bearer)
                    .query([
                        ("page", page.clone().to_string().as_str()),
                        ("per_page", PER_PAGE.to_string().as_str()),
                    ])
                    .send()
                    .await
                    .unwrap()
                    .json::<ResponseItems<ProducedGood>>()
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
        let cloned_is_adj = is_adj.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            cloned_is_visible.set(!*cloned_is_visible);
            cloned_item.set(None); //Сбросим state для Item редактирование
            cloned_is_adj.set(false); // Сбросим добавление корректировки
        })
    };

    let on_edit = {
        let cloned_item = item.clone();
        let cloned_is_visible = is_visible.clone();
        Callback::from(move |item: ProducedGood| {
            cloned_item.set(Some(item));
            // Toggle modal
            cloned_is_visible.set(!*cloned_is_visible);
        })
    };

    let on_add_adj = {
        let cloned_item = item.clone();
        let cloned_is_visible = is_visible.clone();
        let cloned_is_adj = is_adj.clone();
        Callback::from(move |item: ProducedGood| {
            cloned_item.set(Some(item));
            // Toggle modal
            cloned_is_visible.set(!*cloned_is_visible);
            cloned_is_adj.set(true);
        })
    };

    let on_save_adj = {
        let cloned_is_visible = is_visible.clone();
        let cloned_item = item.clone();
        let cloned_rendered = rendered.clone();
        let cloned_is_adj = is_adj.clone();
        let navigator = use_navigator();
        Callback::from(move |cnt| {
            let mut header_bearer = String::from("Bearer ");
            let token: Option<String> = LocalStorage::get("token").unwrap_or(None);
            if let Some(t) = token.clone() {
                header_bearer.push_str(&t);
            }

            let cloned_is_visible = cloned_is_visible.clone();
            let cloned_item = cloned_item.clone();
            let cloned_rendered = cloned_rendered.clone();
            let cloned_is_adj = cloned_is_adj.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let req_data = RequestDataAdj { cnt };
                // Хак для Home
                let path = "/api/produced-goods";

                if let Some(item) = (*cloned_item).clone() {
                    let _: ResponseMsg = http::Request::post(&format!("{}/{}/adj", path, item.id))
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
                cloned_is_adj.set(false);

                if let Some(navigator) = navigator {
                    navigator.push(&Route::Home);
                }
            });
        })
    };

    let on_save = {
        // todo
        let cloned_is_visible = is_visible.clone();
        let cloned_item = item.clone();
        let cloned_rendered = rendered.clone();
        let navigator = use_navigator();
        Callback::from(move |(product_id, cnt)| {
            // e.prevent_default();

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
                let req_data = RequestData { product_id, cnt };
                // Хак для Home
                let path = "/api/produced-goods";

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
                    navigator.push(&Route::Home);
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
        <div class="overflow-auto rounded-lg border border-gray-200 shadow-md m-5">
            <table class="w-full border-collapse bg-white text-left text-sm text-gray-500 table-auto">
                <thead class="bg-gray-50">
                    <tr>
                        <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"#"}</th>
                        <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"Продукт"}</th>
                        <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"Пользователь"}</th>
                        <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"Корректировки"}</th>
                        <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"Кол-во"}</th>
                        <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"Дата создания"}</th>
                        <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase"></th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-gray-100 border-t border-gray-100">
                    <ProducedGoodList
                        items={items.deref().items.clone()}
                        current_user={current_user}
                        {on_edit}
                        {on_add_adj}
                    />
                </tbody>
            </table>
        </div>

        // Paginate
        if items.cnt > 0 {
            <Paginate
                cnt={items.cnt}
                path={Route::Home}
                page={page}
                per_page={PER_PAGE}
            />
        }

        <Modal
            is_visible={*is_visible}
            is_adj={*is_adj}
            item={(*item).clone()}
            {toggle_modal}
            {on_save}
            {on_save_adj}
        />

        <Footer />

        </>
    }
}
