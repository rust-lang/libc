use crate::ir::{Constant, Field, Function, Static, Struct, TypeAlias, Union};

pub struct Predicate<T>(Box<dyn Fn(T) -> bool>);

impl<T> Predicate<T> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(T) -> bool + 'static,
    {
        Self(Box::new(f))
    }

    pub fn test(&self, value: T) -> bool {
        (self.0)(value)
    }
}

impl<T, F> From<F> for Predicate<T>
where
    F: Fn(T) -> bool + 'static,
{
    fn from(f: F) -> Self {
        Predicate(Box::new(f))
    }
}

// To be used as SkipItem::Struct((|s: Struct| !s.public()).into())
enum SkipItem {
    Constant(Predicate<Constant>),
    Function(Predicate<Function>),
    Static(Predicate<Static>),
    TypeAlias(Predicate<TypeAlias>),
    Struct(Predicate<Struct>),
    Field(Predicate<Field>),
    Union(Predicate<Union>),
}
