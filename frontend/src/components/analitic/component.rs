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

    html! {
        <>
        <HeaderComponent />
        <div class="xs:grid xs:justify-items-end sm:flex sm:justify-end mb-5 min-w-96">
            <input
                type="text"
                onchange={onchange_product}
                class="
                    min-w-[188px]
                    px-4
                    py-2
                    text-gray-600
                    text-gray
                    rounded-md
                    mr-5
                    mt-5
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
                    min-w-[188px]
                    px-4
                    py-2
                    text-gray-600
                    text-gray
                    rounded-md
                    mr-5
                    mt-5
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
                    min-w-[188px]
                    px-4
                    py-2
                    text-gray-600
                    text-gray
                    rounded-md
                    mr-5
                    mt-5
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
                    min-w-[188px]
                    px-4
                    py-2
                    text-gray-600
                    text-gray
                    rounded-md
                    mr-5
                    mt-5
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
        <div class="overflow-auto rounded-lg border border-gray-200 shadow-md m-5 max-h-[68%]">
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
