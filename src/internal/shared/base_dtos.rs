use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Base DTO trait that all DTOs should implement
pub trait BaseDto: Send + Sync {
    fn validate(&self) -> Result<(), String>;
}

/// Base entity trait for domain entities
pub trait BaseEntity: Send + Sync {
    type Id;
    
    fn id(&self) -> &Self::Id;
    fn created_at(&self) -> chrono::DateTime<chrono::Utc>;
    fn updated_at(&self) -> chrono::DateTime<chrono::Utc>;
}

/// Common pagination request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationRequest {
    pub page: u32,
    pub page_size: u32,
}

impl Default for PaginationRequest {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
        }
    }
}

impl PaginationRequest {
    pub fn new(page: u32, page_size: u32) -> Self {
        Self { page, page_size }
    }

    pub fn offset(&self) -> u32 {
        (self.page - 1) * self.page_size
    }

    pub fn limit(&self) -> u32 {
        self.page_size
    }
}

/// Common pagination response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationResponse<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

impl<T> PaginationResponse<T> {
    pub fn new(data: Vec<T>, total: u64, page: u32, page_size: u32) -> Self {
        let total_pages = if page_size > 0 {
            ((total as f64) / (page_size as f64)).ceil() as u32
        } else {
            0
        };

        Self {
            data,
            total,
            page,
            page_size,
            total_pages,
        }
    }
}

/// Common filter options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterOptions {
    pub sort_by: Option<String>,
    pub sort_order: SortOrder,
    pub search: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl Default for FilterOptions {
    fn default() -> Self {
        Self {
            sort_by: None,
            sort_order: SortOrder::Asc,
            search: None,
        }
    }
}

/// Base response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn error(error: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error.into()),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// ID wrapper for type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId<T> {
    id: Uuid,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> EntityId<T> {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self {
            id,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.id
    }
}

impl<T> From<Uuid> for EntityId<T> {
    fn from(id: Uuid) -> Self {
        Self::from_uuid(id)
    }
}

impl<T> std::fmt::Display for EntityId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
