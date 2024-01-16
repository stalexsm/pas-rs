use chrono::Datelike;
use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let start_year: i32 = 2024;
    let date_naive = chrono::Utc::now().date_naive().year();

    let mut all_rihgts_reserved = String::new();
    if date_naive > start_year {
        all_rihgts_reserved.push_str(&format!(
            "© {} - {} Все права защищены",
            start_year, date_naive
        ));
    } else {
        all_rihgts_reserved.push_str(&format!("© {} Все права защищены", date_naive));
    }

    html! {
        <div class="sticky top-[100vh] bg-white">
            <footer class="w-full text-gray-900 bg-gray-100 body-font">
                <div class="bg-gray-150">
                    <div class="container px-5 py-4 mx-auto">
                        <p class="text-sm text-gray-700 xl:text-center">{all_rihgts_reserved}</p>
                    </div>
                </div>
            </footer>
        </div>
    }
}
