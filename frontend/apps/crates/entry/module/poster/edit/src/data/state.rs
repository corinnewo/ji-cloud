#![feature(type_alias_impl_trait)]
#![feature(min_type_alias_impl_trait)]
use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable,  SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use crate::debug;
use super::{actions, raw};
use itertools::Itertools;
use std::fmt::Write;
use serde::Deserialize;
use components::module::page::ModulePageKind;
use std::collections::HashSet;
use components::module::history::state::HistoryState;
use shared::{domain::{
    jig::{JigId, module::ModuleId},
    audio::AudioId
}, media::MediaLibrary};
use dominator_helpers::futures::AsyncLoader;
use wasm_bindgen_futures::spawn_local;
use utils::prelude::*;
use super::actions::{HistoryChangeFn, HistoryUndoRedoFn};
//See: https://users.rust-lang.org/t/eli5-existential/57780/16?u=dakom
//
//Basically, the type of these callbacks are closures created from *inside*
//Since we don't have the actual type here on the *outside* we can't define it
//However, we do know something about the type - namely, that it will *exist*
//Hence, the so-called "existential" type
//
//For this to be true it must actually be defined eventually though
//so that the compiler can kinda figure it out and fill the type in backwards
pub type HistoryStateImpl = HistoryState<raw::ModuleData, HistoryChangeFn, HistoryUndoRedoFn>;

pub struct State {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub theme_id: Mutable<ThemeId>,
    pub instructions: Instructions,
    pub save_loader: Rc<AsyncLoader>,
    pub step: Mutable<Step>,
    pub steps_completed: Mutable<HashSet<Step>>,
    history: RefCell<Option<Rc<HistoryStateImpl>>>,
}

pub struct Instructions {
    pub audio_id: Mutable<Option<AudioId>>,
    pub text: Mutable<Option<String>>
}

impl Instructions {
    pub fn new(raw_data: &raw::ModuleData) -> Self {
        Self {
            audio_id: Mutable::new(raw_data.instructions.audio_id.clone()),
            text: Mutable::new(raw_data.instructions.text.clone())
        }
    }
}

impl State {
    pub fn new(jig_id: JigId, module_id: ModuleId, raw_data:raw::ModuleData) -> Rc<Self> {
        let step = Mutable::new(match debug::settings().step.as_ref() {
            Some(step) => step.clone(),
            None => Step::One
        });


        let _self = Rc::new(Self {
            jig_id,
            module_id,
            theme_id: Mutable::new(raw_data.theme_id),
            instructions: Instructions::new(&raw_data),
            save_loader: Rc::new(AsyncLoader::new()),
            step,
            steps_completed: Mutable::new(HashSet::new()),
            history: RefCell::new(None),
        });

        let history = Rc::new(HistoryState::new(
            raw_data,
            actions::history_on_change(_self.clone()),
            actions::history_on_undoredo(_self.clone()),
        ));

        *_self.history.borrow_mut() = Some(history);

        _self
    }

    pub fn get_history(&self) -> Rc<HistoryStateImpl> {
        self.history.borrow().as_ref().unwrap_ji().clone()
    }
    pub fn theme_id_str_signal(&self) -> impl Signal<Item = &'static str> {
        self.theme_id.signal_ref(|id| id.as_str_id())
    }

    pub fn page_kind_signal(&self) -> impl Signal<Item = ModulePageKind> {

        self.step.signal()
            .map(|step| {
                if step == Step::Four {
                    ModulePageKind::GridResizePreview
                } else {
                    ModulePageKind::GridResizeScrollable
                }
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Step {
    One,
    Two,
    Three,
    Four
}

impl Step {
    pub fn label(&self) -> &'static str {
        match self {
            Step::One => crate::strings::steps_nav::STR_CONTENT,
            Step::Two => crate::strings::steps_nav::STR_DESIGN,
            Step::Three => crate::strings::steps_nav::STR_SETTINGS,
            Step::Four => crate::strings::steps_nav::STR_PREVIEW,
        }
    }

    pub fn number(&self) -> u8 {
        match self {
            Step::One => 1, 
            Step::Two => 2, 
            Step::Three => 3, 
            Step::Four => 4 
        }
    }
}
