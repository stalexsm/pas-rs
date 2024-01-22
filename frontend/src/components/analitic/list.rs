use crate::User;
use yew::prelude::*;

use super::Analitic;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub items: Vec<Analitic>,
    pub current_user: Option<User>,
}

#[function_component(AnaliticList)]
pub fn produced_good_lists(
    Props {
        current_user: _,
        items,
    }: &Props,
) -> Html {
    // Компонент списка данных для аналитики

    html! {
        <>
        {items.iter().map(|item|{

            html! {
                <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4">{item.id}</td>
                    <td class="px-6 py-4">{item.name.clone()}</td>
                    <td class="px-6 py-4">{item.fio.clone()}</td>
                    <td class="px-6 py-4">{item.measure.clone()}</td>
                    <td class="px-6 py-4">{item.cnt}</td>
                </tr>
            }
        }).collect::<Vec<_>>()}
        </>
    }
}
