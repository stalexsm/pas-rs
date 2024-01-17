use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct Props {
    pub is_visible: bool,

    pub toggle: Callback<MouseEvent>,
    pub on_save: Callback<(String, String)>,
}

#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
    // Заполнение данными

    let passwd1 = use_state_eq(String::new);
    let passwd2 = use_state_eq(String::new);
    let matched_passwd = use_state_eq(|| false);

    let show_passwd1 = use_state_eq(|| false);
    let show_passwd2 = use_state_eq(|| false);

    let onclick_passwd1 = {
        let cloned_show_passwd1 = show_passwd1.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            cloned_show_passwd1.set(!*cloned_show_passwd1);
        })
    };

    let onclick_passwd2 = {
        let cloned_show_passwd2 = show_passwd2.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            cloned_show_passwd2.set(!*cloned_show_passwd2);
        })
    };

    let onchange_passwd1 = {
        let cloned_passwd1 = passwd1.clone();
        let cloned_passwd2 = passwd2.clone();
        let cloned_matched_passwd = matched_passwd.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();

            cloned_passwd1.set(value.clone());
            cloned_matched_passwd.set(value == *cloned_passwd2);
        })
    };

    let onchange_passwd2 = {
        let cloned_passwd1 = passwd1.clone();
        let cloned_passwd2 = passwd2.clone();
        let cloned_matched_passwd = matched_passwd.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();

            cloned_passwd2.set(value.clone());
            cloned_matched_passwd.set(value == *cloned_passwd1);
        })
    };

    let on_save = {
        let cloned_passwd1 = passwd1.clone();
        let cloned_passwd2 = passwd2.clone();
        let cloned_on_save = props.on_save.clone();
        let cloned_matched_passwd = matched_passwd.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            if *cloned_matched_passwd {
                cloned_on_save.emit(((*cloned_passwd1).clone(), (*cloned_passwd2).clone()));
            }
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
                                {"Изменение пароля"}
                            </h1>
                            <form
                                class="group"
                            >
                                <label for="passwd1" class="text-gray-800 text-sm font-bold leading-tight tracking-normal">
                                    {"Пароль"}
                                </label>
                                <div class="relative">
                                    <input
                                        onchange={onchange_passwd1}
                                        required={true}
                                        type={if *show_passwd1 {"text"} else {"password"}}
                                        id="passwd1"
                                        pattern={"[0-9a-zA-Z@!&^_.]{8,}"}
                                        class={classes!(
                                            "mb-5",
                                            "mt-2",
                                            "text-gray-600",
                                            "focus:outline-none",
                                            "focus:border",
                                            "focus:border-indigo-700",
                                            "font-normal",
                                            "w-full",
                                            "h-10",
                                            "flex",
                                            "items-center",
                                            "pl-3",
                                            "text-sm",
                                            "border-gray-300",
                                            "rounded",
                                            "border",
                                            if !(*passwd1).is_empty() && !*matched_passwd {"border-red-500"} else {""}
                                        )}
                                        placeholder="Введите пароль"
                                    />
                                    <span onclick={onclick_passwd1} class="absolute inset-y-0 right-3.5 pl-3 flex items-center mt-auto text-gray-500 cursor-pointer">
                                        if *show_passwd1 {
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
                                <label for="passwd2" class="text-gray-800 text-sm font-bold leading-tight tracking-normal">{"Повторите пароль"}</label>
                                <div class="relative">
                                <input
                                    onchange={onchange_passwd2}
                                    required={true}
                                    type={if *show_passwd2 {"text"} else {"password"}}
                                    id="passwd2"
                                    pattern={"[0-9a-zA-Z@!&^_.]{8,}"}
                                    class={classes!(
                                        "mb-5",
                                        "mt-2",
                                        "text-gray-600",
                                        "focus:outline-none",
                                        "focus:border",
                                        "focus:border-indigo-700",
                                        "font-normal",
                                        "w-full",
                                        "h-10",
                                        "flex",
                                        "items-center",
                                        "pl-3",
                                        "text-sm",
                                        "border-gray-300",
                                        "rounded",
                                        "border",
                                        if !(*passwd2).is_empty() && !*matched_passwd {"border-red-500"} else {""}
                                    )}
                                    placeholder="Введите пароль еще раз"
                                />
                                <span onclick={onclick_passwd2} class="absolute inset-y-0 right-3.5 pl-3 flex items-center mt-auto text-gray-500 cursor-pointer">
                                    if *show_passwd2 {
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
                                <div class="flex items-center justify-center w-full">
                                    <button
                                        onclick={props.toggle.clone()}
                                        class="focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-400 ml-3 bg-gray-100 transition duration-150 text-gray-600 ease-in-out hover:border-gray-400 hover:bg-gray-300 border rounded px-8 py-2 text-sm mr-5" >
                                        {"Отменить"}
                                    </button>
                                    <button
                                        onclick={on_save}
                                        class={classes!(
                                            "group-invalid:pointer-events-none",
                                            "group-invalid:opacity-30",
                                            "focus:outline-none",
                                            "focus:ring-2",
                                            "focus:ring-offset-2",
                                            "focus:ring-blue-500",
                                            "transition",
                                            "duration-150",
                                            "ease-in-out",
                                            "hover:bg-blue-700",
                                            "bg-blue-500",
                                            "rounded",
                                            "text-white",
                                            "px-8",
                                            "py-2",
                                            "text-sm",
                                            if *matched_passwd {""} else {"pointer-events-none, opacity-30"}
                                        )}>
                                        {"Сохранить"}
                                    </button>
                                </div>
                                <button
                                    onclick={props.toggle.clone()}
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
