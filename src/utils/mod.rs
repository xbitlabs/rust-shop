pub fn page_offset(page_size:i64,page_index:i64)->i64{
    (page_index - 1) * page_size
}
pub fn page_count(record_count:i64,page_size:i64)->i64 {
    if record_count % page_size == 0 {
        record_count / page_size
    } else {
        (record_count / page_size) + 1
    }
}

pub mod datetime{
    use chrono::{DateTime, Local, TimeZone, Utc};

    pub fn now() ->DateTime<Utc>{
        let local = Local::now();
        let utc = Utc
            .from_local_datetime(&local.naive_local())
            .single()
            .unwrap();
        utc
    }
}