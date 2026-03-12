use std::cmp::Ordering;
use std::fmt;
use dioxus::signals::{Signal, WritableExt};
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use sqlx::FromRow;
use crate::shared::DBStructSupport;

pub enum EnumNBAGeneral {
    NbaGeneral(NbaGeneral),
    NbaGeneralPartialNoYearsNoFantasy(NbaGeneralPartialNoYearsNoFantasy),
}

#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
#[derive(Deserialize, Serialize, Default, PartialEq, Clone)]
pub struct NbaGeneral {
    pub player_name: String,
    pub nickname: String,
    pub team: String,
    pub age: f64,
    pub gp: i16,
    pub w: i16,
    pub l: i16,
    pub min: f64,
    pub fgm: f64,
    pub fga: f64,
    pub fg_pct: f64,
    pub fg3m: f64,
    pub fg3a: f64,
    pub fg3_pct: f64,
    pub ftm: f64,
    pub fta: f64,
    pub ft_pct: f64,
    pub oreb: f64,
    pub dreb: f64,
    pub reb: f64,
    pub ast: f64,
    pub tov: f64,
    pub stl: f64,
    pub blk: f64,
    pub blka: f64,
    pub pf: f64,
    pub pfd: f64,
    pub pts: f64,
    pub plus_minus: f64,
    pub nba_fantasy_pts: f64,
    pub dd2: i16,
    pub td3: i16,
    pub start_year: i16,
    pub end_year: i16
}

impl DBStructSupport for NbaGeneral {
    const TABLE_NAME: &'static str = "nba_general";
    const SELECT_COLUMNS: &'static str = "*";

    fn sort_vector(&self, list: & mut Vec<Self>, column: String, descending: bool)
    where
        Self: Sized + 'static, // Signal requires 'static
    {

        match column.as_str() {
            "player_name" => list.sort_by(|a, b| a.player_name.cmp(&b.player_name)),
            "nickname" => list.sort_by(|a, b| a.nickname.cmp(&b.nickname)),
            "team" => list.sort_by(|a, b| a.team.cmp(&b.team)),

            "age" => list.sort_by(|a, b| a.age.partial_cmp(&b.age).unwrap_or(Ordering::Equal)),
            "gp" => list.sort_by_key(|a| a.gp),
            "w" => list.sort_by_key(|a| a.w),
            "l" => list.sort_by_key(|a| a.l),

            "min" => list.sort_by(|a, b| a.min.partial_cmp(&b.min).unwrap_or(Ordering::Equal)),
            "fgm" => list.sort_by(|a, b| a.fgm.partial_cmp(&b.fgm).unwrap_or(Ordering::Equal)),
            "fga" => list.sort_by(|a, b| a.fga.partial_cmp(&b.fga).unwrap_or(Ordering::Equal)),
            "fg_pct" => list.sort_by(|a, b| a.fg_pct.partial_cmp(&b.fg_pct).unwrap_or(Ordering::Equal)),

            "fg3m" => list.sort_by(|a, b| a.fg3m.partial_cmp(&b.fg3m).unwrap_or(Ordering::Equal)),
            "fg3a" => list.sort_by(|a, b| a.fg3a.partial_cmp(&b.fg3a).unwrap_or(Ordering::Equal)),
            "fg3_pct" => list.sort_by(|a, b| a.fg3_pct.partial_cmp(&b.fg3_pct).unwrap_or(Ordering::Equal)),

            "ftm" => list.sort_by(|a, b| a.ftm.partial_cmp(&b.ftm).unwrap_or(Ordering::Equal)),
            "fta" => list.sort_by(|a, b| a.fta.partial_cmp(&b.fta).unwrap_or(Ordering::Equal)),
            "ft_pct" => list.sort_by(|a, b| a.ft_pct.partial_cmp(&b.ft_pct).unwrap_or(Ordering::Equal)),

            "oreb" => list.sort_by(|a, b| a.oreb.partial_cmp(&b.oreb).unwrap_or(Ordering::Equal)),
            "dreb" => list.sort_by(|a, b| a.dreb.partial_cmp(&b.dreb).unwrap_or(Ordering::Equal)),
            "reb" => list.sort_by(|a, b| a.reb.partial_cmp(&b.reb).unwrap_or(Ordering::Equal)),

            "ast" => list.sort_by(|a, b| a.ast.partial_cmp(&b.ast).unwrap_or(Ordering::Equal)),
            "tov" => list.sort_by(|a, b| a.tov.partial_cmp(&b.tov).unwrap_or(Ordering::Equal)),
            "stl" => list.sort_by(|a, b| a.stl.partial_cmp(&b.stl).unwrap_or(Ordering::Equal)),
            "blk" => list.sort_by(|a, b| a.blk.partial_cmp(&b.blk).unwrap_or(Ordering::Equal)),
            "blka" => list.sort_by(|a, b| a.blka.partial_cmp(&b.blka).unwrap_or(Ordering::Equal)),

            "pf" => list.sort_by(|a, b| a.pf.partial_cmp(&b.pf).unwrap_or(Ordering::Equal)),
            "pfd" => list.sort_by(|a, b| a.pfd.partial_cmp(&b.pfd).unwrap_or(Ordering::Equal)),
            "pts" => list.sort_by(|a, b| a.pts.partial_cmp(&b.pts).unwrap_or(Ordering::Equal)),

            "plus_minus" => list.sort_by(|a, b| a.plus_minus.partial_cmp(&b.plus_minus).unwrap_or(Ordering::Equal)),
            "nba_fantasy_pts" => list.sort_by(|a, b| {
                a.nba_fantasy_pts
                    .partial_cmp(&b.nba_fantasy_pts)
                    .unwrap_or(Ordering::Equal)
            }),

            "dd2" => list.sort_by_key(|a| a.dd2),
            "td3" => list.sort_by_key(|a| a.td3),
            "start_year" => list.sort_by_key(|a| a.start_year),
            "end_year" => list.sort_by_key(|a| a.end_year),

            _ => list.sort_by(|a, b| a.pts.partial_cmp(&b.pts).unwrap_or(Ordering::Equal)),
        }

    }
}

#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
#[derive(Deserialize, Serialize, Default, PartialEq, Clone)]
pub struct NbaGeneralPartialNoYearsNoFantasy {
    pub player_name: String,
    pub team: String,
    pub age: f64,
    pub gp: i16,
    pub w: i16,
    pub l: i16,
    pub min: f64,
    pub fgm: f64,
    pub fga: f64,
    pub fg_pct: f64,
    pub fg3m: f64,
    pub fg3a: f64,
    pub fg3_pct: f64,
    pub ftm: f64,
    pub fta: f64,
    pub ft_pct: f64,
    pub oreb: f64,
    pub dreb: f64,
    pub reb: f64,
    pub ast: f64,
    pub tov: f64,
    pub stl: f64,
    pub blk: f64,
    pub blka: f64,
    pub pf: f64,
    pub pfd: f64,
    pub pts: f64,
    pub plus_minus: f64,
    pub dd2: i16,
    pub td3: i16,
}
impl fmt::Display for NbaGeneral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
Player: {} ({})
Team: {}
Age: {:.1}, Seasons: {}–{}
Games: {} | W-L: {}-{} ()
Minutes: {:.1}
FG: {:.1}/{:.1} ({:.3})
3PT: {:.1}/{:.1} ({:.3})
FT: {:.1}/{:.1} ({:.3})
REB: {:.1} (OREB {:.1}, DREB {:.1})
AST: {:.1} | TOV: {:.1}
STL: {:.1} | BLK: {:.1} | BLKA: {:.1}
PF: {:.1} | PFD: {:.1}
PTS: {:.1}
+/-: {:.1}
DD2: {:.1} | TD3: {:.1}",
            self.player_name,
            self.nickname,
            self.team,
            self.age,
            self.start_year,
            self.end_year,
            self.gp,
            self.w,
            self.l,
            self.min,
            self.fgm,
            self.fga,
            self.fg_pct,
            self.fg3m,
            self.fg3a,
            self.fg3_pct,
            self.ftm,
            self.fta,
            self.ft_pct,
            self.reb,
            self.oreb,
            self.dreb,
            self.ast,
            self.tov,
            self.stl,
            self.blk,
            self.blka,
            self.pf,
            self.pfd,
            self.pts,
            self.plus_minus,
            self.dd2,
            self.td3,
        )
    }
}

impl DBStructSupport for NbaGeneralPartialNoYearsNoFantasy {

    const TABLE_NAME: &'static str = "nba_general";
    const SELECT_COLUMNS: &'static str = "player_name, team, age, gp, w, l, min, fgm, fga, fg_pct, fg3m, \
    fg3a, fg3_pct, ftm, fta, ft_pct, oreb, dreb, reb, ast, tov, stl, blk, blka, pf, pfd, pts, plus_minus, dd2, td3";

    fn sort_vector(&self, list: & mut Vec<Self>, column: String, descending: bool)
    where
        Self: Sized,
    {
        use std::cmp::Ordering;

        list.sort_by(|a, b| {
            let ord = match column.as_str() {
                "player_name" => a.player_name.cmp(&b.player_name),
                "team" => a.team.cmp(&b.team),

                // total_cmp handles f64 perfectly (including NaN)
                "age" => a.age.total_cmp(&b.age),
                "gp" => a.gp.cmp(&b.gp),
                "w" => a.w.cmp(&b.w),
                "l" => a.l.cmp(&b.l),

                "min" => a.min.total_cmp(&b.min),
                "fgm" => a.fgm.total_cmp(&b.fgm),
                "fga" => a.fga.total_cmp(&b.fga),
                "fg_pct" => a.fg_pct.total_cmp(&b.fg_pct),

                "fg3m" => a.fg3m.total_cmp(&b.fg3m),
                "fg3a" => a.fg3a.total_cmp(&b.fg3a),
                "fg3_pct" => a.fg3_pct.total_cmp(&b.fg3_pct),

                "ftm" => a.ftm.total_cmp(&b.ftm),
                "fta" => a.fta.total_cmp(&b.fta),
                "ft_pct" => a.ft_pct.total_cmp(&b.ft_pct),

                "oreb" => a.oreb.total_cmp(&b.oreb),
                "dreb" => a.dreb.total_cmp(&b.dreb),
                "reb" => a.reb.total_cmp(&b.reb),

                "ast" => a.ast.total_cmp(&b.ast),
                "tov" => a.tov.total_cmp(&b.tov),
                "stl" => a.stl.total_cmp(&b.stl),
                "blk" => a.blk.total_cmp(&b.blk),
                "blka" => a.blka.total_cmp(&b.blka),

                "pf" => a.pf.total_cmp(&b.pf),
                "pfd" => a.pfd.total_cmp(&b.pfd),

                "pts" => a.pts.total_cmp(&b.pts),
                "plus_minus" => a.plus_minus.total_cmp(&b.plus_minus),

                "dd2" => a.dd2.cmp(&b.dd2),
                "td3" => a.td3.cmp(&b.td3),

                _ => a.pts.total_cmp(&b.pts),
            };

            // If descending is true, flip the order
            if descending {
                ord.reverse()
            } else {
                ord
            }
        });

    }
}
