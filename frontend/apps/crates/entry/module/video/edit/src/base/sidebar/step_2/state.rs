use crate::base::state::Base;
use components::{image::search::{callbacks::Callbacks as ImageSearchCallbacks, state::{ImageSearchCheckboxKind, ImageSearchOptions, State as ImageSearchState}}, stickers::state::Stickers, tabs::MenuTabKind};
use dominator::clone;
use futures_signals::signal::Mutable;
use std::rc::Rc;

pub struct Step2 {
    pub base: Rc<Base>,
    pub tab: Mutable<Tab>,
}

impl Step2 {
    pub fn new(base: Rc<Base>) -> Rc<Self> {
        let kind = match crate::debug::settings().content_tab {
            Some(kind) => kind,
            None => MenuTabKind::Video,
        };

        let tab = Mutable::new(Tab::new(base.clone(), kind));

        Rc::new(Self { base, tab })
    }
}

#[derive(Clone)]
pub enum Tab {
    Video,
    Text, // uses top-level state since it must be toggled from main too
    Image(Rc<ImageSearchState>),
}

impl Tab {
    pub fn new(base: Rc<Base>, kind: MenuTabKind) -> Self {
        match kind {
            MenuTabKind::Video => Self::Video,
            MenuTabKind::Text => Self::Text,
            MenuTabKind::Image => {
                let opts = ImageSearchOptions {
                    checkbox_kind: Some(ImageSearchCheckboxKind::StickersFilter),
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(Some(clone!(base => move |image| {
                    log::info!("{:?}", image);
                    Stickers::add_sprite(base.stickers.clone(), image);
                })));
                let state = ImageSearchState::new(opts, callbacks);

                Self::Image(Rc::new(state))
            },

            _ => unimplemented!("unsupported tab kind!")
        }
    }

    pub fn kind(&self) -> MenuTabKind {
        match self {
            Self::Video => MenuTabKind::Video,
            Self::Text => MenuTabKind::Text,
            Self::Image(_) => MenuTabKind::Image,
        }
    }
}
