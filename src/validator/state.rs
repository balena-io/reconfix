use crate::validator::error::ValidationError;

#[derive(Debug, Default)]
pub struct ValidationState {
    errors: Vec<ValidationError>,
}

impl ValidationState {
    pub fn new() -> ValidationState {
        ValidationState { errors: vec![] }
    }

    pub fn new_with_error(error: ValidationError) -> ValidationState {
        ValidationState { errors: vec![error] }
    }

    pub fn new_with_errors<I>(errors: I) -> ValidationState
    where
        I: IntoIterator<Item = ValidationError>,
    {
        ValidationState {
            errors: errors.into_iter().collect(),
        }
    }

    pub fn push_error(&mut self, error: ValidationError) {
        self.errors.push(error)
    }

    pub fn extend(&mut self, other: ValidationState) {
        self.errors.extend(other.errors);
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn errors(&self) -> &Vec<ValidationError> {
        &self.errors
    }
}

impl From<ValidationError> for ValidationState {
    fn from(error: ValidationError) -> ValidationState {
        ValidationState { errors: vec![error] }
    }
}

impl<T> From<T> for ValidationState
where
    T: IntoIterator<Item = ValidationError>,
{
    fn from(iter: T) -> Self {
        ValidationState {
            errors: iter.into_iter().collect(),
        }
    }
}
