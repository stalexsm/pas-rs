use gloo::{
    net::http,
    storage::{LocalStorage, Storage},
};
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::hooks::{use_location, use_navigator};
use yew_router::prelude::*;

use crate::{check_is_admin, components::use_outside_click, AppContext, ResponseMsg, Route, User};
use crate::{components::header::modal::Modal, Role};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestData {
    passwd1: String,
    passwd2: String,
}

#[function_component(HeaderComponent)]
pub fn header() -> Html {
    // Context
    let ctx = use_context::<AppContext>();
    let current_user: Option<User> = ctx.and_then(|ctx| ctx.0.clone());

    let modal_visible = use_state(|| false);
    let edit_passwd_visible = use_state_eq(|| false);

    let mobile_visible_menu = use_state(|| false);

    let mut menus: Vec<(Route, String)> = vec![(Route::Home, String::from("Производство"))];
    if let Some(u) = current_user.clone() {
        if u.role == Role::Director {
            menus.extend(vec![
                (Route::Product, String::from("Товары")),
                (Route::MeasureUnit, String::from("Единицы измерения")),
                (Route::User, String::from("Пользователи")),
                (Route::Analitic, String::from("Аналитика")),
            ])
        }

        if check_is_admin(u.role) {
            menus.extend(vec![
                (Route::Product, String::from("Товары")),
                (Route::MeasureUnit, String::from("Единицы измерения")),
                (Route::User, String::from("Пользователи")),
                (Route::Organization, String::from("Организации")),
                (Route::Analitic, String::from("Аналитика")),
            ])
        }
    }

    let mut current_path = Route::Home.to_path();
    if let Some(local) = use_location() {
        current_path = local.path().to_string();
    }

    let node_ref = use_node_ref();
    let onclick = {
        let current_visible = *modal_visible;
        let cloned_modal_visible = modal_visible.clone();
        let f = Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            cloned_modal_visible.set(!current_visible);
        });

        use_outside_click(node_ref.clone(), f.clone(), current_visible);

        f
    };

    let onclick_mobile_menu = {
        let cloned_mobile_visible_menu = mobile_visible_menu.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            cloned_mobile_visible_menu.set(!*cloned_mobile_visible_menu);
        })
    };

    let navigator = use_navigator();
    let logout = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut header_bearer = String::from("Bearer ");
                let token: Option<String> = LocalStorage::get("token").unwrap_or(None);
                if let Some(t) = token.clone() {
                    header_bearer.push_str(&t);
                }

                http::Request::post("/api/logout")
                    .header("Content-Type", "application/json")
                    .header("Authorization", &header_bearer)
                    .send()
                    .await
                    .unwrap();

                LocalStorage::delete("token");
                if let Some(navigator) = navigator {
                    navigator.push(&Route::Auth);
                }
            });
        })
    };

    let edit_passwd_modal_toggle = {
        let cloned_edit_passwd_visible = edit_passwd_visible.clone();
        let cloned_modal_visible = modal_visible.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            cloned_modal_visible.set(false);
            cloned_edit_passwd_visible.set(!*cloned_edit_passwd_visible);
        })
    };

    let on_save = {
        // todo
        let cloned_edit_passwd_visible = edit_passwd_visible.clone();
        let cloned_current_user = current_user.clone();
        Callback::from(move |(passwd1, passwd2)| {
            // e.prevent_default();

            let mut header_bearer = String::from("Bearer ");
            let token: Option<String> = LocalStorage::get("token").unwrap_or(None);
            if let Some(t) = token.clone() {
                header_bearer.push_str(&t);
            }

            let cloned_edit_passwd_visible = cloned_edit_passwd_visible.clone();
            let cloned_current_user = cloned_current_user.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let req_data = RequestData { passwd1, passwd2 };

                if let Some(current_user) = cloned_current_user {
                    let path = "/api/users";
                    let _: ResponseMsg =
                        http::Request::patch(&format!("{}/{}/passwd", path, current_user.id))
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

                    cloned_edit_passwd_visible.set(!*cloned_edit_passwd_visible);
                }
            });
        })
    };

    html! {
        <nav class="bg-gray-800">
            <div class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                <div class="relative flex h-16 items-center justify-between">
                    <div class="absolute inset-y-0 left-0 flex items-center sm:hidden">
                        // Icon when menu is closed.
                        <button
                            onclick={onclick_mobile_menu}
                            type="button"
                            class="relative inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white"
                            aria-controls="mobile-menu"
                            aria-expanded="false"
                        >
                            <span class="absolute -inset-0.5"></span>
                            <span class="sr-only">{"Меню"}</span>
                            // Icon when menu is closed.
                            // Menu open: "hidden", Menu closed: "block"
                            <svg
                                class="block h-6 w-6"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                aria-hidden="true"
                            >
                                <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                            </svg>
                            // Icon when menu is open.
                            // Menu open: "block", Menu closed: "hidden"
                            <svg class="hidden h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>
                    <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
                        <div style="min-width: 40px;" class="flex flex-shrink-0 items-center">
                            <Link<Route> to={Route::Home}>
                                <img class="h-8 w-auto" src="./assets/img/logo.png" alt="PAS"/>
                            </Link<Route>>
                        </div>
                        <div class="hidden sm:ml-6 sm:block">
                            <div class="flex space-x-4">
                            // Current: "bg-gray-900 text-white", Default: "text-gray-300 hover:bg-gray-700 hover:text-white"
                            {for menus.iter().map(|(path ,label)| {
                                let path = path.clone();

                                let mut classes = "text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium";
                                if path.to_path() == current_path {
                                    classes = "bg-gray-900 text-white rounded-md px-3 py-2 text-sm font-medium";
                                }
                                html! {
                                    <Link<Route> to={path} classes={classes}>{ label }</Link<Route>>
                                }
                            })}
                            </div>
                        </div>
                    </div>
                    <div class="absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0">
                    // <button type="button" class="relative rounded-full bg-gray-800 p-1 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800">
                    //     <span class="absolute -inset-1.5"></span>
                    //     <span class="sr-only">{"View notifications"}</span>
                    //     <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                    //     <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0" />
                    //     </svg>
                    // </button>
                        <div ref={node_ref} class="relative ml-3">
                            <button
                                {onclick}
                                type="button"
                                class="text-gray-300 relative flex rounded-full bg-gray-800 text-sm focus:outline-none focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800"
                                id="user-menu-button"
                            >
                                {current_user.as_ref().map(|u| u.fio.clone())}
                            </button>
                            <div class={format!("absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none {}", if !*modal_visible {"hidden"} else {""} )}
                                role="menu"
                                aria-orientation="vertical"
                                aria-labelledby="user-menu-button"
                                tabindex="-1">
                                    <a
                                        onclick={edit_passwd_modal_toggle.clone()}
                                        href="#"
                                        class="block px-4 py-2 text-sm text-gray-700"
                                        role="menuitem"
                                        tabindex="-1"
                                        id="user-menu-item-2">
                                        {"Изменить пароль"}
                                    </a>
                                    <a
                                        onclick={logout}
                                        href="#"
                                        class="block px-4 py-2 text-sm text-gray-700"
                                        role="menuitem"
                                        tabindex="-1"
                                        id="user-menu-item-2">
                                        {"Выйти"}
                                    </a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            // Mobile menu, show/hide based on menu state.
            <div class={classes!("sm:hidden", if !*mobile_visible_menu {"hidden"} else {""})} id="mobile-menu">
                <div class="space-y-1 px-2 pb-3 pt-2">
                    {for menus.iter().map(|(path ,label)| {
                        let path = path.clone();

                        let mut classes = "text-gray-300 hover:bg-gray-700 hover:text-white block rounded-md px-3 py-2 text-base font-medium";
                        if path.to_path() == current_path {
                            classes = "bg-gray-900 text-white block rounded-md px-3 py-2 text-base font-medium";
                        }
                        html! {
                            <Link<Route> to={path} classes={classes}>{ label }</Link<Route>>
                        }
                    })}
                </div>
            </div>
            <Modal
               is_visible={*edit_passwd_visible}
               toggle={edit_passwd_modal_toggle}
               {on_save}
            />
        </nav>
    }
}
