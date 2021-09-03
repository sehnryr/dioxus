use crate::recoil;
use crate::state::{FilterState, TODOS};
use dioxus_core::prelude::*;

pub fn FilterToggles(cx: Context<()>) -> DomTree {
    let reducer = recoil::use_callback(&cx, || ());
    let items_left = recoil::use_atom_family(&cx, &TODOS, uuid::Uuid::new_v4());

    let toggles = [
        ("All", "", FilterState::All),
        ("Active", "active", FilterState::Active),
        ("Completed", "completed", FilterState::Completed),
    ]
    .iter()
    .map(|(name, path, filter)| {
        rsx!(li {
            class: "{name}"
            a {
                "{name}"
                href: "{path}"
                onclick: move |_| reducer.set_filter(&filter)
            }
        })
    });

    // todo
    let item_text = "";
    let items_left = "";

    cx.render(rsx! {
        footer {
            span {
                strong {"{items_left}"}
                span {"{item_text} left"}
            }
            ul {
                class: "filters"
                {toggles}
            }
        }
    })
}