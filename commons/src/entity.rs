use chrono::{DateTime, Utc};

pub trait Entity<ID> {
    fn id(&self) -> &ID;
}

pub trait EntityWithTimestamp<ID>: Entity<ID> {
    fn created_at(&self) -> &DateTime<Utc>;
    fn updated_at(&self) -> &DateTime<Utc>;
}
