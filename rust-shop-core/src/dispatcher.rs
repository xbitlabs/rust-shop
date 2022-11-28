use crate::RequestCtx;

pub struct Dispatcher;

impl Dispatcher {
    pub async fn do_dispatch(req:&mut RequestCtx)->anyhow::Result<()>{
        Ok(())
    }
}