// use rustc_serialize::json::Json;
use rustc_serialize::{Encodable, json};
use serde_json;
use serde_json::Value;

use hyper::{Client, Url};
use hyper::header::{Referer, UserAgent};
use std::io::Read;
use std::collections::HashMap;
use stats::{StatType, Stat};
use parse::*;
use constants::*;
use err::NBAError;

pub fn post_query<T>(base_url: String, payload: &T) -> Result<String, NBAError>
    where T: Encodable
{
    let client = Client::new();
    let mut url = try!(Url::parse(&base_url));
    let s = try!(json::encode(payload));
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

    let body = {
        let mut s = String::new();
        let _ = response.read_to_string(&mut s);
        s
    };

    Ok(body)
}



pub trait Scrape {
    fn get_json<T>(stat: StatType, payload: &T) -> Result<Vec<Stat>, NBAError> where T: Encodable;
}


impl Scrape for Stat {
    fn get_json<T>(stat: StatType, payload: &T) -> Result<Vec<Stat>, NBAError>
        where T: Encodable
    {
        let base_url = match stat {
            StatType::PlayByPlay => PLAYBYPLAY_BASE_URL,
            StatType::GameHeader => GAMEHEADER_BASE_URL,
            StatType::EastConfStandings => EASTCONFSTANDINGS_BASE_URL,
            StatType::WestConfStandings => WESTCONFSTANDINGS_BASE_URL,
            StatType::TeamRoster => TEAMROSTER_BASE_URL,

        };
        let s: String = try!(post_query(base_url.to_owned(), &payload));

        // match stat {
        //     StatType::TeamRoster => println!("{:?}", s),
        //     _ => println!("yes"),
        // }
        let data: Value = try!(serde_json::from_str(&s));
        let data = data.as_object().unwrap();

        let data = data.get("resultSets")
                       .ok_or(NBAError::MissingField("resultSets"))
                       .unwrap()
                       .as_array()
                       .expect("cannot convert json to array");
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
