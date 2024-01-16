use crate::{Role, User};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct Props {
    pub current_user: Option<User>,
    pub is_visible: bool,
    pub item: Option<User>,

    pub toggle_modal: Callback<MouseEvent>,
    pub on_save: Callback<(String, Option<String>, Role, Option<bool>)>,
}

#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
    // Заполнение данными

    let email = use_state_eq(|| "".to_string());
    let fio = use_state_eq(|| None);
    let role = use_state_eq(Role::default);
    let blocked = use_state_eq(|| false);

    {
        let cloned_item = props.item.clone();
        let cloned_email = email.clone();
        let cloned_fio = fio.clone();
        let cloned_role = role.clone();
        let cloned_blocked = blocked.clone();
        use_effect_with(props.is_visible, move |visible| {
            if *visible {
                if let Some(item) = cloned_item.clone() {
                    cloned_email.set(item.email);
                    cloned_fio.set(item.fio);
                    cloned_role.set(item.role);
                    cloned_blocked.set(item.blocked);
                } else {
                    cloned_email.set("".to_string());
                    cloned_fio.set(Some("".to_string()));
                    cloned_role.set(Role::User);
                    cloned_blocked.set(false);
                }
            }
        });
    }

    let cloned_email = email.clone();
    let onchange_email = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlSelectElement>()
            .value();

        cloned_email.set(value);
    });

    let cloned_fio = fio.clone();
    let onchange_fio = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlSelectElement>()
            .value();

        cloned_fio.set(Some(value));
    });

    let cloned_role = role.clone();
    let onchange_role = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlSelectElement>()
            .value();

        cloned_role.set(Role::from(value));
    });

    let cloned_blocked = blocked.clone();
    let onchange_blocked = Callback::from(move |event: Event| {
        let value = event.target_unchecked_into::<HtmlInputElement>().checked();

        cloned_blocked.set(value);
    });

    let on_save = {
        let cloned_email = email.clone();
        let cloned_fio = fio.clone();
        let cloned_role = role.clone();
        let cloned_blocked = blocked.clone();
        let cloned_on_save = props.on_save.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            cloned_on_save.emit((
                (*cloned_email).clone(),
                (*cloned_fio).clone(),
                *cloned_role,
                Some(*cloned_blocked),
            ));
        })
    };

    html! {
        <div>
            <div
                class={format!("py-12 bg-gray-700 transition duration-150 ease-in-out z-10 absolute top-0 right-0 bottom-0 left-0 {}", if props.is_visible {""} else {"hidden"})}
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
                                    disabled={props.item.is_some()}
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
                                    disabled={props.item.as_ref().map_or_else(|| false, |it| props.current_user.as_ref().map_or(false, |u| u.id == it.id))}
                                    onchange={onchange_role}
                                    id="role"
                                    class="mb-5 mt-2 text-gray-600 focus:outline-none focus:border focus:border-indigo-700 font-normal w-full h-10 flex items-center pl-3 text-sm border-gray-300 rounded border"
                                    placeholder="Выберите Ед. измерения">
                                    {[(Role::Admin, "Администратор"), (Role::User, "Пользователь")].iter().map(|(role, name)| {
                                            html! {
                                                <option selected={props.item.as_ref().map_or_else(|| false, |it| &it.role == role)} value={role.to_string()}>{name}</option>
                                            }
                                        }).collect::<Html>()
                                    }
                                </select>
                                if props.item.is_some(){
                                    <div class="mb-5 flex items-center">
                                    <label class="relative inline-flex cursor-pointer items-center">
                                        <input
                                            disabled={props.item.as_ref().map_or_else(|| false, |it| props.current_user.as_ref().map_or(false, |u| u.id == it.id))}
                                            onchange={onchange_blocked}
                                            checked={props.item.as_ref().map(|it| it.blocked).unwrap_or(false)}
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
                                        onclick={props.toggle_modal.clone()}
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
                                    onclick={props.toggle_modal.clone()}
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
