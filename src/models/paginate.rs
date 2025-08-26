use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
}

pub fn default_page() -> u32 { 1 }
pub fn default_limit() -> u32 { 20 }
pub fn default_sort_order() -> String { "asc".to_string() }

pub const PAGINATE_MAX_LIMIT: u32 = 100;

#[derive(Debug, Deserialize)]
pub struct PaginateQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

impl Default for PaginateQuery {
    fn default() -> Self {
        Self {
            page: default_page(),
            limit: default_limit()
        }
    }
}