use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct PageParams {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

pub struct DefaulOrder {
   
}
#[derive(Debug, Deserialize)]
pub struct PageTurnReq<T, U> {
    pub page_turn: PageParams,
    pub orders: Option<U>,
    pub filter: Option<T>,
}
#[derive(Debug, Serialize)]
pub struct PageTurnResponse<T> {
    pub total_count: i64,
    pub data: Vec<T>,
}

impl<T> PageTurnResponse<T>
{
    pub fn new(total_count: i64, data: Vec<T>) -> Self {
        Self {
            total_count,
            data,
        }
    }   
}
#[async_trait]
pub trait Page<T, U> where T: Send + Sync , U: Send + Sync  {
    async fn page(&self, req: T) -> Result<U>;
 }