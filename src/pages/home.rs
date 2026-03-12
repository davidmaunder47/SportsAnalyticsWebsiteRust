use dioxus::prelude::*;

use dioxus::prelude::{asset, Asset};

use crate::pages::common;

static CSSGENERAL: Asset = asset!("/assets/CSS/general.css");
static CSSMAIN: Asset = asset!("/assets/CSS/main.css");

#[component]
pub fn Home() -> Element {
    rsx! {
        Stylesheet{ href: CSSMAIN}
        Stylesheet{ href: CSSGENERAL}

        h1 { class: "pageinfo",
            "Welcome to SPORTS BUFF Get in-depth stats and highlights about your favourite players and teams"
        }
        common::NavigationBar{}
        HomeImages{}
        common::Footer{}
    }

}

//finish this code by generating images TODO
#[component]
fn HomeImages() -> Element {
    rsx! {

        // 🔥 Featured Stats
        section { class: "section", id: "tournaments",
            h2 { "🔥 Featured Stats" }

            div { class: "cards",

                div { class: "card",
                    img {
                        src: asset!("/assets/Images/Sports/philly-special.jpg"),
                        alt: "Valorant Cup"
                    }
                    h3 { "Valorant Championship" }
                    p { "Oct 15 - Oct 22, 2025" }
                }

                div { class: "card",
                    img {
                        src: asset!("/assets/Images/Sports/leafs-lose.jpg"),
                        alt: "League of Legends Worlds"
                    }
                    h3 { "LoL World Finals" }
                    p { "Nov 5 - Nov 20, 2025" }
                }

                div { class: "card",
                    img {
                        src: asset!("/assets/Images/Sports/lebron-block.jpg"),
                        alt: "CS2 Masters"
                    }
                    h3 { "CS2 Masters" }
                    p { "Dec 1 - Dec 10, 2025" }
                }
            }
        }

        // 🎥 Top Videos
        section { class: "section", id: "videos",
            h2 { "🎥 Top Videos" }

            div { class: "cards",

                div { class: "card",
                    img {
                        src: asset!("/assets/Images/Sports/steph.jpg"),
                        alt: "Team Phoenix"
                    }
                    h3 { "Team Phoenix" }
                    p { "Reigning champions in Valorant" }
                }

                div { class: "card",
                    img {
                        src: asset!("/assets/Images/Sports/kwahi-shot.jpg"),
                        alt: "Shadow Squad"
                    }
                    h3 { "Shadow Squad" }
                    p { "Elite CS2 lineup" }
                }

                div { class: "card",
                    img {
                        src: asset!("/assets/Images/Sports/golden-goal.jpg"),
                        alt: "Legion Alpha"
                    }
                    h3 { "Legion Alpha" }
                    p { "Dominating LoL tournaments" }
                }
            }
        }

        // 🏆 Top Teams
        section { class: "section", id: "teams",
            h2 { "🏆 Top Teams" }

            div { class: "cards",

                div { class: "card",
                    img {
                        src: asset!("/assets/Images/Sports/steph.jpg"),
                        alt: "Team Phoenix"
                    }
                    h3 { "Team Phoenix" }
                    p { "Reigning champions in Valorant" }
                }

                div { class: "card",
                    img {
                        src: asset!("/assets/Images/Sports/kwahi-shot.jpg"),
                        alt: "Shadow Squad"
                    }
                    h3 { "Shadow Squad" }
                    p { "Elite CS2 lineup" }
                }

                div { class: "card",
                    img {
                        src: asset!("/assets/Images/Sports/golden-goal.jpg"),
                        alt: "Legion Alpha"
                    }
                    h3 { "Legion Alpha" }
                    p { "Dominating LoL tournaments" }
                }
            }
        }
    }
}