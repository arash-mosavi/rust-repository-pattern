use mongodb::bson::{doc, Document};
use serde::Serialize;


pub struct MongoFilter {
    filter: Document,
}

impl MongoFilter {
    pub fn new() -> Self {
        Self {
            filter: doc! {},
        }
    }

    pub fn eq<T: Serialize>(mut self, field: &str, value: T) -> Self {
        self.filter.insert(field, mongodb::bson::to_bson(&value).unwrap());
        self
    }

    pub fn ne<T: Serialize>(mut self, field: &str, value: T) -> Self {
        self.filter.insert(
            field,
            doc! { "$ne": mongodb::bson::to_bson(&value).unwrap() },
        );
        self
    }

    pub fn gt<T: Serialize>(mut self, field: &str, value: T) -> Self {
        self.filter.insert(
            field,
            doc! { "$gt": mongodb::bson::to_bson(&value).unwrap() },
        );
        self
    }

    pub fn lt<T: Serialize>(mut self, field: &str, value: T) -> Self {
        self.filter.insert(
            field,
            doc! { "$lt": mongodb::bson::to_bson(&value).unwrap() },
        );
        self
    }

    pub fn build(self) -> Document {
        self.filter
    }
}

impl Default for MongoFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mongo_filter() {
        let filter = MongoFilter::new()
            .eq("name", "test")
            .gt("age", 18)
            .build();

        assert!(filter.contains_key("name"));
        assert!(filter.contains_key("age"));
    }
}
