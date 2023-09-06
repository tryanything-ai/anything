use crate::error::AnythingError;

pub trait Validation
where
    Self: Sized,
{
    fn validate(&self) -> Result<(), AnythingError>;
    fn validated(self) -> Result<Self, AnythingError> {
        self.validate().and_then(|_| Ok(self))
    }
}
