
#[cfg(test)]
mod tests {
    use stats::*;
    use queries::*;
    use scrape::*;
    #[test]
    fn test_get_json() {
        let payload = PlayByPlayQuery {
            gameid: "0041400106".to_owned(),
            startperiod: "0".to_owned(),
            endperiod: "14".to_owned(),
        };
        let stats = Stat::get_json(StatType::PlayByPlay, &payload);
        println!("{:?}", stats);
    }
}
