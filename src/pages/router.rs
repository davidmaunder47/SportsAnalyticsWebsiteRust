
use crate::pages::nba::*;
use crate::pages::home::Home;
use crate::pages::nhl::*;
use crate::pages::nfl::*;
use crate::pages::mlb::*;
use crate::pages::esports::*;


use dioxus::prelude::*;
use dioxus_router::{Routable};
#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/mlb")]
    Mlb {},

    #[nest("/nba")]
        #[route("/")]
        Nba {},

        #[route("/OnOff")]
        OnOff {},

        #[route("/Stats")]
        Stats {},

        #[route("/Search")]
        Search {},
    
    #[end_nest]


    #[route("/nfl")]
    Nfl {},

    #[route("/nhl")]
    Nhl {},

    #[route("/esports")]
    Esports {},
}
