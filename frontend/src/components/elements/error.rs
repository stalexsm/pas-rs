use gloo::timers::callback::Timeout;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct Props {
    pub is_visible: bool,
    pub toggle: Callback<bool>,

    #[prop_or("Ошибка!".to_string())]
    pub title: String,

    #[prop_or("Произошла ошибка!".to_string())]
    pub detail: String,
}

#[function_component(AlertError)]
pub fn alert_error(props: &Props) -> Html {
    let cloned_toggle = props.toggle.clone();

    // Вызвать timers
    if props.is_visible {
        Timeout::new(5_000, move || cloned_toggle.emit(false)).forget();
    }

    html! {
       <>
       <div class={classes!("relative", if !props.is_visible {"hidden"} else {""})}>
           <div class={classes!("absolute","flex","justify-start", "z-30")}>
               <div class="bg-red-100 border-l-4 border-red-500 text-red-700 p-4 shadow-lg min-w-80" role="alert">
                   <p class="font-bold">{props.title.clone()}</p>
                   <p>{props.detail.clone()}</p>
               </div>
           </div>
       </div>
       </>
    }
}
