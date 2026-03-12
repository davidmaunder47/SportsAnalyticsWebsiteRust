use dioxus::prelude::*;
use dioxus_router::{Link};
use serde::Serialize;
use router::Route;
use crate::backend::DbJsonError;
use crate::pages::common;
use crate::pages::router;
use crate::shared::nba::{NbaGeneral, EnumNBAGeneral, NbaGeneralPartialNoYearsNoFantasy};
use crate::shared::{convert_json_tostringvec, DBStructSupport};
static CSSNBA : Asset = asset!("/assets/CSS/nba/nba.css");
static CSSNBASTATS : Asset = asset!("/assets/CSS/nba/nbastats.css");
#[component]
pub fn Nba() -> Element {
    rsx! {
        NavBar {}
        section {
            class: "hero",
            h2{"Welcome to the Hardwood"}
        }
        common::Footer{}
    }
}


#[component]
fn NavBar() -> Element {
    rsx! {
        Stylesheet{ href: CSSNBA}
        header{
            h1{"NBA Hub"}
            p { "Your ultimate basketball companion"}
        }

        nav {
            Link { to: router::Route::Home {}, "HOME" }
            Link { to: router::Route::OnOff {}, "OnOff" }
            Link { to: router::Route::Stats {}, "Stats" }
            Link { to: router::Route::Search {}, "Search" }
        }
    }
}

#[component]
pub fn OnOff() -> Element {
    rsx!{
        NavBar{}
        h1{"on off test for now"}
        common::Footer{}
    }
}

#[server]
async fn db_query_nba(year: i32) -> Result<Vec<NbaGeneralPartialNoYearsNoFantasy>> {
    use crate::backend::database::nba::NBAGeneralBaseDB;
    use crate::backend::database::general::DBManager;
    let nba_database: NBAGeneralBaseDB  = NBAGeneralBaseDB::new_connection_to_db("nbastats").await.map_err(DbJsonError::from)?;
    let players : Vec<NbaGeneralPartialNoYearsNoFantasy> = nba_database.query_db_year("nba_general", year).await?;
    Ok(players)
}
//todo!() move to common file
//use i32 since postgres doesnt deal well with u32 types
fn covert_string_to_int(input: &str) -> i32 {
    println!("{}", input);
    input.split('-')
        .next()
        .and_then(|part| part.parse::<i32>().ok())
        .unwrap_or(2025)
}


//Todo move this to a common thing
#[component]
pub fn table_build<T: DBStructSupport>(table: Resource<Result<Vec<T>>>, player: T) -> Element
where T: Serialize + 'static + std::cmp::PartialEq + std::clone::Clone,
{
    let ref_guard = table.read_unchecked();
    let mut player_data = use_signal(|| Vec::<T>::new());
    // let mut unwrap_table = match &*ref_guard {
    //     Some(Ok(data)) => player_data.set(data.clone()),
    //     Some(Err(err)) => return rsx! { "Error: {err}" },
    //     None => return rsx! { div { "Fetching data..." } },
    // };

    // 2. Track the Resource state reactively
    let resource_result = table.read();

    // 3. EFFECT: When the resource finishes fetching NEW data, update player_data
    // This runs every time the Resource (table) changes its value
    use_effect(move || {
        if let Some(Ok(new_data)) = table.read().as_ref() {
            player_data.set(new_data.clone());
        }
    });

    // Handle initial loading/error states
    if resource_result.is_none() {
        return rsx! { div { "Fetching data..." } };
    }
    if let Some(Err(err)) = &*resource_result {
        return rsx! { "Error: {err}" };
    }

    let headers = convert_json_tostringvec(&player).unwrap_or_default();
    let mut tbd = use_signal(|| "".to_string());
    let function_player = use_signal(||player.clone());
    let mut descending = use_signal(|| true);

    rsx! {
        table {
            tr {
                for header in &headers {
                    th {
                        style: "cursor: pointer;",
                        onclick: {
                            let h = header.clone();
                            let h2 = header.clone();
                            move |_| {
                                tbd.set(h.clone());
                                function_player.read().sort_vector(&mut player_data.write(), h2.clone(), *descending.read());
                                descending.toggle();
                            }
                        },
                        "{header}"
                    }
                }
            }

            for p in player_data.read().iter().take(50) {
                if let Ok(json_val) = serde_json::to_value(p) {
                    if let Some(obj) = json_val.as_object() {
                        tr { // START a new row for each player
                            for (_key, value) in obj {
                                td { "{value}" }
                            }
                        }
                    }
                }
            }
        }

        {tbd}
    }
}



// #[component]
// pub fn table_build_no_clickable_columns<T: DBStructSupport>(table: Vec<T>, player: T) -> Element
// where T: Serialize + 'static + std::cmp::PartialEq + std::clone::Clone,
// {
//     let headers = match convert_json_tostringvec(&player) {
//         Ok(h) => h,
//         Err(_) => vec![],
//     };
//
//     let player_data = use_signal(|| table);
//
//     rsx! {
//         table {
//             // 1. Header Row
//             tr {
//                 for header in &headers {
//                     th {
//                         style: "cursor: pointer;",
//                         // onclick: move |_| {
//                         //
//                         // },
//                         "{header}"
//                     }
//                 }
//             }
//
//             for p in player_data.read().iter().take(50) {
//                 if let Ok(json_val) = serde_json::to_value(p) {
//                     if let Some(obj) = json_val.as_object() {
//                         tr { // START a new row for each player
//                             for (_key, value) in obj {
//                                 td { "{value}" }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }



#[component]
pub fn Stats() -> Element {
    let years = common::get_seasons_diff(1996, 2026);
    let mut current_year = use_signal(|| 2025);
    //get data on click
    //rendered data after click
    //have button that sorts data based on click
    let player: NbaGeneralPartialNoYearsNoFantasy = std::default::Default::default();
    let mut get_data = use_resource(move || async move {
        db_query_nba(*current_year.peek()).await
    });

    let headers = match convert_json_tostringvec(&player) {
        Ok(h) => h,
        Err(_) => vec![],
    };

    rsx! {
        NavBar {}
        Stylesheet{ href: CSSNBASTATS}

        label { "Select Years " }
        select {
            onchange: move |event| current_year.set(covert_string_to_int(&event.value())),
            for year in years {
                option { "{year}" }
            }
        }

        button {
            position: "relative",
            left: "10px",
            onclick: move |_| {
                get_data.restart();
            },
            "GetData"
        }

        table_build{table: get_data, player: player}
        common::Footer {}
    }

}

#[component]
pub fn Search() -> Element {
    rsx! {
        NavBar{}
        common::Footer{}
    }
}