use dominator::{clone, html, Dom};
use futures_signals::{
    signal::SignalExt,
    signal_vec::{MutableVec, SignalVecExt},
};
use shared::domain::jig::JigResponse;
use std::rc::Rc;
use utils::{ages::AgeRangeVecExt, events, jig::published_at_string};
use components::module::_common::thumbnail::ModuleThumbnail;

use super::super::state::State;

pub fn render(state: Rc<State>, query: String, jigs: Rc<MutableVec<JigResponse>>) -> Dom {
    html!("home-search-results", {
        .property_signal("resultsCount", jigs.signal_vec_cloned().len().map(|len| len as u32))
        .property("query", &query)
        .child(
            html!("home-search-results-section", {
                .property("slot", "sections")
                .property_signal("resultsCount", jigs.signal_vec_cloned().len().map(|len| len as u32))
                .children_signal_vec(jigs.signal_vec_cloned().map(clone!(state => move |jig| {
                    render_result(state.clone(), &jig)
                })))
            })
        )
    })
}

fn render_result(state: Rc<State>, jig: &JigResponse) -> Dom {
    let jig_ages = jig.jig_data.age_ranges.clone();
    html!("home-search-result", {
        .property("slot", "results")
        .property("title", &jig.jig_data.display_name)
        .property("playedCount", "???")
        .property("likedCount", "???")
        .property("author", &jig.author_name.clone().unwrap_or_default())
        .property("publishedAt", {
            match jig.published_at {
                Some(publish_at) => published_at_string(publish_at, false),
                None => String::new(),
            }
        })
        .property("language", &jig.jig_data.language)
        .property_signal("ages", state.search_options.age_ranges.signal_cloned().map(move |age_ranges| {
            age_ranges.range_string(&jig_ages)
        }))
        .property("description", jig.jig_data.description.clone())
        .children(&mut [
            ModuleThumbnail::render(
                Rc::new(ModuleThumbnail {
                    jig_id: jig.id.clone(),
                    module: jig.jig_data.modules[0].clone(),
                    is_jig_fallback: true,
                }),
                Some("image")
            ),

            html!("home-search-result-details", {
                .property("slot", "categories")
                .children(jig.jig_data.categories.iter().map(|category_id| {
                    html!("home-search-result-category", {
                        .property_signal("label", {
                            state.search_options.category_label_lookup.signal_cloned().map(clone!(category_id => move |category_label_lookup| {
                                match category_label_lookup.get(&category_id) {
                                    Some(label) => label.to_owned(),
                                    None => String::new(),
                                }
                            }))
                        })
                    })
                }))
            }),
            html!("button-rect", {
                .property("slot", "play-button")
                .property("color", "blue")
                .property("bold", true)
                .text("Play")
                .event({
                    let jig_id = jig.id;
                    clone!(state => move |_: events::Click| {
                        state.play_jig.set(Some(jig_id));
                    })
                })
            }),
        ])
    })
}

// new: false,
// leaningPathJigCount: undefined,
// playedCount: 10,
// likedCount: 20,
// byJiTeam: false,
