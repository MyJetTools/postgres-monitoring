#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LocationState {
    Dashboard,
    DbSize,
}

impl LocationState {
    pub fn copy_state(&self) -> Self {
        *self
    }
}
