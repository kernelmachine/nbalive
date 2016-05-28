use serde_json::Value;
use stats::Stat;

pub fn parse_playbyplay(rows: &Vec<Value>) -> Vec<Stat> {
    let mut raw_features = Vec::new();
    for row in rows {
        let row = row.as_array().unwrap();
        let feat = Stat::PlayByPlay {
            game_id: row[0].as_i64(),
            eventnum: row[1].as_u64(),
            eventmsgtype: row[2].as_u64(),
            eventmsgactiontype: row[3].as_u64(),
            period: row[4].as_u64(),
            wctimestring: row[5].as_string().map(|x| x.to_owned()),
            pctimestring: row[6].as_string().map(|x| x.to_owned()),
            homedescription: row[7].as_string().map(|x| x.to_owned()),
            neutraldescription: row[8].as_string().map(|x| x.to_owned()),
            visitordescription: row[9].as_string().map(|x| x.to_owned()),
            score: row[10].as_u64(),
            scoremargin: row[11].as_u64(),
        };
        raw_features.push(feat);
    }
    raw_features
}
