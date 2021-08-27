use std::rc::Rc;

use dominator::{Dom, clone, html};
use futures_signals::signal::{Signal, SignalExt};
use shared::domain::user::{UserProfile, UserScope};
use strum::IntoEnumIterator;
use utils::{events, routes::{AdminRoute, HomeRoute, JigRoute, Route, UserRoute}};

use crate::page_header::state::{LoggedInState, PageLinks};

use super::{actions, state::State};

const STR_SIGN_UP: &'static str = "Sign up";
const STR_LOGIN: &'static str = "Login";
const STR_LOGOUT: &'static str = "Logout";
const STR_ADMIN: &'static str = "Admin";
const STR_DONATE: &'static str = "Donate";

pub fn render(state: Rc<State>, slot: Option<&str>, active_page: Option<PageLinks>) -> Dom {
    actions::fetch_profile(Rc::clone(&state));

    html!("page-header", {
        .apply_if(slot.is_some(), |dom| {
            dom.property("slot", slot.unwrap())
        })
        .children(PageLinks::iter().map(|page_link| {
            html!("page-header-link", {
                .property("slot", "links")
                .property("kind", page_link.kind_str())
                .property("active", match &active_page {
                    Some(active_page) if active_page == &page_link => true,
                    _ => false,
                })
                .property("href", &page_link.route().to_string())
            })
        }))
        .children(&mut [
            html!("button-rect", {
                .property("slot", "donate")
                .property("color", "green")
                .property("size", "small")
                .property("bold", true)
                .text(STR_DONATE)
            }),
            html!("page-header-student-code", {
                .property("slot", "student-code")
            }),
        ])
        .children_signal_vec(state.logged_in.signal_cloned().map(clone!(state => move|logged_in| {
            match logged_in {
                LoggedInState::LoggedIn(user) => render_logged_in(Rc::clone(&state), &user),
                LoggedInState::LoggedOut => render_logged_out(),
                LoggedInState::Loading => vec![],
            }
        })).to_signal_vec())
    })
}

fn admin_privileges(state: Rc<State>) -> impl Signal<Item = bool> {
    state.logged_in.signal_ref(|logged_in_state| {
        match logged_in_state {
            LoggedInState::LoggedIn(profile) if profile.scopes.contains(&UserScope::Admin) => true,
            _ => false
        }
    })
}

fn render_logged_in(state: Rc<State>, user: &UserProfile) -> Vec<Dom> {
    vec![html!("page-header-profile", {
        .property("slot", "user")
        .property("name", &user.given_name)
        .property("email", &user.email)
        .child(html!("button-rect", {
            .property("slot", "logout")
            .property("kind", "outline")
            .property("size", "small")
            .property("color", "blue")
            .text(STR_LOGOUT)
            .event(clone!(state => move |_: events::Click| {
                actions::logout(Rc::clone(&state));
            }))
        }))
        .child_signal(admin_privileges(Rc::clone(&state)).map(|admin_privileges| {
            match admin_privileges {
                false => None,
                true => {
                    Some(html!("a", {
                        .property("slot", "admin")
                        .property("href", {
                            let url: String = Route::Admin(AdminRoute::Landing).into();
                            url
                        })
                        .text(STR_ADMIN)
                    }))
                }
            }
        }))
    })]
}

fn render_logged_out() -> Vec<Dom> {
    vec![
        html!("button-rect", {
            .property("slot", "user")
            .property("kind", "text")
            .property("color", "black")
            .property("href", &Route::User(UserRoute::Register).to_string())
            .text(STR_SIGN_UP)
        }),
        html!("button-rect", {
            .property("slot", "user")
            .property("kind", "text")
            .property("color", "black")
            .property("href", &Route::User(UserRoute::Login).to_string())
            .text(STR_LOGIN)
        }),
    ]
}
