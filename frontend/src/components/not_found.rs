use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <main class="grid min-h-full place-items-center bg-white px-6 py-24 sm:py-32 lg:px-8">
            <div class="text-center">
            <p class="text-base font-semibold text-blue-500">{"404"}</p>
            <h1 class="mt-4 text-3xl font-bold tracking-tight text-gray-900 sm:text-5xl">{"Страница не найдена"}</h1>
            <p class="mt-6 text-base leading-7 text-gray-600">{"Извините, мы не смогли найти страницу, которую вы ищете."}</p>
            <div class="mt-10 flex items-center justify-center gap-x-6">
                <Link<Route> to={Route::Home} classes={"rounded-md bg-blue-500 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-blue-700 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"}>{ "Возвращайся домой" }</Link<Route>>
                // <a href="#" class="text-sm font-semibold text-gray-900">Contact support <span aria-hidden="true">&rarr;</span></a>
            </div>
            </div>
        </main>
    }
}
