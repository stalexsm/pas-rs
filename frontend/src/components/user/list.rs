use crate::{check_is_admin, Role, User};
use chrono::Local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub items: Vec<User>,
    pub current_user: Option<User>,
    pub on_edit: Callback<User>,
}

#[function_component(UserList)]
pub fn user_lists(
    Props {
        current_user,
        items,
        on_edit,
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

            // Color adj
            let color = if !item.blocked {"green"} else {"red"};
            html! {
                <tr class="hover:bg-gray-50">
                    <td class="px-6 py-4">{item.id}</td>
                    <td class="px-6 py-4">
                    if current_user.as_ref().map_or(false, |u| u.id == item.id) {
                        <svg xmlns="http://www.w3.org/2000/svg"
                            fill="#60a5fa"
                            viewBox="0 0 24 24"
                            stroke-width="1.0"
                            stroke="currentColor"
                            class="w-5 h-5">
                            <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M11.48 3.499a.562.562 0 011.04 0l2.125 5.111a.563.563 0 00.475.345l5.518.442c.499.04.701.663.321.988l-4.204 3.602a.563.563 0 00-.182.557l1.285 5.385a.562.562 0 01-.84.61l-4.725-2.885a.563.563 0 00-.586 0L6.982 20.54a.562.562 0 01-.84-.61l1.285-5.386a.562.562 0 00-.182-.557l-4.204-3.602a.563.563 0 01.321-.988l5.518-.442a.563.563 0 00.475-.345L11.48 3.5z" />
                        </svg>
                    }
                    </td>
                    <th class="flex gap-3 px-6 py-4 font-normal text-gray-900">
                    <div class="text-sm">
                        <div class="font-medium text-gray-700">
                            {item.fio.clone()}
                        </div>
                        <div class="text-gray-400">
                            {item.email.clone()}
                        </div>
                    </div>
                    </th>
                    if current_user.as_ref().map_or(false, |u| check_is_admin(u.role)) {
                         <td class="px-6 py-4"> {item.organization.clone().map_or("-".to_string(), |o| o.name.clone())} </td>
                    }
                    <td class="px-6 py-4">{
                        match item.role {
                            Role::Developer => {"Разработчик"},
                            Role::Admin => {"Администратор"},
                            Role::Director => {"Руководитель"},
                            Role::User => {"Пользователь"},

                        }
                    }</td>
                    <td class="px-6 py-4">
                    <span
                        class={format!("inline-flex items-center gap-1 rounded-full bg-{0}-50 px-2 py-1 text-xs font-semibold text-{0}-600", color)}
                    >
                        {if item.blocked {"Заблокирован"} else {"Разблокирован"}}
                    </span>
                    </td>
                    <td class="px-6 py-4">{item.created_at.with_timezone(&Local).format("%d.%m.%Y %H:%M").to_string()}</td>
                    <td class="px-6 py-4">
                    <div class="flex justify-end gap-4">
                    if item.id != 1 {
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
