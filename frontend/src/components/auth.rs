use gloo::storage::{LocalStorage, Storage};
use gloo_net::http;
use log::debug;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    components::{
        elements::{error::AlertError, input::Input},
        ResponseError,
    },
    AppContext, Route, User,
};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub email: String,
    pub passwd: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseData {
    token: String,
}

#[function_component(AuthComponent)]
pub fn auth() -> Html {
    let state = use_state(State::default);
    let detail = use_state(String::new);
    let alert_visible = use_state(|| false);

    let cloned_alert_visible = alert_visible.clone();
    let toggle_alert_err = Callback::from(move |visible| {
        cloned_alert_visible.set(visible);
    });

    let ctx = use_context::<AppContext>();

    let cloned_state = state.clone();
    let email_changed = Callback::from(move |email| {
        let mut data = (*cloned_state).clone();
        data.email = email;

        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let passwd_changed = Callback::from(move |passwd| {
        let mut data = (*cloned_state).clone();
        data.passwd = passwd;

        cloned_state.set(data);
    });

    let navigator = use_navigator();
    let cloned_state = state.clone();
    let ctx = ctx.clone();
    let cloned_detail = detail.clone();
    let cloned_alert_visible = alert_visible.clone();
    let onclick = Callback::from(move |e: MouseEvent| {
        e.prevent_default();

        let request_data = State {
            ..(*cloned_state).clone()
        };
        let navigator = navigator.clone();
        let ctx = ctx.clone();
        let cloned_detail = cloned_detail.clone();
        let cloned_alert_visible = cloned_alert_visible.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match http::Request::post("/api/auth")
                .header("Content-Type", "application/json")
                .json(&request_data)
                .unwrap()
                .send()
                .await
            {
                Ok(response_result) => {
                    if response_result.ok() {
                        let response_result: ResponseData = response_result.json().await.unwrap();

                        LocalStorage::set("token", response_result.token.clone())
                            .expect("Не удалось записать токен в локальное хранилище!");

                        let header_bearer = format!("Bearer {}", response_result.token);
                        let response_user = http::Request::get("/api/current")
                            .header("Content-Type", "application/json")
                            .header("Authorization", &header_bearer)
                            .send()
                            .await
                            .unwrap()
                            .json::<User>()
                            .await
                            .unwrap();

                        cloned_alert_visible.set(false);
                        cloned_detail.set("".to_string());

                        if let Some(ctx) = ctx {
                            ctx.dispatch(Some(response_user));
                        }

                        if let Some(navigator) = navigator {
                            navigator.push(&Route::Home);
                        }
                    } else {
                        let response_result: ResponseError = response_result.json().await.unwrap();
                        debug!("{:?}", response_result.detail);

                        cloned_alert_visible.set(true);
                        cloned_detail.set(response_result.detail);
                    }
                }
                Err(err) => {
                    debug!("{:?}", err);

                    cloned_alert_visible.set(true);
                    cloned_detail.set(err.to_string());
                }
            }
        });
    });

    html! {
        <>
        <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
            <div class="sm:mx-auto sm:w-full sm:max-w-sm">
            <img class="mx-auto h-10 w-auto" src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600" alt="Your Company"/>
            <h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">{"Войдите в свою учетную запись"}</h2>
            </div>
            <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
            <form class="space-y-6 group">
                <div>
                <label for="email" class="block text-sm font-medium leading-6 text-gray-900">{"Email"}</label>
                <div class="mt-2">
                    <Input
                        handle_onchange={email_changed}
                        name="email"
                        input_type="email"
                        required=true
                        disabled=false
                        classes="p-5 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                        />
                    </div>
                </div>
                <div>
                <div class="flex items-center justify-between">
                    <label for="password" class="block text-sm font-medium leading-6 text-gray-900">{"Пароль"}</label>
                    <div class="text-sm">
                    <a href="#"
                    class="font-semibold text-indigo-600 hover:text-indigo-500">
                    {"Забыли пароль?"}</a>
                    </div>
                </div>
                <div class="mt-2">
                <Input
                    handle_onchange={passwd_changed}
                    name="passwd"
                    input_type="password"
                    required=true
                    disabled=false
                    classes="p-5 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                    />
                </div>
                </div>
                <div>
                <button
                    {onclick}
                    class="group-invalid:pointer-events-none group-invalid:opacity-30 flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                    {"Войти"}
                </button>
                </div>
            </form>
            </div>
            // Alert Error
            <AlertError
                is_visible={*alert_visible}
                detail={(*detail).clone()}
                toggle={toggle_alert_err}
            />

        </div>

        </>
    }
}
