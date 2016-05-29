
#[derive(RustcDecodable, RustcEncodable)]
pub struct PlayByPlayQuery {
    pub gameid: String,
    pub startperiod: String,
    pub endperiod: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct GameHeaderQuery {
    pub leagueid: String,
    pub gamedate: String,
    pub dayoffset: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct EastConfStandingsQuery {
    pub leagueid: String,
    pub gamedate: String,
    pub dayoffset: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct WestConfStandingsQuery {
    pub leagueid: String,
    pub gamedate: String,
    pub dayoffset: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct TeamRosterQuery {
    pub season: String,
    pub teamid: String,
}
