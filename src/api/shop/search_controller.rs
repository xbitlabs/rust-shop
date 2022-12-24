mod search_controller {
    use sonic_channel::*;

    pub fn main() -> result::Result<()> {
        let channel = SearchChannel::start("localhost:1491", "SecretPassword")?;

        let objects = channel.query(QueryRequest::new(
            Dest::col_buc("collection", "bucket"),
            "recipe",
        ))?;
        dbg!(objects);

        Ok(())
    }
}
