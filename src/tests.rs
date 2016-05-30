
#[cfg(test)]
mod tests {
    use stats::*;
    use queries::*;
    use scrape::*;
    use parse::*;
    use constants::*;
    use serde_json::Value;
    use serde_json;
    #[test]
    fn test_get_playbyplay() {
        let payload = Query::PlayByPlayQuery {
            gameid: "0041400106".to_owned(),
            startperiod: "0".to_owned(),
            endperiod: "14".to_owned(),
        };
        let stats = Stat::get_data(StatType::PlayByPlay, payload);
    }

    #[test]
    fn test_get_gameheader() {
        let payload = Query::GameHeaderQuery {
            leagueid: "00".to_owned(),
            gamedate: "02/21/2015".to_owned(),
            dayoffset: "0".to_owned(),
        };
        let stats = Stat::get_data(StatType::GameHeader, payload);
    }
    #[test]
    fn test_get_eastconfstandings() {
        let payload = Query::EastConfStandingsQuery {
            leagueid: "00".to_owned(),
            gamedate: "02/21/2015".to_owned(),
            dayoffset: "0".to_owned(),
        };
        let stats = Stat::get_data(StatType::EastConfStandings, payload);
    }

    #[test]
    fn test_get_westconfstandings() {
        let payload = Query::WestConfStandingsQuery {
            leagueid: "00".to_owned(),
            gamedate: "02/21/2015".to_owned(),
            dayoffset: "0".to_owned(),
        };
        let stats = Stat::get_data(StatType::WestConfStandings, payload);
    }

    #[test]
    fn test_get_teamroster() {
        let payload = Query::TeamRosterQuery {
            teamid: "1610612739".to_owned(),
            season: "2015-16".to_owned(),
        };
        let stats = Stat::get_data(StatType::TeamRoster, payload);
    }

    #[test]
    fn test_find_idx() {
        let payload = Query::TeamRosterQuery {
            teamid: "1610612739".to_owned(),
            season: "2015-1f6".to_owned(),
        };
        let data: Value = Stat::post_query(TEAMROSTER_BASE_URL.to_owned(), payload).unwrap();
        let data = data.as_object().expect("could not objectify");
        let data = data.get("resultSets").expect("could not resultSet").as_array().unwrap();
        let data = data[0].as_object().expect("could not objectify");
        let headers = data.get("headers").expect("could not header");
        let headers = headers.as_array().unwrap();
        let idx = find_idx(headers, "GAME_ID");
        assert!(idx.is_err());
        let idx = find_idx(headers, "TeamID");
        assert!(idx.is_ok());

    }

}
