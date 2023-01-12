
pub struct ProductPageQueryRequest{
    pub page_size:i64,
    pub page_index:i64,
    pub keyword:Option<String>,
    pub category_id:Option<i64>,
    pub status:Option<String>,
    pub sort:Option<String>
}