use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::pagination::Pagination;
use crate::Result;
/// This struct is used to represent the query parameters that are sent to the
/// server endpoints for pagination.
#[derive(Debug, Deserialize)]
pub struct PageTurn {
    pub from: Option<String>,
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct PageTurnReq<T, U> {
    pub page_turn: PageTurn,
    pub orders: Option<U>,
    pub filter: Option<T>,
}
#[derive(Debug, Serialize)]
pub struct PageTurnResponse<T> {
    pub pagination: Pagination,
    pub data: Vec<T>,
}

impl<T> PageTurnResponse<T>
{
    pub fn new(page_turner: PageTurn, data: Vec<T>) -> Self {
        let pagination = Pagination::build_from_request_query(page_turner).count(1).build();
        Self {
            pagination,
            data,
        }
    }   
}
#[async_trait]
pub trait Page<T, U> where T: Send + Sync , U: Send + Sync  {
    async fn page(&self, req: T) -> Result<U>;
 }