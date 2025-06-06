use gloo::{
    net::http,
    storage::{LocalStorage, Storage},
};
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::hooks::{use_location, use_navigator};

use crate::{
    check_is_admin,
    components::{
        elements::paginate::{Paginate, Q},
        footer::Footer,
        header::component::HeaderComponent,
        user::{list::UserList, modal::Modal},
        PER_PAGE,
    },
    AppContext, ResponseId, ResponseItems, ResponseMsg, Role, Route, User,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestData {
    email: String,
    fio: String,
    role: Role,
    blocked: Option<bool>,
    organization_id: Option<i64>,
}

#[function_component(UserComponent)]
pub fn user() -> Html {
    // Компонент домашней страницы

    let ctx = use_context::<AppContext>();
    let current_user: Option<User> = ctx.and_then(|ctx| ctx.0.clone());

    let location = use_location().unwrap();
    let page = location.query::<Q>().map(|it| it.page).unwrap_or(1);
    let rendered = use_state_eq(|| false);

    let is_visible = use_state_eq(|| false);

    let item: UseStateHandle<Option<User>> = use_state_eq(|| None);
    let items: UseStateHandle<ResponseItems<User>> = use_state_eq(|| ResponseItems {
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

                let response = http::Request::get("/api/users")
                    .header("Content-Type", "application/json")
                    .header("Authorization", &header_bearer)
                    .query([
                        ("page", page.clone().to_string().as_str()),
                        ("per_page", PER_PAGE.to_string().as_str()),
                    ])
                    .send()
                    .await
                    .unwrap()
                    .json::<ResponseItems<User>>()
                    .await
                    .unwrap();

                items.set(response);
                cloned_rendered.set(false);
            });
            || ()
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

    let on_edit = {
        let cloned_item = item.clone();
        let cloned_is_visible = is_visible.clone();
        Callback::from(move |item: User| {
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
        Callback::from(move |(email, fio, role, blocked, organization_id)| {
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
                    email,
                    fio,
                    role,
                    blocked,
                    organization_id,
                };

                // Хак для Home
                let path = "/api/users";

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
                    navigator.push(&Route::User);
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
        <div class="overflow-auto rounded-lg border border-gray-200 shadow-md mx-5 my-2 max-h-[68%]">
            <table class="w-full border-collapse bg-white text-left text-sm text-gray-500 table-auto">
                <thead class="bg-gray-50 sticky top-0">
                    <tr>
                        <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"#"}</th>
                        <th scope="col" class="w-0.5 px-6 py-4 font-medium text-gray-900 uppercase"></th>
                        <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"Фио"}</th>
                        if current_user.as_ref().map_or(false, |u| check_is_admin(u.role)) {
                             <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"Организация"}</th>
                        }
                        <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"Роль"}</th>
                        <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"Блокировка"}</th>
                        <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"Дата создания"}</th>
                        <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase"></th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-gray-100 border-t border-gray-100">
                    <UserList
                        items={items.items.clone()}
                        current_user={current_user.clone()}
                        {on_edit}
                    />
                </tbody>
            </table>
        </div>

        // Paginate
        if items.cnt > 0 {
            <Paginate
                cnt={items.cnt}
                path={Route::User}
                page={page}
                per_page={PER_PAGE}
            />
        }

        <Modal
            current_user={current_user}
            is_visible={*is_visible}
            item={(*item).clone()}
            {toggle_modal}
            {on_save}
        />

        <Footer />

        </>
    }
}
