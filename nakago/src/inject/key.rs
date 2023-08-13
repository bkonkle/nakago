use std::{
    any::{type_name, Any, TypeId},
    fmt::Display,
};

/// A type key for the map
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Key {
    pub(crate) id: Id,
    pub(crate) type_name: String,
}

/// A dependency ID, which can be either a TypeId or a unique String dependency tag.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Id {
    /// A TypeId from `std::any`
    TypeId(TypeId),

    /// A unique Tag
    Tag(&'static str),
}

impl Key {
    pub(crate) fn from_type_id<T: Any + ?Sized>() -> Self {
        Self {
            id: Id::TypeId(TypeId::of::<T>()),
            type_name: type_name::<T>().to_string(),
        }
    }

    pub(crate) fn from_tag<T: Any + ?Sized>(tag: &'static str) -> Self {
        Self {
            id: Id::Tag(tag),
            type_name: type_name::<T>().to_string(),
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.id {
            Id::TypeId(_) => write!(f, "{}", self.type_name),
            Id::Tag(tag) => write!(f, "Tag({tag})"),
        }
    }
}
