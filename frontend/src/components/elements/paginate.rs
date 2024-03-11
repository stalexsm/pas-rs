use crate::Route;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Default, Clone)]
pub struct Props {
    pub cnt: i64,
    pub path: Route,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Q {
    pub page: i64,
    pub per_page: i64,
}

#[function_component(Paginate)]
pub fn paginate(props: &Props) -> Html {
    // Обработка Page

    let start_page = (props.page - 2).max(1).min(props.cnt - 4);
    let end_page = (start_page + 4).min(props.cnt);

    html! {
        <div class="flex items-center justify-between border-t border-gray-200 bg-white px-4 py-3 sm:px-6">
            <div class="flex flex-1 justify-between sm:hidden">
                <Link<Route, Q>
                    to={props.path.clone()}
                    classes={classes!(
                        "relative",
                        "inline-flex",
                        "items-center",
                        "rounded-md",
                        "border",
                        "border-gray-300",
                        "bg-white",
                        "px-4",
                        "py-2",
                        "text-sm",
                        "font-medium",
                        "text-gray-700",
                        "hover:bg-gray-50",
                        if props.page == 1 {"cursor-default pointer-events-none opacity-30"} else {""}
                    )}
                    query={Some(Q {
                        page: if (props.page - 1) >= 1 {props.page - 1} else {1},
                        per_page: props.per_page,
                    })}
                >
                    {"Предыдущая"}
                </Link<Route, Q>>
                <Link<Route, Q>
                    to={props.path.clone()}
                    classes={classes!(
                            "relative",
                            "ml-3",
                            "inline-flex",
                            "items-center",
                            "rounded-md",
                            "border",
                            "border-gray-300",
                            "bg-white",
                            "px-4",
                            "py-2",
                            "text-sm",
                            "font-medium",
                            "text-gray-700",
                            "hover:bg-gray-50",
                            if props.page >= props.cnt {"cursor-default pointer-events-none opacity-30"} else {""}

                        )}
                    query={Some(Q {
                        page: if (props.page + 1) <= props.cnt {props.page + 1} else {props.page},
                        per_page: props.per_page,
                    })}
                >
                    {"Следующая"}
                </Link<Route, Q>>
            </div>
            <div class="hidden sm:flex sm:flex-1 sm:items-center sm:justify-between">
                <div></div>
            <div>
                <nav class="isolate inline-flex -space-x-px rounded-md shadow-sm" aria-label="Pagination">
                    <Link<Route, Q>
                        to={props.path.clone()}
                        classes={
                            classes!(
                                "relative",
                                "inline-flex",
                                "items-center",
                                "rounded-l-md",
                                "px-2",
                                "py-2",
                                "text-gray-400",
                                "ring-1",
                                "ring-inset",
                                "ring-gray-300",
                                "hover:bg-gray-50",
                                "focus:z-20",
                                "focus:outline-offset-0",
                                if props.page == 1 {"cursor-default pointer-events-none opacity-30"} else {""}
                            )
                        }
                        query={Some(Q {
                            page: if (props.page - 1) >= 1 {props.page - 1} else {1},
                            per_page: props.per_page,
                        })}
                    >
                        <span class="sr-only">{"Предыдущая"}</span>
                        <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                            <path
                                fill-rule="evenodd"
                                d="M12.79 5.23a.75.75 0 01-.02 1.06L8.832 10l3.938 3.71a.75.75 0 11-1.04 1.08l-4.5-4.25a.75.75 0 010-1.08l4.5-4.25a.75.75 0 011.06.02z"
                                clip-rule="evenodd"
                            />
                        </svg>
                    </Link<Route, Q>>
                {for (start_page..=end_page).map(|i| html! {
                    <Link<Route, Q>
                        to={props.path.clone()}
                        classes={
                            if props.page == i {
                            "cursor-default pointer-events-none relative z-10 inline-flex items-center bg-blue-600 px-4 py-2 text-sm font-semibold text-white focus:z-20 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
                            } else {
                                "relative inline-flex items-center px-4 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-20 focus:outline-offset-0"
                            }
                        }
                        query={Some(Q {
                            page: i,
                            per_page: props.per_page,
                        })}
                        disabled={props.page == i}
                    >
                        {i}
                    </Link<Route, Q>>
                })}

                <Link<Route, Q>
                    to={props.path.clone()}
                    classes={classes!(
                        "relative",
                        "inline-flex",
                        "items-center",
                        "rounded-r-md",
                        "px-2",
                        "py-2",
                        "text-gray-400",
                        "ring-1",
                        "ring-inset",
                        "ring-gray-300",
                        "hover:bg-gray-50",
                        "focus:z-20",
                        "focus:outline-offset-0",
                        if props.page >= props.cnt {"cursor-default pointer-events-none opacity-30"} else {""}
                    )}
                    query={Some(Q {
                        page: if (props.page + 1) <= props.cnt {props.page + 1} else {props.page},
                        per_page: props.per_page,
                    })}
                >
                    {
                        html! {
                            <>
                            <span class="sr-only">{"Следующая"}</span>
                            <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                <path
                                    fill-rule="evenodd"
                                    d="M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                            </>
                        }
                    }
                </Link<Route, Q>>
                </nav>
            </div>
            </div>
        </div>
    }
}
