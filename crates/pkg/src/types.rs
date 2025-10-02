use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type EntityId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationRequest {
    pub page: u32,
    pub page_size: u32,
}

impl Default for PaginationRequest {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 10,
        }
    }
}

impl PaginationRequest {
    pub fn offset(&self) -> u32 {
        (self.page - 1) * self.page_size
    }

    pub fn limit(&self) -> u32 {
        self.page_size
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

impl<T> PaginationResponse<T> {
    pub fn new(items: Vec<T>, total: u64, page: u32, page_size: u32) -> Self {
        let total_pages = ((total as f64) / (page_size as f64)).ceil() as u32;
        Self {
            items,
            total,
            page,
            page_size,
            total_pages,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: None,
        }
    }

    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: Some(message),
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            message: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

impl Default for SortDirection {
    fn default() -> Self {
        Self::Asc
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortRequest {
    pub field: String,
    pub direction: SortDirection,
}
