#[derive(RustcDecodable, RustcEncodable, Debug)]
pub enum Query {
    PlayByPlayQuery {
        gameid: String,
        startperiod: String,
        endperiod: String,
    },

    GameHeaderQuery {
        leagueid: String,
        gamedate: String,
        dayoffset: String,
    },

    EastConfStandingsQuery {
        leagueid: String,
        gamedate: String,
        dayoffset: String,
    },

    WestConfStandingsQuery {
        leagueid: String,
        gamedate: String,
        dayoffset: String,
    },

    TeamRosterQuery {
        season: String,
        teamid: String,
    },
}
