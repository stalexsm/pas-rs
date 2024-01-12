use super::Product;
use crate::{components::rbs::measure::MeasureUnit, ResponseItems};
use gloo::storage::{LocalStorage, Storage};
use gloo_net::http;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct Props {
    pub is_visible: bool,
    pub item: Option<Product>,

    pub toggle_modal: Callback<MouseEvent>,
    pub on_save: Callback<(i64, String)>,
}

#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
    // Заполнение данными

    let domain_api = std::env!("DOMAIN_API");

    let name = use_state_eq(|| "".to_string());
    let mu_id = use_state_eq(|| 0);

    let measure_units: UseStateHandle<Vec<MeasureUnit>> = use_state_eq(Vec::new);

    {
        let cloned_measure_units = measure_units.clone();
        let item = props.item.clone();
        let cloned_mu_id = mu_id.clone();
        let cloned_name = name.clone();
        use_effect_with(props.is_visible, move |visible| {
            if *visible {
                wasm_bindgen_futures::spawn_local(async move {
                    let mut header_bearer = String::from("Bearer ");
                    let token: Option<String> = LocalStorage::get("token").unwrap_or(None);
                    if let Some(t) = token.clone() {
                        header_bearer.push_str(&t);
                    }

                    let path = format!("{}/api/measure-units", domain_api);
                    let response =
                        http::Request::get(&path) // todo helpers
                            .header("Content-Type", "application/json")
                            .header("Authorization", &header_bearer)
                            .query([("page", "1"), ("per_page", "10000")])
                            .send()
                            .await
                            .unwrap()
                            .json::<ResponseItems<MeasureUnit>>()
                            .await
                            .unwrap();

                    cloned_measure_units.set(response.items.clone());

                    if let Some(item) = item.clone() {
                        cloned_mu_id.set(item.measure_unit.id);
                        cloned_name.set(item.name);
                    } else {
                        cloned_mu_id.set(response.items.last().map_or(0, |it| it.id));
                        cloned_name.set("".to_string());
                    }
                })
            }
        });
    }

    let cloned_mu_id = mu_id.clone();
    let onchange_measure = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlSelectElement>()
            .value();

        cloned_mu_id.set(value.parse::<i64>().ok().unwrap_or(0));
    });

    let cloned_name = name.clone();
    let onchange_name = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        cloned_name.set(value);
    });

    let on_save = {
        let cloned_mu_id = mu_id.clone();
        let cloned_name = name.clone();
        let cloned_on_save = props.on_save.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            cloned_on_save.emit((*cloned_mu_id, (*cloned_name).clone()));
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
                                <label for="name" class="text-gray-800 text-sm font-bold leading-tight tracking-normal">
                                    {"Наименование"}
                                </label>
                                <input
                                    onchange={onchange_name}
                                    required={true}
                                    type="text"
                                    pattern="^[a-zA-Zа-яА-Я][0-9]*.{2,}$"
                                    id="name"
                                    class="mb-5 mt-2 text-gray-600 focus:outline-none focus:border focus:border-indigo-700 font-normal w-full h-10 flex items-center pl-3 text-sm border-gray-300 rounded border"
                                    placeholder="Введите наименование"
                                    value={(*name).clone()}
                                />
                                <label for="measure" class="text-gray-800 text-sm font-bold leading-tight tracking-normal">{"Продукт"}</label>
                                <select
                                    onchange={onchange_measure}
                                    id="measure"
                                    class="mb-5 mt-2 text-gray-600 focus:outline-none focus:border focus:border-indigo-700 font-normal w-full h-10 flex items-center pl-3 text-sm border-gray-300 rounded border"
                                    placeholder="Выберите единицу измерения">
                                    {
                                        (*measure_units).iter().map(|item| {
                                            html! {
                                                <option selected={item.id == *mu_id} value={item.id.to_string()}>{format!("{}", &item.name)}</option>
                                            }
                                        }).collect::<Html>()
                                    }
                                </select>
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
