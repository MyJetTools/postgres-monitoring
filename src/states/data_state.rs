use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum DataState<T: Debug + Clone> {
    None,
    Loading,
    Loaded(T),
}

impl<T: Debug + Clone> DataState<T> {
    pub fn is_none(&self) -> bool {
        match self {
            DataState::None => true,
            _ => false,
        }
    }

    pub fn is_loading(&self) -> bool {
        match self {
            DataState::Loading => true,
            _ => false,
        }
    }

    pub fn to_not_loaded_cases(&self) -> Option<NotLoadedCases> {
        match self {
            DataState::None => NotLoadedCases::None.into(),
            DataState::Loading => NotLoadedCases::Loading.into(),
            DataState::Loaded(_) => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum NotLoadedCases {
    None,
    Loading,
}
