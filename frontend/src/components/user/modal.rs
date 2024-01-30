use crate::{check_is_admin, ResponseItems, Role, Select, User};
use gloo::{
    net::http,
    storage::{LocalStorage, Storage},
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

// Список параметров для on_save
type OnSaveParams = (String, String, Role, Option<bool>, Option<i64>);

#[derive(Properties, PartialEq, Default)]
pub struct Props {
    pub current_user: Option<User>,
    pub is_visible: bool,
    pub item: Option<User>,

    pub toggle_modal: Callback<MouseEvent>,
    pub on_save: Callback<OnSaveParams>,
}

#[function_component(Modal)]
pub fn modal(
    Props {
        current_user,
        is_visible,
        item,
        toggle_modal,
        on_save,
    }: &Props,
) -> Html {
    // Заполнение данными

    let mut roles = vec![(Role::User, "Пользователь")];
    if let Some(u) = current_user.clone() {
        if check_is_admin(u.role) {
            if u.role == Role::Developer {
                roles.extend([(Role::Director, "Директор"), (Role::Admin, "Администратор")])
            } else {
                roles.extend([(Role::Director, "Директор")])
            }
        }
    }

    let email = use_state_eq(|| "".to_string());
    let fio = use_state_eq(|| "".to_string());
    let role = use_state_eq(|| Role::User);
    let blocked = use_state_eq(|| false);

    let organization_id = use_state_eq(|| None);
    let organizations: UseStateHandle<Vec<Select>> = use_state(Vec::new);

    {
        let cloned_item = item.clone();
        let cloned_email = email.clone();
        let cloned_fio = fio.clone();
        let cloned_role = role.clone();
        let cloned_blocked = blocked.clone();
        let cloned_organizations = organizations.clone();
        let cloned_organization_id = organization_id.clone();
        use_effect_with(
            (
                *is_visible,
                current_user
                    .as_ref()
                    .map_or(false, |u| check_is_admin(u.role)),
            ),
            move |(visible, is_admin)| {
                if *visible {
                    // Если Admin то получим организации

                    if *is_admin {
                        wasm_bindgen_futures::spawn_local(async move {
                            let mut header_bearer = String::from("Bearer ");
                            let token: Option<String> = LocalStorage::get("token").unwrap_or(None);
                            if let Some(t) = token.clone() {
                                header_bearer.push_str(&t);
                            }

                            let response =
                                http::Request::get("/api/organizations") // todo helpers
                                    .header("Content-Type", "application/json")
                                    .header("Authorization", &header_bearer)
                                    .query([("page", "1"), ("per_page", "10000")])
                                    .send()
                                    .await
                                    .unwrap()
                                    .json::<ResponseItems<Select>>()
                                    .await
                                    .unwrap();

                            cloned_organizations.set(response.items.clone());

                            if let Some(item) = cloned_item.clone() {
                                cloned_email.set(item.email);
                                cloned_fio.set(item.fio);
                                cloned_role.set(item.role);
                                cloned_blocked.set(item.blocked);
                                cloned_organization_id.set(item.organization.map(|o| o.id));
                            } else {
                                cloned_role.set(Role::User);
                                cloned_email.set("".to_string());
                                cloned_fio.set("".to_string());
                                cloned_blocked.set(false);

                                let o_id = response.items.last().map_or(0, |it| it.id);
                                cloned_organization_id.set(Some(o_id));

                                // Только, если не админ
                                if *cloned_role != Role::Admin {
                                    let o_id = response.items.last().map_or(0, |it| it.id);
                                    cloned_organization_id.set(Some(o_id));
                                } else {
                                    cloned_organization_id.set(None);
                                }
                            }
                        })
                    } else if let Some(item) = cloned_item.clone() {
                        cloned_email.set(item.email);
                        cloned_fio.set(item.fio);
                        cloned_role.set(item.role);
                        cloned_blocked.set(item.blocked);
                        cloned_organization_id.set(item.organization.map(|o| o.id));
                    } else {
                        cloned_email.set("".to_string());
                        cloned_fio.set("".to_string());
                        cloned_role.set(Role::default());
                        cloned_blocked.set(false);
                        cloned_organization_id.set(None);
                    }
                }
            },
        );
    }

    let onchange_email = {
        let cloned_email = email.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();

            cloned_email.set(value);
        })
    };

    let onchange_fio = {
        let cloned_fio = fio.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();

            cloned_fio.set(value);
        })
    };

    let onchange_role = {
        let cloned_role = role.clone();
        let cloned_o = organization_id.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();

            let value = value.into();
            cloned_role.set(value);
            // Если создаем Admin, то сбросим организацию
            if value == Role::Admin {
                cloned_o.set(None);
            }
        })
    };

    let onchange_blocked = {
        let cloned_blocked = blocked.clone();
        Callback::from(move |event: Event| {
            let value = event.target_unchecked_into::<HtmlInputElement>().checked();

            cloned_blocked.set(value);
        })
    };

    let onchange_organization = {
        let cloned_o = organization_id.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();

            cloned_o.set(value.parse::<i64>().ok());
        })
    };

    let on_save = {
        let cloned_email = email.clone();
        let cloned_fio = fio.clone();
        let cloned_role = role.clone();
        let cloned_blocked = blocked.clone();
        let cloned_organization_id = organization_id.clone();
        let cloned_on_save = on_save.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            cloned_on_save.emit((
                (*cloned_email).clone(),
                (*cloned_fio).clone(),
                *cloned_role,
                Some(*cloned_blocked),
                *cloned_organization_id,
            ));
        })
    };

    html! {
        <div>
            <div
                class={format!("py-12 bg-gray-700 transition duration-150 ease-in-out z-10 absolute top-0 right-0 bottom-0 left-0 {}", if *is_visible {""} else {"hidden"})}
                    id="modal"
                >
                    <div
                        role="alert"
                        class="container mx-auto w-11/12 md:w-2/3 max-w-lg"
                    >
                        <div
                            class="relative py-8 px-5 md:px-10 bg-white shadow-md rounded border border-gray-400"
                        >
                            <h1
                                class="text-gray-800 font-lg font-bold tracking-normal leading-tight mb-4"
                            >
                                {"Создание/Редактирование"}
                            </h1>
                            <form
                                class="group"
                            >
                                <label for="email" class="text-gray-800 text-sm font-bold leading-tight tracking-normal">{"Email"}</label>
                                <input
                                    disabled={item.is_some()}
                                    onchange={onchange_email}
                                    required={true}
                                    pattern=r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
                                    id="email"
                                    type="email"
                                    class="mb-5 mt-2 text-gray-600 focus:outline-none focus:border focus:border-indigo-700 font-normal w-full h-10 flex items-center pl-3 text-sm border-gray-300 rounded border"
                                    placeholder="Введите Email"
                                    value={(*email).clone()}
                                />
                                <label for="fio" class="text-gray-800 text-sm font-bold leading-tight tracking-normal">{"ФИО"}</label>
                                <input
                                    onchange={onchange_fio}
                                    id="fio"
                                    required={true}
                                    pattern=r"[a-zA-Zа-яА-я].{2,}"
                                    class="mb-5 mt-2 text-gray-600 focus:outline-none focus:border focus:border-indigo-700 font-normal w-full h-10 flex items-center pl-3 text-sm border-gray-300 rounded border"
                                    placeholder="Введите ФИО"
                                    value={(*fio).clone()}
                                />
                                <label for="role" class="text-gray-800 text-sm font-bold leading-tight tracking-normal">{"Роль"}</label>
                                <select
                                    disabled={item.as_ref().map_or_else(|| false, |it| current_user.as_ref().map_or(false, |u| u.id == it.id))}
                                    onchange={onchange_role}
                                    id="role"
                                    class="mb-5 mt-2 text-gray-600 focus:outline-none focus:border focus:border-indigo-700 font-normal w-full h-10 flex items-center pl-3 text-sm border-gray-300 rounded border"
                                    placeholder="Выберите роль">
                                    {
                                        roles.iter().map(|(role_enum, name)| {
                                            html! {
                                                <option selected={item.as_ref().map_or(false, |it| &it.role == role_enum) || &(*role).clone() == role_enum} value={role_enum.to_string()}>{name}</option>
                                            }
                                        }).collect::<Html>()
                                    }
                                </select>

                                if current_user.as_ref().map_or(false, |u| check_is_admin(u.role)) && Role::Admin != *role {
                                    <label for="organization" class="text-gray-800 text-sm font-bold leading-tight tracking-normal">{"Организация"}</label>
                                    <select
                                        id="organization"
                                        onchange={onchange_organization}
                                        class="mb-5 mt-2 text-gray-600 focus:outline-none focus:border focus:border-indigo-700 font-normal w-full h-10 flex items-center pl-3 text-sm border-gray-300 rounded border"
                                        placeholder="Выберите организацию">
                                        {
                                            organizations.iter().map(|o| {
                                                html! {
                                                    <option selected={item.as_ref().map_or(false, |it| it.organization.as_ref().map_or(false, |i| i.id == o.id))} value={o.id.to_string()}>{o.name.clone()}</option>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </select>
                                }

                                if item.is_some(){
                                    <div class="mb-5 flex items-center">
                                    <label class="relative inline-flex cursor-pointer items-center">
                                        <input
                                            disabled={item.as_ref().map_or_else(|| false, |it| current_user.as_ref().map_or(false, |u| u.id == it.id))}
                                            onchange={onchange_blocked}
                                            checked={item.as_ref().map(|it| it.blocked).unwrap_or(false)}
                                            id="switch"
                                            type="checkbox"
                                            class="peer sr-only" />
                                        <label for="switch" class="hidden"></label>
                                        <div class="peer h-6 w-11 rounded-full border bg-slate-200 after:absolute after:left-[2px] after:top-0.5 after:h-5 after:w-5 after:rounded-full after:border after:border-gray-300 after:bg-white after:transition-all after:content-[''] peer-checked:bg-slate-800 peer-checked:after:translate-x-full peer-checked:after:border-white peer-focus:ring-green-300"></div>
                                    </label>
                                        <span class="ml-3 text-gray-600 text-sm font-normal text-xs">{"Разблокирован/Заблокирован"}</span>
                                    </div>
                                }
                                <div class="flex items-center justify-center w-full">
                                    <button
                                        onclick={toggle_modal.clone()}
                                        class="focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-400 ml-3 bg-gray-100 transition duration-150 text-gray-600 ease-in-out hover:border-gray-400 hover:bg-gray-300 border rounded px-8 py-2 text-sm mr-5" >
                                        {"Отменить"}
                                    </button>
                                    <button
                                    onclick={on_save}
                                        class="group-invalid:pointer-events-none group-invalid:opacity-30 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition duration-150 ease-in-out hover:bg-blue-700 bg-blue-500 rounded text-white px-8 py-2 text-sm">
                                        {"Сохранить"}
                                    </button>
                                </div>
                                <button
                                    onclick={toggle_modal.clone()}
                                    class="cursor-pointer absolute top-0 right-0 mt-4 mr-5 text-gray-400 hover:text-gray-600 transition duration-150 ease-in-out rounded focus:ring-2 focus:outline-none focus:ring-gray-600"
                                    aria-label="close modal"
                                    role="button">
                                    <svg  xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-x" width="20" height="20" viewBox="0 0 24 24" stroke-width="2.5" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                        <path stroke="none" d="M0 0h24v24H0z" />
                                        <line x1="18" y1="6" x2="6" y2="18" />
                                        <line x1="6" y1="6" x2="18" y2="18" />
                                    </svg>
                                </button>
                            </form>
                        </div>
                    </div>
            </div>
        </div>
    }
}
