use components::{
    module::_common::play::prelude::DomRenderable,
    backgrounds::dom::render_single_background_raw
};
use dominator::{html, Dom, clone};
use std::rc::Rc;
use components::backgrounds;
use futures_signals::{
    signal_vec::SignalVecExt,
    signal::SignalExt
};
use utils::prelude::*;
use super::{
    state::{Base, Phase},
    game::dom::render as render_game,
};


impl DomRenderable for Base {
    fn render(state: Rc<Base>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .child(render_single_background_raw(&state.background, state.theme_id, None))
            .child_signal(state.phase.signal_cloned().map(|phase| {
                match phase {
                    Phase::Init | Phase::Ending => None,
                    Phase::Playing(game) => Some(render_game(game)),
                }
            }))
        })
    }
}
