use chrono::{DateTime, Utc};

pub trait WithId<ID> {
    fn id(&self) -> &ID;
}

pub trait WithCreated {
    fn created_at(&self) -> &DateTime<Utc>;
}

pub trait WithUpdated {
    fn updated_at(&self) -> &DateTime<Utc>;
}

pub trait WithDeleted {
    fn deleted_at(&self) -> Option<&DateTime<Utc>>;
}

#[macro_export]
macro_rules! derive_with_id {
    ($struct_name:ident, $id_type:ty) => {
        impl WithId<$id_type> for $struct_name {
            fn id(&self) -> &$id_type {
                &self.id
            }
        }
    };
}

#[macro_export]
macro_rules! derive_with_created {
    ($struct_name:ident) => {
        impl WithCreated for $struct_name {
            fn created_at(&self) -> &DateTime<Utc> {
                &self.created_at
            }
        }
    };
}

#[macro_export]
macro_rules! derive_with_updated {
    ($struct_name:ident) => {
        impl WithUpdated for $struct_name {
            fn updated_at(&self) -> &DateTime<Utc> {
                &self.updated_at
            }
        }
    };
}

#[macro_export]
macro_rules! derive_with_deleted {
    ($struct_name:ident) => {
        impl WithDeleted for $struct_name {
            fn deleted_at(&self) -> Option<&DateTime<Utc>> {
                self.deleted_at.as_ref()
            }
        }
    };
}


pub use {derive_with_id, derive_with_created, derive_with_updated, derive_with_deleted};
