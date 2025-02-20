use super::state::*;
use components::{
    tabs::{MenuTab, MenuTabKind},
    color_select::dom::render as render_color_picker,
    image::search::dom::render as render_image_search,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::prelude::*;

pub fn render(state: Rc<Step1>) -> Dom {
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

fn render_tab(state: Rc<Step1>, tab_kind:MenuTabKind) -> Dom {
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