use std::collections::HashMap;

use anymap::AnyMap;
use crate::rustations::{GetterSetter, PropertyKey};

pub struct WorldObjectProperties{
    properties: AnyMap,
    
}

impl WorldObjectProperties {
    pub fn new() -> Self {
        Self { properties: AnyMap::new() }
    }
}


/// This is just something that was quick to throw together. might need to 
/// redo it to allow multiple proporties to be got at once, thou that sould 
/// only be a problems with editing it, shoulf be abble to read multiple at 
/// once.
/// 
/// could replace the HashMaps in the AnyMap with RefCel<HashMap> or go 
/// back to my original idea of stores the HashMaps in their own field
impl<K, T> GetterSetter<K, T> for WorldObjectProperties {
    fn insert(&mut self, key: PropertyKey<K, T>, value: T) -> Option<T> {
        let key: K =  key.into();
        if let Some(hashmap) = self.properties.get::<HashMap<K, T>>() {
            hashmap.insert(key, value)
        } else {
            let mut hashmap = HashMap::<K, T>::new();
            hashmap.insert(key, value);
            self.properties.insert(value);
            None
        }
    }

    fn get(&self, key: &PropertyKey<K, T>) -> Option<&T> {
        self.properties.get::<T>().get(key.into())
    }

    fn get_mut(&self, key: &PropertyKey<K, T>) -> Option<&mut T> {
        self.properties.get_mut::<T>().get_mut(key.into())
    }
}
/*
impl GetterSetter<IntPropertyKey, Int> for WorldObjectProperties {
    fn insert(&mut self, key: PropertyKey<IntPropertyKey, Int>, value: Int) -> Option<Int> {
        self.int_properties.insert(key.into(), value)
    }

    fn get(&self, key: &PropertyKey<IntPropertyKey, Int>, value: Int) -> Option<&Int> {
        self.int_properties.get(key.into())
    }

    fn get_mut(&self, key: &PropertyKey<IntPropertyKey, Int>, value: Int) -> Option<&mut Int> {
        self.int_properties.get_mut(key.into())
    }
}
impl GetterSetter<StringPropertyKey, String> for WorldObjectProperties {
    fn insert(&mut self, key: PropertyKey<StringPropertyKey, String>, value: String) -> Option<String> {
        self.string_properties.insert(key.into(), value)
    }

    fn get(&self, key: &PropertyKey<StringPropertyKey, String>, value: String) -> Option<&String> {
        self.string_properties.get(key.into())
    }

    fn get_mut(&self, key: &PropertyKey<StringPropertyKey, String>, value: String) -> Option<&mut String> {
        self.string_properties.get_mut(key.into())
    }
}
*/