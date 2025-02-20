use super::state::*;
use components::module::_groups::cards::lookup::Side;
use gloo_timers::future::TimeoutFuture;
use shared::domain::jig::module::body::_groups::cards::{CardPair, Card};
use shared::domain::jig::module::body::matching::PlayerSettings;
use wasm_bindgen_futures::spawn_local;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::base::state::Base;
use std::cell::RefCell;
use std::rc::Rc;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt}
};
use rand::prelude::*;
use utils::prelude::*;
use crate::base::state::Phase;
use components::module::_common::play::prelude::*;
use std::convert::TryInto;

impl Game {
    pub fn next(state: Rc<Self>) {
        let rounds_played = state.rounds_played.load(Ordering::SeqCst);

        let has_ended = {
            if rounds_played >= state.base.settings.n_rounds.try_into().unwrap_ji() {
                true
            } else if state.remaining.borrow().len() == 0 {
                log::info!("deck finished, re-shuffling!");
                Self::reset_deck(state.clone());
                false
            } else {
                false
            }
        };

        if !has_ended {
            state.current.set(Some(Current::new(state.clone())));
            state.rounds_played.store(rounds_played+1, Ordering::SeqCst);
            log::info!("playing round {} of {}", rounds_played+1, state.base.settings.n_rounds);
        } else {
            log::info!("GAME OVER!");
            state.base.phase.set(Phase::Ending);
            state.base.set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Positive)));
        }
    }

    pub fn reset_deck(state: Rc<Self>) {

        let mut remaining:Vec<CardPairId> = state.base.raw_pairs
            .iter()
            .enumerate()
            .map(|(index, pair)| {
                CardPairId (pair.0.clone(), pair.1.clone(), index)
            })
            .collect();

        remaining.shuffle(&mut *state.rng.borrow_mut()); 

        *state.used.borrow_mut() = Vec::with_capacity(remaining.len());
        *state.remaining.borrow_mut() = remaining;
        state.current.set(None);
    }
}
