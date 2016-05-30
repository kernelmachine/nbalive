use rustc_serialize::json;
use serde_json;
use serde_json::Value;
use regex::Regex;
use hyper::{Client, Url};
use hyper::header::{Referer, UserAgent};
use std::io::Read;
use std::collections::HashMap;
use stats::{StatType, Stat};
use parse::*;
use constants::*;
use err::NBAError;
use queries::Query;


pub trait Scrape {
    fn check_payload(payload: &Query);
    fn post_query(base_url: String, payload: Query) -> Result<Value, NBAError>;
    fn get_data(stat: StatType, payload: Query) -> Result<Vec<Stat>, NBAError>;
}


impl Scrape for Stat {
    fn check_payload(payload: &Query) {
        match payload {
            &Query::PlayByPlayQuery { ref gameid, ref startperiod, ref endperiod } => {
                let gameid_re = Regex::new(r"\d{10}").unwrap();
                let startperiod_re = Regex::new(r"[0-9]|1[0-4]").unwrap();
                let endperiod_re = Regex::new(r"[0-9]|1[0-4]").unwrap();
                assert!(gameid_re.is_match(gameid));
                assert!(startperiod_re.is_match(startperiod));
                assert!(endperiod_re.is_match(endperiod));
            }
            &Query::GameHeaderQuery { ref leagueid, ref gamedate, ref dayoffset } => {
                let leagueid_re = Regex::new(r"00|01").unwrap();
                let gamedate_re = Regex::new(r"^\d{2}/\d{2}/\d{4}$").unwrap();
                let dayoffset_re = Regex::new(r"\d{1}|\d{2}").unwrap();
                assert!(leagueid_re.is_match(leagueid));
                assert!(gamedate_re.is_match(gamedate));
                assert!(dayoffset_re.is_match(dayoffset));

            }

            &Query::EastConfStandingsQuery { ref leagueid, ref gamedate, ref dayoffset } => {
                let leagueid_re = Regex::new(r"00|01").unwrap();
                let gamedate_re = Regex::new(r"^\d{2}/\d{2}/\d{4}$").unwrap();
                let dayoffset_re = Regex::new(r"\d{1}|\d{2}").unwrap();
                assert!(leagueid_re.is_match(leagueid));
                assert!(gamedate_re.is_match(gamedate));
                assert!(dayoffset_re.is_match(dayoffset));
            }
            &Query::WestConfStandingsQuery { ref leagueid, ref gamedate, ref dayoffset } => {
                let leagueid_re = Regex::new(r"00|01").unwrap();
                let gamedate_re = Regex::new(r"^\d{2}/\d{2}/\d{4}$").unwrap();
                let dayoffset_re = Regex::new(r"\d{1}|\d{2}").unwrap();
                assert!(leagueid_re.is_match(leagueid));
                assert!(gamedate_re.is_match(gamedate));
                assert!(dayoffset_re.is_match(dayoffset));
            }
            &Query::TeamRosterQuery { ref season, ref teamid } => {
                let season_re = Regex::new(r"^\d{4}-\d{2}$").unwrap();
                let team_id_re = Regex::new(r"\d{10}").unwrap();
                assert!(season_re.is_match(season));
                assert!(team_id_re.is_match(teamid));
            }
        }
    }
    fn post_query(base_url: String, payload: Query) -> Result<Value, NBAError> {
        let client = Client::new();
        let mut url = try!(Url::parse(&base_url));

        let s = try!(json::encode(&payload));
        let dict: HashMap<String, String> = try!(json::decode(&s));
        for (key, value) in dict {
            url.query_pairs_mut().append_pair(&key, &value);
        }

        let referer = REFERER.to_owned();
        let user_agent = USERAGENT.to_owned();

        let mut response = try!(client.get(url)
                                      .header(Referer(referer))
                                      .header(UserAgent(user_agent))
                                      .send());

        let body: String = {
            let mut s = String::new();
            let _ = response.read_to_string(&mut s);
            s
        };

        let data: Value = try!(serde_json::from_str(&body));
        Ok(data)

    }

    fn get_data(stat: StatType, payload: Query) -> Result<Vec<Stat>, NBAError> {
        Stat::check_payload(&payload);
        let base_url = match stat {
            StatType::PlayByPlay => PLAYBYPLAY_BASE_URL,
            StatType::GameHeader => GAMEHEADER_BASE_URL,
            StatType::EastConfStandings => EASTCONFSTANDINGS_BASE_URL,
            StatType::WestConfStandings => WESTCONFSTANDINGS_BASE_URL,
            StatType::TeamRoster => TEAMROSTER_BASE_URL,

        };

        let data: Value = try!(Stat::post_query(base_url.to_owned(), payload));
        let data = data.as_object().unwrap();

        let data = data.get("resultSets")
                       .ok_or(NBAError::MissingField("resultSets"))
                       .unwrap()
                       .as_array()
                       .unwrap();

        let data = match stat {
            StatType::PlayByPlay => data[0].as_object().unwrap(),
            StatType::GameHeader => data[0].as_object().unwrap(),
            StatType::EastConfStandings => data[4].as_object().unwrap(),
            StatType::WestConfStandings => data[5].as_object().unwrap(),
            StatType::TeamRoster => data[0].as_object().unwrap(),

        };
        let headers = data.get("headers").ok_or(NBAError::MissingField("headers")).unwrap();
        let headers = headers.as_array().unwrap();
        let data = data.get("rowSet").ok_or(NBAError::MissingField("rowSet")).unwrap();
        let rows = data.as_array().unwrap();

        match stat {
            StatType::PlayByPlay => parse_playbyplay(&headers, &rows),
            StatType::GameHeader => parse_gameheader(&headers, &rows),
            StatType::EastConfStandings => parse_eastconfstandings(&headers, &rows),
            StatType::WestConfStandings => parse_westconfstandings(&headers, &rows),
            StatType::TeamRoster => parse_teamroster(&headers, &rows),

        }


    }
}
