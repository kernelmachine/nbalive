


pub enum StatType {
    PlayByPlay,
}


#[derive(Debug)]
pub enum Stat {
    PlayByPlay {
        game_id: Option<i64>,
        eventnum: Option<u64>,
        eventmsgtype: Option<u64>,
        eventmsgactiontype: Option<u64>,
        period: Option<u64>,
        wctimestring: Option<String>,
        pctimestring: Option<String>,
        homedescription: Option<String>,
        neutraldescription: Option<String>,
        visitordescription: Option<String>,
        score: Option<u64>,
        scoremargin: Option<u64>,
    },
}
