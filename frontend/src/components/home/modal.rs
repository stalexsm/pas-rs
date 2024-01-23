use super::ProducedGood;
use crate::{components::rbs::product::Product, ResponseItems};
use gloo::{
    net::http,
    storage::{LocalStorage, Storage},
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct Props {
    pub is_visible: bool,
    pub is_adj: bool,
    pub item: Option<ProducedGood>,

    pub toggle_modal: Callback<MouseEvent>,
    pub on_save: Callback<(i64, i64)>,
    pub on_save_adj: Callback<i64>,
}

#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
    // Заполнение данными

    let product_id = use_state_eq(|| 0);
    let cnt = use_state_eq(|| 0);
    let adj = use_state_eq(|| 0);

    let products: UseStateHandle<Vec<Product>> = use_state_eq(Vec::new);

    {
        let cloned_products = products.clone();
        let item = props.item.clone();
        let cloned_product_id = product_id.clone();
        let cloned_cnt = cnt.clone();
        use_effect_with(props.is_visible, move |visible| {
            if *visible {
                wasm_bindgen_futures::spawn_local(async move {
                    let mut header_bearer = String::from("Bearer ");
                    let token: Option<String> = LocalStorage::get("token").unwrap_or(None);
                    if let Some(t) = token.clone() {
                        header_bearer.push_str(&t);
                    }

                    let response =
                        http::Request::get("/api/products") // todo helpers
                            .header("Content-Type", "application/json")
                            .header("Authorization", &header_bearer)
                            .query([("page", "1"), ("per_page", "10000")])
                            .send()
                            .await
                            .unwrap()
                            .json::<ResponseItems<Product>>()
                            .await
                            .unwrap();

                    cloned_products.set(response.items.clone());

                    if let Some(item) = item.clone() {
                        cloned_product_id.set(item.product.id);
                        cloned_cnt.set(item.cnt);
                    } else {
                        cloned_product_id.set(response.items.last().map_or(0, |it| it.id));
                        cloned_cnt.set(0);
                    }
                })
            }
        });
    }

    let cloned_product_id = product_id.clone();
    let onchange_product = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlSelectElement>()
            .value();

        cloned_product_id.set(value.parse::<i64>().ok().unwrap_or(0));
    });

    let cloned_cnt = cnt.clone();
    let onchange_cnt = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        cloned_cnt.set(value.parse::<i64>().ok().unwrap_or(0));
    });

    let cloned_adj = adj.clone();
    let onchange_adj = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        cloned_adj.set(value.parse::<i64>().ok().unwrap_or(0));
    });

    let on_save = {
        let cloned_product_id = product_id.clone();
        let cloned_cnt = cnt.clone();
        let cloned_adj = adj.clone();
        let cloned_on_save = props.on_save.clone();
        let cloned_on_save_adj = props.on_save_adj.clone();
        let cloned_is_adj = props.is_adj;
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            if cloned_is_adj {
                cloned_on_save_adj.emit(*cloned_adj);
            } else {
                cloned_on_save.emit((*cloned_product_id, *cloned_cnt));
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
                                {"Создание/Редактирование"}
                            </h1>
                            <form
                                class="group"
                            >
                                <label for="product" class="text-gray-800 text-sm font-bold leading-tight tracking-normal">{"Продукт"}</label>
                                <select
                                    disabled={props.is_adj}
                                    onchange={onchange_product}
                                    id="product"
                                    class="mb-5 mt-2 text-gray-600 focus:outline-none focus:border focus:border-indigo-700 font-normal w-full h-10 flex items-center pl-3 text-sm border-gray-300 rounded border"
                                    placeholder="Выберите продукт">
                                    {
                                        (*products).iter().map(|item| {
                                            html! {
                                                // <option selected={props.item.as_ref().map_or(false, |it| it.product.id == item.id)} value={item.id.to_string()}>{format!("{} ({})", &item.name, &item.measure_unit.name.clone())}</option>
                                                <option selected={item.id == *product_id} value={item.id.to_string()}>{format!("{} ({})", &item.name, &item.measure_unit.name.clone())}</option>
                                            }
                                        }).collect::<Html>()
                                    }
                                </select>
                                <label for="cnt" class="text-gray-800 text-sm font-bold leading-tight tracking-normal">
                                    {"Кол-во"}
                                </label>
                                <input
                                    disabled={props.is_adj}
                                    onchange={onchange_cnt}
                                    required={true}
                                    type="number"
                                    min=1
                                    id="cnt"
                                    class="mb-2 mt-2 text-gray-600 focus:outline-none focus:border focus:border-indigo-700 font-normal w-full h-10 flex items-center pl-3 text-sm border-gray-300 rounded border"
                                    placeholder="Введите кол-во"
                                    value={
                                        let cnt = *cnt;
                                        if cnt > 0 {
                                            cnt.to_string()
                                        } else {
                                            "".to_string()
                                        }
                                    }
                                />
                                if let Some(it) = props.item.clone() {
                                    <p
                                        class="flex items-center gap-1 mb-5 font-sans text-sm antialiased font-normal leading-normal text-gray-700"
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-4 h-4 -mt-px">
                                            <path
                                                fill-rule="evenodd"
                                                d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm8.706-1.442c1.146-.573 2.437.463 2.126 1.706l-.709 2.836.042-.02a.75.75 0 01.67 1.34l-.04.022c-1.147.573-2.438-.463-2.127-1.706l.71-2.836-.042.02a.75.75 0 11-.671-1.34l.041-.022zM12 9a.75.75 0 100-1.5.75.75 0 000 1.5z"
                                                clip-rule="evenodd">
                                            </path>
                                        </svg>
                                        {"Корректировка: "}
                                        <span
                                            class={format!("font-medium rounded-full px-2 py-1 {}", if it.adj >= 0 {"text-green-500 bg-green-50"} else {"text-red-500 bg-red-50"})}
                                        >
                                            {it.adj}
                                        </span>
                                    </p>
                                }

                                if props.is_adj {
                                    <label for="adj" class="text-gray-800 text-sm font-bold leading-tight tracking-normal">{"Корректировка"}</label>
                                    <input
                                        onchange={onchange_adj}
                                        min={props.item.as_ref().map(|it| (-(it.cnt + it.adj)).to_string())}
                                        required={true}
                                        type="number"
                                        id="adj"
                                        class="mb-5 mt-2 text-gray-600 focus:outline-none focus:border focus:border-indigo-700 font-normal w-full h-10 flex items-center pl-3 text-sm border-gray-300 rounded border"
                                        placeholder="Введите корректировку"
                                    />
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
