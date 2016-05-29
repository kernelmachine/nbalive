use serde_json::Value;
use stats::Stat;
use err::NBAError;

pub fn find_idx(ls: &Vec<Value>, elem: &'static str) -> Result<usize, NBAError> {
    ls.iter()
      .position(|ref r| r.as_string() == Some(elem))
      .ok_or(NBAError::HeaderMissingError(elem.to_owned()))

}

pub fn parse_playbyplay(headers: &Vec<Value>, rows: &Vec<Value>) -> Result<Vec<Stat>, NBAError> {
    let mut raw_features = Vec::new();
    let game_id_idx = try!(find_idx(headers, "GAME_ID"));
    let eventnum_idx = try!(find_idx(headers, "EVENTNUM"));
    let eventmsgtype_idx = try!(find_idx(headers, "EVENTMSGTYPE"));
    let eventmsgactiontype_idx = try!(find_idx(headers, "EVENTMSGACTIONTYPE"));
    let period_idx = try!(find_idx(headers, "PERIOD"));
    let wctimestring_idx = try!(find_idx(headers, "WCTIMESTRING"));
    let pctimestring_idx = try!(find_idx(headers, "PCTIMESTRING"));
    let homedescription_idx = try!(find_idx(headers, "HOMEDESCRIPTION"));
    let neutraldescription_idx = try!(find_idx(headers, "NEUTRALDESCRIPTION"));
    let visitordescription_idx = try!(find_idx(headers, "VISITORDESCRIPTION"));
    let score_idx = try!(find_idx(headers, "SCORE"));
    let scoremargin_idx = try!(find_idx(headers, "SCOREMARGIN"));
    for row in rows {
        let row = row.as_array().unwrap();
        let feat = Stat::PlayByPlay {
            game_id: row[game_id_idx].as_i64(),
            eventnum: row[eventnum_idx].as_u64(),
            eventmsgtype: row[eventmsgtype_idx].as_u64(),
            eventmsgactiontype: row[eventmsgactiontype_idx].as_u64(),
            period: row[period_idx].as_u64(),
            wctimestring: row[wctimestring_idx].as_string().map(|x| x.to_owned()),
            pctimestring: row[pctimestring_idx].as_string().map(|x| x.to_owned()),
            homedescription: row[homedescription_idx]
                                 .as_string()
                                 .map(|x| x.to_owned()),
            neutraldescription: row[neutraldescription_idx]
                                    .as_string()
                                    .map(|x| x.to_owned()),
            visitordescription: row[visitordescription_idx]
                                    .as_string()
                                    .map(|x| x.to_owned()),
            score: row[score_idx].as_u64(),
            scoremargin: row[scoremargin_idx].as_u64(),
        };
        raw_features.push(feat);
    }
    Ok(raw_features)
}

pub fn parse_gameheader(headers: &Vec<Value>, rows: &Vec<Value>) -> Result<Vec<Stat>, NBAError> {
    let mut raw_features = Vec::new();
    let gamedate_est_idx = try!(find_idx(headers, "GAMEDATE_EST"));
    let game_sequence_idx = try!(find_idx(headers, "GAME_SEQUENCE"));
    let game_id_idx = try!(find_idx(headers, "GAME_ID"));
    let game_status_id_idx = try!(find_idx(headers, "GAME_STATUS_ID"));
    let game_status_text_idx = try!(find_idx(headers, "GAME_STATUS_TEXT"));
    let gamecode_idx = try!(find_idx(headers, "GAMECODE"));
    let home_team_id_idx = try!(find_idx(headers, "HOME_TEAM_ID"));
    let visitor_team_id_idx = try!(find_idx(headers, "VISITOR_TEAM_ID"));
    let season_idx = try!(find_idx(headers, "SEASON"));
    let live_period_idx = try!(find_idx(headers, "LIVE_PERIOD"));
    let live_pc_time_idx = try!(find_idx(headers, "LIVE_PC_TIME"));
    let natl_tv_broadcaster_abbreviation_idx = try!(find_idx(headers,
                                                             "NATL_TV_BROADCASTER_ABBREVIATION"));
    let live_period_time_bcast_idx = try!(find_idx(headers, "LIVE_PERIOD_TIME_BCAST"));
    let wh_status_idx = try!(find_idx(headers, "WH_STATUS"));

    for row in rows {
        let row = row.as_array().unwrap();
        let feat = Stat::GameHeader {
            gamedate_est: row[gamedate_est_idx].as_string().map(|x| x.to_owned()),
            game_sequence: row[game_sequence_idx].as_u64(),
            game_id: row[game_id_idx].as_string().map(|x| x.to_owned()),
            game_status_id: row[game_status_id_idx].as_u64(),
            game_status_text: row[game_status_text_idx].as_string().map(|x| x.to_owned()),
            gamecode: row[gamecode_idx].as_string().map(|x| x.to_owned()),
            home_team_id: row[home_team_id_idx].as_string().map(|x| x.to_owned()),
            visitor_team_id: row[visitor_team_id_idx].as_string().map(|x| x.to_owned()),
            season: row[season_idx].as_u64(),
            live_period: row[live_period_idx].as_u64(),
            live_pc_time: row[live_pc_time_idx].as_u64(),
            natl_tv_broadcaster_abbreviation: row[natl_tv_broadcaster_abbreviation_idx]
                                                  .as_string()
                                                  .map(|x| x.to_owned()),
            live_period_time_bcast: row[live_period_time_bcast_idx]
                                        .as_string()
                                        .map(|x| x.to_owned()),
            wh_status: row[wh_status_idx].as_u64(),
        };
        raw_features.push(feat);
    }
    Ok(raw_features)
}


pub fn parse_eastconfstandings(headers: &Vec<Value>,
                               rows: &Vec<Value>)
                               -> Result<Vec<Stat>, NBAError> {
    let mut raw_features = Vec::new();
    let team_id_idx = try!(find_idx(headers, "TEAM_ID"));
    let league_id_idx = try!(find_idx(headers, "LEAGUE_ID"));
    let season_id_idx = try!(find_idx(headers, "SEASON_ID"));
    let standings_date_idx = try!(find_idx(headers, "STANDINGSDATE"));
    let conference_idx = try!(find_idx(headers, "CONFERENCE"));
    let team_idx = try!(find_idx(headers, "TEAM"));
    let g_idx = try!(find_idx(headers, "G"));
    let wins_idx = try!(find_idx(headers, "W"));
    let losses_idx = try!(find_idx(headers, "L"));
    let w_pct_idx = try!(find_idx(headers, "W_PCT"));
    let home_record_idx = try!(find_idx(headers, "HOME_RECORD"));
    let road_record_idx = try!(find_idx(headers, "ROAD_RECORD"));

    for row in rows {
        let row = row.as_array().unwrap();
        let feat = Stat::EastConfStandings {
            team_id: row[team_id_idx].as_string().map(|x| x.to_owned()),
            league_id: row[league_id_idx].as_string().map(|x| x.to_owned()),
            season_id: row[season_id_idx].as_string().map(|x| x.to_owned()),
            standings_date: row[standings_date_idx].as_string().map(|x| x.to_owned()),
            conference: row[conference_idx].as_string().map(|x| x.to_owned()),
            team: row[team_idx].as_string().map(|x| x.to_owned()),
            g: row[g_idx].as_u64(),
            wins: row[wins_idx].as_u64(),
            losses: row[losses_idx].as_u64(),
            w_pct: row[w_pct_idx].as_f64(),
            home_record: row[home_record_idx].as_string().map(|x| x.to_owned()),
            road_record: row[road_record_idx].as_string().map(|x| x.to_owned()),
        };
        raw_features.push(feat);
    }
    Ok(raw_features)
}

pub fn parse_westconfstandings(headers: &Vec<Value>,
                               rows: &Vec<Value>)
                               -> Result<Vec<Stat>, NBAError> {
    let mut raw_features = Vec::new();
    let team_id_idx = try!(find_idx(headers, "TEAM_ID"));
    let league_id_idx = try!(find_idx(headers, "LEAGUE_ID"));
    let season_id_idx = try!(find_idx(headers, "SEASON_ID"));
    let standings_date_idx = try!(find_idx(headers, "STANDINGSDATE"));
    let conference_idx = try!(find_idx(headers, "CONFERENCE"));
    let team_idx = try!(find_idx(headers, "TEAM"));
    let g_idx = try!(find_idx(headers, "G"));
    let wins_idx = try!(find_idx(headers, "W"));
    let losses_idx = try!(find_idx(headers, "L"));
    let w_pct_idx = try!(find_idx(headers, "W_PCT"));
    let home_record_idx = try!(find_idx(headers, "HOME_RECORD"));
    let road_record_idx = try!(find_idx(headers, "ROAD_RECORD"));

    for row in rows {
        let row = row.as_array().unwrap();
        let feat = Stat::WestConfStandings {
            team_id: row[team_id_idx].as_string().map(|x| x.to_owned()),
            league_id: row[league_id_idx].as_string().map(|x| x.to_owned()),
            season_id: row[season_id_idx].as_string().map(|x| x.to_owned()),
            standings_date: row[standings_date_idx].as_string().map(|x| x.to_owned()),
            conference: row[conference_idx].as_string().map(|x| x.to_owned()),
            team: row[team_idx].as_string().map(|x| x.to_owned()),
            g: row[g_idx].as_u64(),
            wins: row[wins_idx].as_u64(),
            losses: row[losses_idx].as_u64(),
            w_pct: row[w_pct_idx].as_f64(),
            home_record: row[home_record_idx].as_string().map(|x| x.to_owned()),
            road_record: row[road_record_idx].as_string().map(|x| x.to_owned()),
        };
        raw_features.push(feat);
    }
    Ok(raw_features)
}


pub fn parse_teamroster(headers: &Vec<Value>, rows: &Vec<Value>) -> Result<Vec<Stat>, NBAError> {
    let mut raw_features = Vec::new();
    let team_id_idx = try!(find_idx(headers, "TeamID"));
    let season_idx = try!(find_idx(headers, "SEASON"));
    let league_id_idx = try!(find_idx(headers, "LeagueID"));
    let player_idx = try!(find_idx(headers, "PLAYER"));
    let num_idx = try!(find_idx(headers, "NUM"));
    let position_idx = try!(find_idx(headers, "POSITION"));
    let height_idx = try!(find_idx(headers, "HEIGHT"));
    let weight_idx = try!(find_idx(headers, "WEIGHT"));
    let birth_date_idx = try!(find_idx(headers, "BIRTH_DATE"));
    let age_idx = try!(find_idx(headers, "AGE"));
    let exp_idx = try!(find_idx(headers, "EXP"));
    let school_idx = try!(find_idx(headers, "SCHOOL"));
    let player_id_idx = try!(find_idx(headers, "PLAYER_ID"));
    for row in rows {
        let row = row.as_array().unwrap();
        let feat = Stat::TeamRoster {
            team_id: row[team_id_idx].as_string().map(|x| x.to_owned()),
            season: row[season_idx].as_string().map(|x| x.to_owned()),
            league_id: row[league_id_idx].as_string().map(|x| x.to_owned()),
            player: row[player_idx].as_string().map(|x| x.to_owned()),
            num: row[num_idx].as_string().map(|x| x.to_owned()),
            position: row[position_idx].as_string().map(|x| x.to_owned()),
            height: row[height_idx].as_string().map(|x| x.to_owned()),
            weight: row[weight_idx].as_string().map(|x| x.to_owned()),
            birth_date: row[birth_date_idx].as_string().map(|x| x.to_owned()),
            age: row[age_idx].as_u64(),
            exp: row[exp_idx].as_string().map(|x| x.to_owned()),
            school: row[school_idx].as_string().map(|x| x.to_owned()),
            player_id: row[player_id_idx].as_string().map(|x| x.to_owned()),
        };
        raw_features.push(feat);
    }
    Ok(raw_features)
}
