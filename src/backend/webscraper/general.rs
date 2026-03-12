use std::time::Duration;
use axum::http::{HeaderMap, HeaderValue};
use reqwest::Error;
use serde_json::{Value};
use anyhow::Result;

pub struct Years {
    start_year: i16,
    end_year: i16,
    season_type: String,
}


#[derive(Debug)]
enum StatsError {
    NotFound,
    ParseError,
}

//TODO:
fn check_season_type(season_type: &str) -> bool {
    true
}

#[derive(Debug)]
pub struct ScapperSportsInfo {
    pub full_url: String, //https://stats.nba.com/stats/leaguedashplayerstats?College=&Conference=&Country=&DateFrom=&DateTo=&Division=&DraftPick=&DraftYear=&GameScope=&GameSegment=&Height=&ISTRound=&LastNGames=0&LeagueID=00&Location=&MeasureType=Base&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&PlusMinus=N&Rank=N&Season={new_year_start}-{new_year_end}&SeasonSegment=&SeasonType={season_type}%20Season&ShotClockRange=&StarterBench=&TeamID=0&VsConference=&VsDivision=&Weight=
    domain_url: &'static str, // https:://www.nba.com
    host_url: &'static str, //stats.nba.com
}

pub trait BuildScapperSportsInfo{

    fn build_scapper_sports_info(start_year: i16, end_year: i16, season_type: &'static str) -> ScapperSportsInfo;

}

pub struct Baseball;
pub struct AmericanFootball;
pub struct Basketball;
pub struct IceHockey;



impl BuildScapperSportsInfo for Basketball {
    fn build_scapper_sports_info(start_year: i16, end_year: i16, season_type: &'static str) -> ScapperSportsInfo {
        let new_year_start = start_year.to_string();
        let new_year_end = (end_year % 100).to_string();

        let url = format!(
            "https://stats.nba.com/stats/leaguedashplayerstats?College=&Conference=&Country=&DateFrom=&DateTo=&Division=&DraftPick=&DraftYear=&GameScope=&GameSegment=&Height=&ISTRound=&LastNGames=0&LeagueID=00&Location=&MeasureType=Base&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&PlusMinus=N&Rank=N&Season={}-{}&SeasonSegment=&SeasonType={}%20Season&ShotClockRange=&StarterBench=&TeamID=0&VsConference=&VsDivision=&Weight=",
            new_year_start,
            new_year_end,
            season_type
        );

        ScapperSportsInfo {
            full_url: url,
            domain_url: "www.nba.com",
            host_url: "stats.nba.com",
        }
    }
}


pub async fn get_response(sports_struct : ScapperSportsInfo) -> Result<reqwest::Response, Error> {
    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("application/json, text/plain"));
    headers.insert("Accept-Language", HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert("Connection", HeaderValue::from_static("keep-alive"));
    headers.insert("Host", HeaderValue::from_static(sports_struct.host_url));
    headers.insert("Origin", HeaderValue::from_static(sports_struct.domain_url)); //https:://www.nba.com
    headers.insert("Referer", HeaderValue::from_static(sports_struct.domain_url)); //http:://www.nba.com
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-site"));
    headers.insert("User-Agent", HeaderValue::from_static(
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36"
    ));
    headers.insert("sec-ch-ua", HeaderValue::from_static(r#""Chromium";v="140", "Not=A?Brand";v="24", "Google Chrome";v="140""#));
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"Windows\""));
    headers.insert("Cookie", HeaderValue::from_static("nbaStatsFsp=1")); // optional but can help

    // Create HTTP client
    let client : reqwest::Client = reqwest::Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(5))
        .build()?;
    let response = client.get(sports_struct.full_url).send().await?;
    Ok(response)
}

//TODO: Check to see if we should get a raw_json string instead so we can use serde_json::from_str
//generic function to get sports data from a specific scrapper for each sport
pub async fn get_sport_stats_json(sports_struct : ScapperSportsInfo) -> Result<Value, Error> {
    let response = get_response(sports_struct).await?;
    let body = response.json().await?;
    Ok(body)
}

pub async fn get_sport_stats_string(sports_struct : ScapperSportsInfo) -> Result<String, Error> {
    let response = get_response(sports_struct).await?;
    let body = response.text().await?;
    Ok(body)
}

pub trait CleanAndConvertJsonToStruct: Sized {
    fn clean_and_convert_json_to_struct(data : &Value, start_year: i16, end_year: i16) -> Result<Vec<Self>>;
}