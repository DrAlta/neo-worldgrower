use crate::rustations::PropertyKey;

pub trait WorldObject{
    fn set_property<PK, T>(&mut self, property_key: PropertyKey<PK, T>, value: T);
}