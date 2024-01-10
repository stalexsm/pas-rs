use super::ProducedGood;
use crate::{Role, User};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub items: Vec<ProducedGood>,
    pub current_user: Option<User>,
    pub on_edit: Callback<ProducedGood>,
    pub on_add_adj: Callback<ProducedGood>,
}

#[function_component(ProducedGoodList)]
pub fn produced_good_lists(
    Props {
        current_user,
        items,
        on_edit,
        on_add_adj,
    }: &Props,
) -> Html {
    // Компонент списка данных для домвшней страницы

    html! {
        <>
        {items.iter().map(|item|{

            // Generate onclick
            let on_edit = {
                let on_edit = on_edit.clone();
                let cloned_item = item.clone();
                Callback::from(move |e: MouseEvent| {
                    e.prevent_default();

                    on_edit.emit(cloned_item.clone());
                })
            };

            let on_add_adj = {
                let on_add_adj = on_add_adj.clone();
                let cloned_item = item.clone();
                Callback::from(move |e: MouseEvent| {
                    e.prevent_default();

                    on_add_adj.emit(cloned_item.clone());
                })
            };

            // Color adj
            let color_adj = if item.adj >= 0 {"green"} else {"red"};
            html! {
                <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4">{item.id}</td>
                    <th class="flex gap-3 px-6 py-4 font-normal text-gray-900">
                        <div class="text-sm">
                        <div class="font-medium text-gray-700">{item.product.name.clone()}</div>
                            <div class="text-gray-400">{item.product.measure_unit.name.clone()}</div>
                        </div>
                    </th>
                    <td class="px-6 py-4">{item.user.fio.clone().unwrap_or_else(|| "N/A".to_string() )}</td>
                    <td class="px-6 py-4">
                    <span
                        class={format!("inline-flex items-center gap-1 rounded-full bg-{0}-50 px-2 py-1 text-xs font-semibold text-{0}-600", color_adj)}
                    >
                        {item.adj}
                    </span>
                    </td>
                    <td class="px-6 py-4">{item.cnt + item.adj}</td>
                    <td class="px-6 py-4">{item.created_at.format("%d.%m.%Y %H:%M").to_string()}</td>
                    <td class="px-6 py-4">
                    <div class="flex justify-end gap-4">
                        <a
                        onclick={on_add_adj}
                        x-data="{ tooltip: 'Delete' }" href="#">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                            <path
                            stroke-linecap="round"
                            stroke-linejoin="round" d="M8.25 18.75a1.5 1.5 0 01-3 0m3 0a1.5 1.5 0 00-3 0m3 0h6m-9 0H3.375a1.125 1.125 0 01-1.125-1.125V14.25m17.25 4.5a1.5 1.5 0 01-3 0m3 0a1.5 1.5 0 00-3 0m3 0h1.125c.621 0 1.129-.504 1.09-1.124a17.902 17.902 0 00-3.213-9.193 2.056 2.056 0 00-1.58-.86H14.25M16.5 18.75h-2.25m0-11.177v-.958c0-.568-.422-1.048-.987-1.106a48.554 48.554 0 00-10.026 0 1.106 1.106 0 00-.987 1.106v7.635m12-6.677v6.677m0 4.5v-4.5m0 0h-12" />
                        </svg>
                        </a>
                        if current_user.as_ref().map_or(false, |u| u.role == Role::Admin.to_string()) {
                            <a
                            onclick={on_edit}
                            x-data="{ tooltip: 'Edite' }" href="#">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                class="h-6 w-6"
                                x-tooltip="tooltip"
                            >
                                <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125"
                                />
                            </svg>
                            </a>
                        }
                    </div>
                    </td>
                </tr>
            }
        }).collect::<Vec<_>>()}

        </>
    }
}
