use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use components::{
    tabs::{MenuTab, MenuTabKind},
    image::search::dom::render as render_image_search,
    color_select::dom::render as render_color_picker
};

pub fn render(state: Rc<Step2>) -> Dom {
    html!("menu-tabs", {
        .children(&mut [
            render_tab(state.clone(), MenuTabKind::Image),
            render_tab(state.clone(), MenuTabKind::Color),
            render_tab(state.clone(), MenuTabKind::Overlay),
            html!("module-sidebar-body", {
                .property("slot", "body")
                .child_signal(state.tab.signal_cloned().map(|tab| {
                    match tab {
                        Tab::Image(state) => {
                            Some(render_image_search(state.clone(), None))
                        },
                        Tab::Color(state) => {
                            Some(render_color_picker(state.clone(), None))
                        },
                        Tab::Overlay(state) => {
                            Some(render_image_search(state.clone(), None))
                        },
                    }
                }))
            })
        ])
    })
}

fn render_tab(state: Rc<Step2>, tab_kind:MenuTabKind) -> Dom {
    MenuTab::render(
        MenuTab::new(
            tab_kind,
            false,
            clone!(state => move || state.tab.signal_ref(clone!(tab_kind => move |curr| {
                curr.kind() == tab_kind
            }))),
            clone!(state, tab_kind => move || {
                state.tab.set(Tab::new(state.base.clone(), tab_kind));
            })
        ),
        Some("tabs")
    )
}