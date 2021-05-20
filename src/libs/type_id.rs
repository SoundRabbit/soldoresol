use std::any;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

pub fn type_id<T>() -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(any::type_name::<T>().as_bytes());
    format!("{:X}", hasher.finish())
}
