// use rustc_serialize::json::Json;
use rustc_serialize::{Encodable, json};
use serde_json;
use serde_json::Value;

use hyper::{Client, Url};
use hyper::header::{Referer, UserAgent};
use std::io::Read;
use std::collections::HashMap;
use stats::{StatType, Stat};
use parse::parse_playbyplay;
use constants::*;

fn post_query<T>(base_url: String, payload: &T) -> String
    where T: Encodable
{
    let client = Client::new();
    let mut url = Url::parse(&base_url).expect("url could not be parsed");
    let s = json::encode(payload).expect("could not encode into json");
    let dict: HashMap<String, String> = json::decode(&s).expect("could not decode into hashmap");
    for (key, value) in dict {
        url.query_pairs_mut().append_pair(&key, &value);
    }

    let referer = REFERER.to_owned();
    let user_agent = USERAGENT.to_owned();

    let mut response = client.get(url)
                             .header(Referer(referer))
                             .header(UserAgent(user_agent))
                             .send()
                             .ok()
                             .expect("url could not be pinged");
    let body = {
        let mut s = String::new();
        let _ = response.read_to_string(&mut s);
        s
    };

    body
}



pub trait Scrape {
    fn get_json<T>(stat: StatType, payload: &T) -> Vec<Stat> where T: Encodable;
}


impl Scrape for Stat {
    fn get_json<T>(stat: StatType, payload: &T) -> Vec<Stat>
        where T: Encodable
    {
        let base_url = match stat {
            StatType::PlayByPlay => PLAYBYPLAY_BASE_URL,
        };
        let s: String = post_query(base_url.to_owned(), &payload);
        let data: Value = serde_json::from_str(&s).expect("could not jsonify");
        let data = data.as_object().expect("could not objectify");
        let data = data.get("resultSets").expect("could not resultSet").as_array().unwrap();
        let data = data[0].as_object().expect("could not objectify");
        let data = data.get("rowSet").expect("could not rowSet");
        let rows = data.as_array().unwrap();

        match stat {
            StatType::PlayByPlay => parse_playbyplay(&rows),
        }


    }
}
