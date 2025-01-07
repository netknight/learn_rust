use crate::entity::Entity;

pub type ServiceResult<T> = Result<T, Box<dyn std::error::Error>>;

pub trait CrudService<ID, DATA, ENTITY: Entity<ID>> {
    fn create(&self, data: DATA) -> ServiceResult<ENTITY>;
    fn read(&self, id: ID) -> ServiceResult<ENTITY>;
    fn update(&self, id: ID, data: DATA) -> ServiceResult<ENTITY>;
    fn delete(&self, id: ID) -> ServiceResult<()>;
}
