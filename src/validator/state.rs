use super::error::ValidationError;

#[derive(Debug, Default)]
pub struct ValidationState {
    errors: Vec<ValidationError>,
}

impl ValidationState {
    pub fn new() -> ValidationState {
        ValidationState::default()
    }

    pub fn new_with_error(error: ValidationError) -> ValidationState {
        ValidationState { errors: vec![error] }
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
