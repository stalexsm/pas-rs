use gloo::{
    net::http,
    storage::{LocalStorage, Storage},
};
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

    let show_passwd = use_state_eq(|| false);
    let onclick_passwd = {
        let cloned_show_passwd = show_passwd.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            cloned_show_passwd.set(!*cloned_show_passwd);
        })
    };

    html! {
        <>
        <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
            <div class="sm:mx-auto sm:w-full sm:max-w-sm">
            <img class="mx-auto h-10 w-auto" src="./assets/img/logo.png" alt="PAS"/>
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
                <div class="mt-2 relative">
                    <Input
                        handle_onchange={passwd_changed}
                        name="passwd"
                        input_type={if *show_passwd {"text"} else {"password"}}
                        required=true
                        disabled=false
                        classes="p-5 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                        />
                    <span onclick={onclick_passwd} class="absolute inset-y-0 right-3.5 pl-3 flex items-center mt-auto text-gray-500 cursor-pointer">
                        if *show_passwd {
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M2.036 12.322a1.012 1.012 0 0 1 0-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178Z" />
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
                            </svg>
                        } else {
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M3.98 8.223A10.477 10.477 0 0 0 1.934 12C3.226 16.338 7.244 19.5 12 19.5c.993 0 1.953-.138 2.863-.395M6.228 6.228A10.451 10.451 0 0 1 12 4.5c4.756 0 8.773 3.162 10.065 7.498a10.522 10.522 0 0 1-4.293 5.774M6.228 6.228 3 3m3.228 3.228 3.65 3.65m7.894 7.894L21 21m-3.228-3.228-3.65-3.65m0 0a3 3 0 1 0-4.243-4.243m4.242 4.242L9.88 9.88" />
                            </svg>
                        }
                    </span>
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
