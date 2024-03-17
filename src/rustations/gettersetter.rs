use std::marker;

pub trait GetterSetter<PK: PartialEq + std::hash::Hash, T> {
    fn insert(&mut self, key: PropertyKey<PK, T>, value: T) -> Option<T>;
    fn get(&self, key: &PropertyKey<PK, T>, value: T) -> Option<&T>;
    fn get_mut(&self, key: &PropertyKey<PK, T>, value: T) -> Option<&mut T>;

}

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct PropertyKey<PK: PartialEq + std::hash::Hash, T> {
    id: PK,
    _marker: marker::PhantomData<T>,
}
impl<PK: PartialEq + std::hash::Hash, T> std::hash::Hash for PropertyKey<PK, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<PK, T> From<PropertyKey<PK, T>> for PK {
    fn from(value: PropertyKey<PK, T>) -> Self {
        value.id
    }
}
impl<PK, T> From<&PropertyKey<PK, T>> for &PK {
    fn from(value: PropertyKey<PK, T>) -> Self {
        &value.id
    }
}