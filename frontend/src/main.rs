use frontend::{
    components::{
        analitic::component::AnaliticComponent,
        auth::AuthComponent,
        elements::loader::Loader,
        home::component::HomeComponent,
        not_found::NotFound,
        rbs::{measure::component::MeasureUnitComponent, product::component::ProductComponent},
        user::component::UserComponent,
    },
    AppContext, AppStateContext, Route, User,
};
use gloo::{
    net::http,
    storage::{LocalStorage, Storage},
};
use log::debug;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // Application
    let ctx = use_reducer(|| AppStateContext(None));
    // Флаг, чтобы дождаться получения пользователя по токену
    let loading = use_state(|| true);

    // Получение пользоватедя по токену, если есть...
    {
        let ctx = ctx.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            let ctx = ctx.clone();
            let loading = loading.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut header_bearer = String::from("Bearer ");
                let token: Option<String> = LocalStorage::get("token").unwrap_or(None);
                if let Some(t) = token.clone() {
                    header_bearer.push_str(&t);

                    match http::Request::get("/api/current")
                        .header("Content-Type", "application/json")
                        .header("Authorization", &header_bearer)
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if response.ok() {
                                let response_user = response.json::<User>().await.unwrap();
                                ctx.dispatch(Some(response_user));
                            } else {
                                LocalStorage::delete("token");
                            }
                        }
                        Err(_) => {
                            LocalStorage::delete("token");
                        }
                    }
                }
                loading.set(false);
            });
        });
    }

    if *loading {
        html! {
           <Loader />
        }
    } else {
        html! {
            <ContextProvider<AppContext> context={ctx}>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </ContextProvider<AppContext>>
        }
    }
}

fn switch(route: Route) -> Html {
    let token: Option<String> = LocalStorage::get("token").unwrap_or(None);
    debug!("Token: {:?} in switch!", token);

    match token {
        Some(_token) => match route {
            Route::Auth => html! { <Redirect<Route> to={Route::Home} />},
            Route::Home => html! { <HomeComponent />},
            Route::Product => html! { <ProductComponent />},
            Route::MeasureUnit => html! { <MeasureUnitComponent />},
            Route::User => html! {<UserComponent /> },
            Route::Analitic => html! {<AnaliticComponent /> },
            Route::NotFound => html! {<NotFound /> },
        },
        None => match route {
            Route::Auth => html! { <AuthComponent /> },
            _ => html! { <Redirect<Route> to={Route::Auth} /> },
        },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
