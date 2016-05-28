
#[derive(RustcDecodable, RustcEncodable)]
pub struct PlayByPlayQuery {
    pub gameid: String,
    pub startperiod: String,
    pub endperiod: String,
}
