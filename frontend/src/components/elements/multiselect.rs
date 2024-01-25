use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::HtmlAnchorElement;
use yew::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub id: i64,
    pub name: String,
}

#[derive(Properties, PartialEq, Default)]
pub struct Props {
    pub onchange: Callback<Vec<String>>,
    pub selected: Vec<String>,
    pub items: Vec<Item>,
}

#[function_component(MultiSelect)]
pub fn multiselect(
    Props {
        onchange,
        selected,
        items,
    }: &Props,
) -> Html {
    // Обработка компонента Multi Select
    let show_opts = use_state(|| false);
    let onclick = {
        let cloned_show_opts = show_opts.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            cloned_show_opts.set(!*cloned_show_opts);
        })
    };

    let selected_opts: UseStateHandle<Vec<String>> = use_state(|| selected.clone());

    {
        let cloned_onchange = onchange.clone();
        use_effect_with((*selected_opts).clone(), move |opts| {
            cloned_onchange.emit(opts.clone());
        });
    }

    html! {
        <div class="flex flex-col sm:min-w-[190px]"> // h-64 w-full md:w-1/2 flex flex-col items-center mx-auto
            <div class="w-full"> // px-4
                <div class="flex flex-col items-center relative">
                    <div class="w-full">
                        <div class="p-1 flex border border-gray-300 bg-white rounded">
                            <div class="flex flex-auto flex-wrap">
                                <div class="
                                    flex
                                    justify-center
                                    items-center
                                    py-1
                                    px-2
                                    bg-white
                                    text-gray-400
                                    text-gray
                                    rounded-md
                                    font-normal
                                    text-sm"
                                >
                                    {format!("Выбрано: {} эл.", (*selected_opts).len())}
                                </div>
                            </div>
                            <div class="text-gray-300 w-8 py-1 pl-2 pr-1 border-l flex items-center border-gray-200">
                            <button {onclick} class="cursor-pointer w-6 h-6 text-gray-600 outline-none focus:outline-none">
                                if *show_opts {
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke-width="1.5"
                                        stroke="currentColor"
                                        class="w-4 h-4">
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            d="m4.5 15.75 7.5-7.5 7.5 7.5"
                                        />
                                    </svg>
                                } else {
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke-width="1.5"
                                        stroke="currentColor"
                                        class="w-4 h-4">
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            d="m19.5 8.25-7.5 7.5-7.5-7.5"
                                        />
                                    </svg>
                                }
                            </button>
                            </div>
                        </div>
                    </div>
                    <div class={
                        classes!(
                            "top-full",
                            "max-h-[300px]",
                            "absolute",
                            "shadow",
                            "top-100",
                            "bg-white",
                            "z-40",
                            "w-full",
                            "lef-0",
                            "rounded",
                            "overflow-y-auto",
                            if *show_opts {""} else {"hidden"}
                        )
                    }>
                        {
                            items.iter().map(|item| {
                                let onclick = {
                                    let cloned_selected_opts = selected_opts.clone();
                                    Callback::from(move |e: MouseEvent| {
                                        e.prevent_default();
                                        let el = e
                                            .target()
                                            .unwrap()
                                            .unchecked_into::<HtmlAnchorElement>();
                                        if el.has_attribute("value") {
                                            if let Some(value) = el.get_attribute("value") {
                                                let mut opts = (*cloned_selected_opts).clone();
                                                let idx = opts
                                                    .clone()
                                                    .iter()
                                                    .position(|x| x.clone() == value);

                                                if let Some(idx) = idx {
                                                    opts.remove(idx);
                                                } else {
                                                    opts.push(value);
                                                }

                                                cloned_selected_opts.set(opts);
                                            }
                                        }
                                    })
                                };

                                let item_string = item.id.to_string();
                                html! {
                                    <div class="flex flex-col w-full">
                                        <div class="cursor-pointer w-full border-gray-100 rounded-t border-b hover:bg-blue-100">
                                            <div class="flex w-full items-center p-2 pl-2 border-transparent border-l-2 relative hover:border-blue-100">
                                                <div class="w-full items-center flex">
                                                    <div {onclick} value={item.id.to_string()} class="text-xs mx-2 leading-6 w-full text-ellipsis truncate">{item.name.clone()}</div>
                                                    if (*selected_opts).contains(&item_string) {
                                                        <span class="mr-5">
                                                            <svg
                                                                xmlns="http://www.w3.org/2000/svg"
                                                                fill="none"
                                                                viewBox="0 0 24 24"
                                                                stroke-width="1.5"
                                                                stroke="currentColor"
                                                                class="w-4 h-4"
                                                            >
                                                                <path
                                                                    stroke-linecap="round"
                                                                    stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5"
                                                                />
                                                            </svg>
                                                        </span>
                                                    }
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        </div>


    }
}
