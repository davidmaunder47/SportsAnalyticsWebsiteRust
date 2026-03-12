use dioxus::prelude::*;
use dioxus_router::{Link, Outlet};

use dioxus::prelude::{asset, Asset};
use crate::pages::router;

static CSSNAV_BAR: Asset = asset!("/assets/CSS/navbar.css");


#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            "© 2025 SPORTS BUFF. All rights reserved."
        }
    }
}

#[component]
pub fn NavigationBar() -> Element {
    rsx! {
        Stylesheet {href: CSSNAV_BAR}
        header { class: "navbox",
            div { class: "logo", "SPORTS BUFF" }

            nav {
                Link { to: router::Route::Home {}, "HOME" }
                Link { to: router::Route::Mlb {}, "MLB" }
                Link { to: router::Route::Nba {}, "NBA" }
                Link { to: router::Route::Nfl {}, "NFL" }
                Link { to: router::Route::Nhl {}, "NHL" }
                Link { to: router::Route::Esports {}, "ESPORTS" }
            }
        }
        Outlet::<router::Route> {}
    }
}

pub fn get_seasons_diff(year_start: u32, year_end: u32) -> Vec<String> {
    (year_start..year_end).rev()
        .map(|year| format!("{}-{:02}", year, (year + 1) % 100))
        .collect()
}

#[allow(dead_code)]
pub fn get_seasons_same(year_start: u32, year_end: u32) -> Vec<String> {
    (year_start..=year_end).map(|year| year.to_string()).collect()
}