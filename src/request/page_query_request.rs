pub trait PageQueryRequest {
    fn get_page_index(&self) -> Option<u32>;
    fn set_page_index(&mut self, page_index: Option<u32>);

    fn get_page_size(&self) -> Option<u32>;
    fn set_page_size(&mut self, page_size: Option<u32>);
}

macro_rules! impl_page_query_request {
    ($($str:ident),+) => {
        $(impl PageQueryRequest for $str {

            fn get_page_index(&self) -> Option<u32> {
                self.page_index
            }

            fn set_page_index(&mut self, page_index: Option<u32>) {
                self.page_index = page_index;
            }

            fn get_page_size(&self) -> Option<u32> {
                self.page_size
            }

            fn set_page_size(&mut self,page_size:Option<u32>)  {
                self.page_size = page_size;
            }
        })*
    };
}
