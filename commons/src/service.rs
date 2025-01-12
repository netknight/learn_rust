use crate::entity::WithId;
use crate::errors::ServiceError;

pub type ServiceResult<T> = Result<T, ServiceError>;

//#[async_trait]
pub trait CrudService<ID, DATA, ENTITY: WithId<ID>> {
    fn create(&self, data: DATA) -> ServiceResult<ENTITY>;
    fn read(&self, id: ID) -> ServiceResult<ENTITY>;
    fn update(&self, id: ID, data: DATA) -> ServiceResult<ENTITY>;
    fn delete(&self, id: ID) -> ServiceResult<()>;
}
