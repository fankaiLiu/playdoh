use serde::{Serialize, Deserialize};

 
const LIMIT: i64 = 100;
const OFFSET: i64 = 0;

#[derive(Debug, Serialize)]
pub struct Pagination {
    pub count: i64,
    pub offset: i64,
    pub limit: i64,
}

impl Pagination {
    pub fn build_from_request_query(query: PageParams) -> PaginationBuilder {
        let limit = query
            .limit
            // Make sure the requested limit is not greater than the maximum allowed
            // limit.
            .map(|limit| std::cmp::min(limit, LIMIT))
            .unwrap_or(LIMIT);

        let offset = query.offset.unwrap_or(OFFSET);

        PaginationBuilder {
            count: None,
            offset,
            limit,
        }
    }
}

pub struct PaginationBuilder {
    pub count: Option<i64>,
    pub offset: i64,
    pub limit: i64,
}

impl Default for PaginationBuilder {
    fn default() -> Self {
        Self {
            count: None,
            offset: OFFSET,
            limit: LIMIT,
        }
    }
}

impl PaginationBuilder {
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }

    pub fn build(self) -> Pagination {
        Pagination {
            count: self.count.expect("Pagination count to be set"),
            offset: self.offset,
            limit: self.limit,
        }
    }
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
#[derive(Debug, Deserialize)]
pub struct PageParams {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

 
 