use std::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::backend::webscraper::{CleanAndConvertJsonToStruct};
use anyhow::{Context, Result};
use crate::shared::nba::NbaGeneral;

//TODO: for each option below we need to do the following:
//1.get data
//2. parse data into json
//3. send data to db
//iterate through general usage
// https://stats.nba.com/stats/leaguedashplayerstats?College=&Conference=&Country=&DateFrom=&DateTo=&Division=&DraftPick=&DraftYear=&GameScope=&GameSegment=&Height=&ISTRound=&LastNGames=0&LeagueID=00&Location=&MeasureType=Base&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&PlusMinus=N&Rank=N&Season=2025-26&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&StarterBench=&TeamID=0&VsConference=&VsDivision=&Weight=

#[derive(Deserialize, Serialize, Debug)]
pub struct NbaGeneralBaseJson {
    player_id: i32,
    player_name: String,
    nickname: String,
    team_id: i32,
    team_abbreviation: String,
    age: f64,
    gp: i16,
    w: i16,
    l: i16,
    w_pct: f64,
    min: f64,
    fgm: f64,
    fga: f64,
    fg_pct: f64,
    fg3m: f64,
    fg3a: f64,
    fg3_pct: f64,
    ftm: f64,
    fta: f64,
    ft_pct: f64,
    oreb: f64,
    dreb: f64,
    reb: f64,
    ast: f64,
    tov: f64,
    stl: f64,
    blk: f64,
    blka: f64,
    pf: f64,
    pfd: f64,
    pts: f64,
    plus_minus: f64,
    nba_fantasy_pts: f64,
    dd2: i16,
    td3: i16,
    wnba_fantasy_pts: f64,
    gp_rank: f64,
    w_rank: f64,
    l_rank: f64,
    w_pct_rank: f64,
    min_rank: f64,
    fgm_rank: f64,
    fga_rank: f64,
    fg_pct_rank: f64,
    fg3m_rank: f64,
    fg3a_rank: f64,
    fg3_pct_rank: f64,
    ftm_rank: f64,
    fta_rank: f64,
    ft_pct_rank: f64,
    oreb_rank: f64,
    dreb_rank: f64,
    reb_rank: f64,
    ast_rank: f64,
    tov_rank: f64,
    stl_rank: f64,
    blk_rank: f64,
    blka_rank: f64,
    pf_rank: f64,
    pfd_rank: f64,
    pts_rank: f64,
    plus_minus_rank: f64,
    nba_fantasy_pts_rank: f64,
    dd2_rank: i16,
    td3_rank: i16,
    wnba_fantasy_pts_rank: f64,
    team_count: f64,
}

//helper function that can be removed eventually
impl fmt::Display for NbaGeneralBaseJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "playerID: {}, {} ({}) - Team: {} ID: {}, Age: {}, PTS: {:.1}, AST: {:.1}, REB: {:.1}",
            self.player_id, self.player_name, self.nickname, self.team_abbreviation, self.team_id, self.age,
            self.pts, self.ast, self.reb
        )
    }
}

impl CleanAndConvertJsonToStruct for NbaGeneralBaseJson {
    fn clean_and_convert_json_to_struct(data: &Value, start_year: i16, end_year: i16) -> Result<Vec<Self>> {
        let player_data = data.pointer("/resultSets/0/rowSet").context("failed to get rowSet")?;
        Ok(serde_json::from_value(player_data.to_owned())?)
    }
}

impl CleanAndConvertJsonToStruct for NbaGeneral {
    fn clean_and_convert_json_to_struct(data: &Value, start_year: i16, end_year: i16) -> Result<Vec<Self>> {
        let nba_general_general_base_json = NbaGeneralBaseJson::clean_and_convert_json_to_struct(data, start_year, end_year)?;
        let nba_general: Vec<Self> = nba_general_general_base_json
            .into_iter()
            .map(|a| Self {
                player_name: a.player_name,
                nickname: a.nickname,
                team: a.team_abbreviation,
                age: a.age,
                gp: a.gp,
                w: a.w,
                l: a.l,
                min: a.min,
                fgm: a.fgm,
                fga: a.fga,
                fg_pct: a.fg_pct,
                fg3m: a.fg3m,
                fg3a: a.fg3a,
                fg3_pct: a.fg3_pct,
                ftm: a.ftm,
                fta: a.fta,
                ft_pct: a.ft_pct,
                oreb: a.oreb,
                dreb: a.dreb,
                reb: a.reb,
                ast: a.ast,
                tov: a.tov,
                stl: a.stl,
                blk: a.blk,
                blka: a.blka,
                pf: a.pf,
                pfd: a.pfd,
                pts: a.pts,
                plus_minus: a.plus_minus,
                nba_fantasy_pts: a.nba_fantasy_pts,
                dd2: a.dd2,
                td3: a.td3,
                start_year: start_year,
                end_year: end_year,
            })
            .collect();

        Ok(nba_general)
    }
}


static GENERAL_OPTIONS: [&str; 3] = [
    "Base",
    "Advanced",
    "Violations",
];




//Clutch
// https://stats.nba.com/stats/leaguedashplayerclutch?AheadBehind=Ahead%20or%20Behind&ClutchTime=Last%205%20Minutes&College=&Conference=&Country=&DateFrom=&DateTo=&Division=&DraftPick=&DraftYear=&GameScope=&GameSegment=&Height=&ISTRound=&LastNGames=0&LeagueID=00&Location=&MeasureType=Base&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&PlusMinus=N&PointDiff=5&Rank=N&Season=2025-26&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&StarterBench=&TeamID=0&VsConference=&VsDivision=&Weight=

static CLUTCH_OPTIONS: [&str; 3] = [
    "Advanced",
    "Base",
    "Scoring",
];


//synergy for playtype -->
// https://stats.nba.com/stats/synergyplaytypes?LeagueID=00&PerMode=PerGame&PlayType=Isolation&PlayerOrTeam=P&SeasonType=Regular%20Season&SeasonYear=2025-26&TypeGrouping=offensive

static PLAYTYPE_OPTIONS: [&str; 12] = [
    "PlayType",
    "Isolation",
    "Transition",
    "PRBallHandler",
    "PRRollman",
    "Postup",
    "Spotup",
    "Handoff",
    "Cut",
    "OffScreen",
    "OffRebound",
    "Misc",
];


//Tracking below
//https://stats.nba.com/stats/leaguedashptstats?College=&Conference=&Country=&DateFrom=&DateTo=&Division=&DraftPick=&DraftYear=&GameScope=&Height=&ISTRound=&LastNGames=0&LeagueID=00&Location=&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PerMode=PerGame&PlayerExperience=&PlayerOrTeam=Player&PlayerPosition=&PtMeasureType=PaintTouch&Season=2025-26&SeasonSegment=&SeasonType=Regular%20Season&StarterBench=&TeamID=0&VsConference=&VsDivision=&Weight=

static TRACKING_OPTIONS: [&str; 13] = [
    "PtMeasureType",
    "Drives",
    "Defense",
    "ChatShoot",
    "Passing",
    "Possessions",
    "PullUpShot",
    "Rebounding",
    "Efficiency",
    "SpeedDistance",
    "ElbowTouch",
    "PostTouch",
    "PaintTouch",
];

//Shot DashBoard
//https://stats.nba.com/stats/leaguedashplayerptshot?CloseDefDistRange=&College=&Conference=&Country=&DateFrom=&DateTo=&Division=&DraftPick=&DraftYear=&DribbleRange=&GameScope=&GameSegment=&GeneralRange=Overall&Height=&ISTRound=&LastNGames=0&LeagueID=00&Location=&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&PlusMinus=N&Rank=N&Season=2025-26&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&ShotDistRange=&StarterBench=&TeamID=0&TouchTimeRange=&VsConference=&VsDivision=&Weight=

static SHOTDASHBOARD_SHOTCLOCKRANGE: [&str; 6] = [
    "24-22",
    "22-18%20Very%20Early",
    "18-15%20Early",
    "15-7%20Average",
    "7-4%20Late",
    "4-0%20Very%20Late",
];
//Dribble
//https://stats.nba.com/stats/leaguedashplayerptshot?CloseDefDistRange=&College=&Conference=&Country=&DateFrom=&DateTo=&Division=&DraftPick=&DraftYear=&DribbleRange=0%20Dribbles&GameSegment=&GeneralRange=&Height=&LastNGames=0&LeagueID=00&Location=&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&Season=2025-26&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&ShotDistRange=&StarterBench=&TeamID=0&TouchTimeRange=&VsConference=&VsDivision=&Weight=

static SHOTDASHBOARD_DRIBBLE: [&str;5] = [
    "0%20Dribbles",
    "1%20Dribble",
    "2%20Dribbles",
    "3-6%20Dribbles",
    "7%2B%20Dribbles",
];

//Touch Time
//TouchTimeRange
//https://stats.nba.com/stats/leaguedashplayerptshot?CloseDefDistRange=&College=&Conference=&Country=&DateFrom=&DateTo=&Division=&DraftPick=&DraftYear=&DribbleRange=&GameScope=&GameSegment=&GeneralRange=&Height=&LastNGames=0&LeagueID=00&Location=&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&PlusMinus=N&Rank=N&Season=2025-26&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&ShotDistRange=&StarterBench=&TeamID=0&TouchTimeRange=Touch%20%3C%202%20Seconds&VsConference=&VsDivision=&Weight=
static SHOTDASHBOARD_TOUCHTIME: [&str;3] =[
    "Touch%20%3C%202%20Seconds",
    "Touch%202-6%20Seconds",
    "Touch%206%2B%20Seconds",
];

//ClosestDefender
//CloseDefDistRange
//https://stats.nba.com/stats/leaguedashplayerptshot?CloseDefDistRange=0-2%20Feet%20-%20Very%20Tight&College=&Conference=&Country=&DateFrom=&DateTo=&Division=&DraftPick=&DraftYear=&DribbleRange=&GameScope=&GameSegment=&GeneralRange=&Height=&LastNGames=0&LeagueID=00&Location=&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&Season=2025-26&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&ShotDistRange=&StarterBench=&TeamID=0&TouchTimeRange=&VsConference=&VsDivision=&Weight=
static SHOTDASHBOARD_CLOSESTDEFENDER: [&str;4] = [
    "0-2%",
    "2-4%",
    "4-6%",
    "6%2B%",
];

//Shooting Distance
//https://stats.nba.com/stats/leaguedashplayershotlocations?College=&Conference=&Country=&DateFrom=&DateTo=&DistanceRange=5ft%20Range&Division=&DraftPick=&DraftYear=&GameScope=&GameSegment=&Height=&ISTRound=&LastNGames=0&Location=&MeasureType=Base&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&PlusMinus=N&Rank=N&Season=2025-26&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&StarterBench=&TeamID=0&VsConference=&VsDivision=&Weight=

//Opponent Shooting
//https://stats.nba.com/stats/leaguedashplayershotlocations?College=&Conference=&Country=&DateFrom=&DateTo=&DistanceRange=5ft%20Range&Division=&DraftPick=&DraftYear=&GameScope=&GameSegment=&Height=&ISTRound=&LastNGames=0&Location=&MeasureType=Opponent&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&PlusMinus=N&Rank=N&Season=2025-26&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&StarterBench=&TeamID=0&VsConference=&VsDivision=&Weight=

//Defense Statics
//https://stats.nba.com/stats/leaguedashptdefend?College=&Conference=&Country=&DateFrom=&DateTo=&DefenseCategory=3%20Pointers&Division=&DraftPick=&DraftYear=&GameSegment=&Height=&LastNGames=0&LeagueID=00&Location=&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&Season=2025-26&SeasonSegment=&SeasonType=Regular%20Season&StarterBench=&TeamID=0&VsConference=&VsDivision=&Weight=
//https://stats.nba.com/stats/leaguedashptdefend?College=&Conference=&Country=&DateFrom=&DateTo=&DefenseCategory=2%20Pointers&Division=&DraftPick=&DraftYear=&GameSegment=&Height=&LastNGames=0&LeagueID=00&Location=&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&Season=2025-26&SeasonSegment=&SeasonType=Regular%20Season&StarterBench=&TeamID=0&VsConference=&VsDivision=&Weight=


//Hustle
// https://stats.nba.com/stats/leaguehustlestatsplayer?College=&Conference=&Country=&DateFrom=&DateTo=&Division=&DraftPick=&DraftYear=&GameScope=&Height=&ISTRound=&LastNGames=0&LeagueID=00&Location=&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&PlayerExperience=&PlayerPosition=&PlusMinus=N&Rank=N&Season=2025-26&SeasonSegment=&SeasonType=Regular%20Season&TeamID=0&VsConference=&VsDivision=&Weight=

//BoxOut
//https://stats.nba.com/stats/leaguehustlestatsplayer?College=&Conference=&Country=&DateFrom=&DateTo=&Division=&DraftPick=&DraftYear=&GameScope=&Height=&ISTRound=&LastNGames=0&LeagueID=00&Location=&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&PlayerExperience=&PlayerPosition=&PlusMinus=N&Rank=N&Season=2025-26&SeasonSegment=&SeasonType=Regular%20Season&TeamID=0&VsConference=&VsDivision=&Weight=
