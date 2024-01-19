use crate::{
    components::{
        analitic::{list::AnaliticList, Analitic},
        footer::Footer,
        header::component::HeaderComponent,
    },
    AppContext, Route, User,
};
use gloo::storage::{LocalStorage, Storage};
use gloo_net::http;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew_router::hooks::{use_location, use_navigator};

#[derive(Debug, Serialize, Deserialize)]
pub struct Q {
    date_one: chrono::NaiveDate,
    date_two: chrono::NaiveDate,

    product: Option<String>,
    user: Option<String>,
}

#[function_component(AnaliticComponent)]
pub fn analitic() -> Html {
    // Компонент домашней страницы

    let ctx = use_context::<AppContext>();
    let current_user: Option<User> = ctx.and_then(|ctx| ctx.0.clone());

    // Для фильтрации по датам.
    let date_naive = chrono::Utc::now().date_naive();
    let location = use_location().unwrap();
    let date_one = use_state_eq(|| {
        location
            .query::<Q>()
            .map(|it| {
                chrono::NaiveDate::parse_from_str(it.date_one.to_string().as_str(), "%Y-%m-%d")
                    .unwrap_or(date_naive - chrono::Duration::days(7))
            })
            .unwrap_or(date_naive - chrono::Duration::days(7))
    });
    let date_two = use_state_eq(|| {
        location
            .query::<Q>()
            .map(|it| {
                chrono::NaiveDate::parse_from_str(it.date_two.to_string().as_str(), "%Y-%m-%d")
                    .unwrap_or(date_naive)
            })
            .unwrap_or(date_naive)
    });

    let product = use_state_eq(|| location.query::<Q>().map(|it| it.product).unwrap_or(None));
    let user = use_state_eq(|| location.query::<Q>().map(|it| it.user).unwrap_or(None));

    let items: UseStateHandle<Vec<Analitic>> = use_state_eq(Vec::new);
    {
        let items = items.clone();
        let navigator = use_navigator();
        use_effect_with(
            (*date_one, *date_two, (*product).clone(), (*user).clone()),
            move |(date_one, date_two, product, user)| {
                let items = items.clone();
                let cloned_date_one = *date_one;
                let cloned_date_two = *date_two;
                let cloned_product = (*product).clone();
                let cloned_user = (*user).clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let mut header_bearer = String::from("Bearer ");
                    let token: Option<String> = LocalStorage::get("token").unwrap_or(None);
                    if let Some(t) = token.clone() {
                        header_bearer.push_str(&t);
                    }

                    let _1 = cloned_date_one.to_string();
                    let _2 = cloned_date_two.to_string();
                    let mut q = vec![("date_one", _1.as_str()), ("date_two", _2.as_str())];

                    if let Some(product) = &cloned_product {
                        q.push(("product", &product));
                    }

                    if let Some(user) = &cloned_user {
                        q.push(("user", user));
                    }

                    let response = http::Request::get("/api/analitics")
                        .header("Content-Type", "application/json")
                        .header("Authorization", &header_bearer)
                        .query(q)
                        .send()
                        .await
                        .unwrap()
                        .json::<Vec<Analitic>>()
                        .await
                        .unwrap();

                    items.set(response);

                    if let Some(navigator) = navigator {
                        navigator
                            .push_with_query(
                                &Route::Analitic,
                                &Q {
                                    date_one: cloned_date_one,
                                    date_two: cloned_date_two,

                                    product: cloned_product,
                                    user: cloned_user,
                                },
                            )
                            .unwrap();
                    }
                });
            },
        );
    }

    let onchange_date_one = {
        let cloned_date_one = date_one.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();

            cloned_date_one
                .set(chrono::NaiveDate::parse_from_str(value.as_str(), "%Y-%m-%d").unwrap());
        })
    };

    let onchange_date_two = {
        let cloned_date_two = date_two.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();

            cloned_date_two
                .set(chrono::NaiveDate::parse_from_str(value.as_str(), "%Y-%m-%d").unwrap());
        })
    };

    let onchange_product = {
        let cloned_product = product.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();

            cloned_product.set(Some(value));
        })
    };

    let onchange_user = {
        let cloned_user = user.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();

            cloned_user.set(Some(value));
        })
    };

    let download_report = {
        let cloned_date_one = *date_one;
        let cloned_date_two = *date_two;
        let cloned_product = (*product).clone();
        let cloned_user = (*user).clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            let cloned_product = cloned_product.clone();
            let cloned_user = cloned_user.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut header_bearer = String::from("Bearer ");
                let token: Option<String> = LocalStorage::get("token").unwrap_or(None);
                if let Some(t) = token.clone() {
                    header_bearer.push_str(&t);
                }

                let _1 = cloned_date_one.to_string();
                let _2 = cloned_date_two.to_string();
                let mut q = vec![("date_one", _1.as_str()), ("date_two", _2.as_str())];

                if let Some(product) = &cloned_product {
                    q.push(("product", &product));
                }

                if let Some(user) = &cloned_user {
                    q.push(("user", user));
                }

                let resp = http::Request::post("/api/upload-report")
                    .header("Content-Type", "application/json")
                    .header("Authorization", &header_bearer)
                    .query(q)
                    .send()
                    .await
                    .unwrap();

                let headers = resp.headers();
                if let Some(disposition) = headers.get("Content-Disposition") {
                    // Ищем имя файла в Content-Disposition.
                    if let Some(filename) = disposition
                        .split(';')
                        .find(|part| part.trim_start().starts_with("filename="))
                    {
                        // Обычно, имя файла начинается после "filename=" и может быть в кавычках.
                        let filename = filename
                            .split('=')
                            .nth(1) // Получаем значение после '='.
                            .map(|s| s.trim()) // Удаляем пробелы по краям.
                            .map(|s| s.trim_matches('"')) // Удаляем кавычки, если они есть.
                            .unwrap_or("");

                        let bytes = resp.binary().await.unwrap();

                        let u8_array = js_sys::Uint8Array::from(&bytes[..]);
                        let array = js_sys::Array::new_with_length(1);
                        array.set(0, u8_array.into());

                        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
                            &array,
                            web_sys::BlobPropertyBag::new().type_("application/octet-stream"),
                        )
                        .unwrap();
                        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                        let document = web_sys::window().unwrap().document().unwrap();

                        let a = document.create_element("a").unwrap();
                        let a = a.dyn_into::<web_sys::HtmlAnchorElement>().unwrap();

                        // Устанавливаем href для элемента <a> как созданную URL для `Blob`
                        a.set_href(&url);
                        // Задаем атрибут `download` с желаемым именем файла
                        a.set_download(filename); // Или другой желаемый формат и имя файла
                                                  // Программно "кликаем" по элементу <a>, чтобы начать скачивание
                        a.click();
                        // Удаляем созданную URL, чтобы освободить ресурсы
                        web_sys::Url::revoke_object_url(&url).unwrap();
                    }
                }
            });
        })
    };

    html! {
        <>
        <HeaderComponent />
        <div class="flex justify-end">
            <button
                onclick={download_report}
                class="
                    min-w-[151px]
                    px-4
                    py-2
                    bg-blue-500
                    text-white
                    rounded-md
                    hover:bg-blue-700
                    mt-2
                    mx-5
                "
            >
                {"Скачать отчет"}
            </button>
        </div>
        <div
            class="
                xs:grid
                xs:gap-x-[15px]
                xs:gap-y-[5px]
                xs:grid-cols-2
                sm:flex
                sm:justify-end
                my-2
                mx-5
            "
        >
            <input
                type="text"
                onchange={onchange_product}
                class="
                    w-[calc((100vw - 2.5rem - 15px) / 2)]
                    px-4
                    py-2
                    text-gray-600
                    text-gray
                    rounded-md
                    font-normal
                    text-sm
                    rounded
                    border
                    border-gray-300
                    focus:border-gray-700
                    focus:border-indigo-700
                    focus:border
                    focus:outline-none
                "
                placeholder="Фильтр по товару"
                value={(*product).clone()}
            />
            <input
                type="text"
                onchange={onchange_user}
                class="
                    w-[calc((100vw - 2.5rem - 15px) / 2)]
                    px-4
                    py-2
                    text-gray-600
                    text-gray
                    rounded-md
                    font-normal
                    text-sm
                    rounded
                    border
                    border-gray-300
                    focus:border-gray-700
                    focus:border-indigo-700
                    focus:border
                    focus:outline-none
                "
                placeholder="Фильтр по пользователю"
                value={(*user).clone()}
            />

            <input
                type="date"
                onchange={onchange_date_one}
                class="
                    w-[calc((100vw - 2.5rem - 15px) / 2)]
                    px-4
                    py-2
                    text-gray-600
                    text-gray
                    rounded-md
                    font-normal
                    text-sm
                    rounded
                    border
                    border-gray-300
                    focus:border-gray-700
                    focus:border-indigo-700
                    focus:border
                    focus:outline-none
                "
                value={date_one.to_string()}

            />
            <input
                type="date"
                onchange={onchange_date_two}
                class="
                    w-[calc((100vw - 2.5rem - 15px) / 2)]
                    px-4
                    py-2
                    text-gray-600
                    text-gray
                    rounded-md
                    font-normal
                    text-sm
                    rounded
                    border
                    border-gray-300
                    focus:border-gray-700
                    focus:border-indigo-700
                    focus:border
                    focus:outline-none
                "
                value={date_two.to_string()}
            />
        </div>
        <div class="overflow-auto rounded-lg border border-gray-200 shadow-md mx-5 my-2 max-h-[68%]">
            <table class="w-full border-collapse bg-white text-left text-sm text-gray-500 table-auto">
                <thead class="bg-gray-50 sticky top-0">
                    <tr>
                    <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"#"}</th>
                    <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"Продукт"}</th>
                    <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"Eдиница Измерения"}</th>
                    <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase">{"Кол-во"}</th>
                    // <th scope="col" class="px-6 py-4 font-medium text-gray-900 uppercase"></th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-gray-100 border-t border-gray-100">
                    <AnaliticList
                        items={(*items).clone()}
                        current_user={current_user}
                    />
                </tbody>
            </table>
        </div>

        <Footer />

        </>
    }
}
