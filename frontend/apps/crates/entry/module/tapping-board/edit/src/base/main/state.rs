use components::module::_common::edit::prelude::*;
use components::traces::{
    bubble::TraceBubble,
    edit::TracesEdit
};
use crate::base::state::Base;
use std::rc::Rc;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::{
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{SignalVec, SignalVecExt}
};
use utils::prelude::*;
use dominator::clone;
use shared::domain::jig::module::body::tapping_board::Step;

pub struct Main {
    pub base: Rc<Base>,
}

impl Main {
    pub fn new(base: Rc<Base>) -> Self {
        Self {
            base,
        }
    }

    pub fn phase_signal(&self) -> impl Signal<Item = Phase> {
        self.base.step.signal()
            .map(|step| step == Step::Three)
            .dedupe()
            .map(|is_step_three| {
                if is_step_three {
                    Phase::Trace
                } else {
                    Phase::Layout
                }
            })
    }

    pub fn trace_bubbles(&self) -> impl SignalVec<Item = Rc<TraceBubble>> {
        self.base
            .traces_meta
            .signal_vec_cloned()
            .map_signal(|trace_meta| trace_meta.bubble.signal_cloned())
            .filter(|bubble| bubble.is_some())
            .map(|bubble| bubble.unwrap_ji())
    }
}

#[derive(Clone, Copy)]
pub enum Phase {
    Layout,
    Trace
}


impl MainExt for Main {
}


