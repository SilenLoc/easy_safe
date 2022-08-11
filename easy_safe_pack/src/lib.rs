
//! yes it is not easy_save since this crate also wants to be safe to use while saving I called
//! it easy_safe
//!
//! This is a crate that should make it possible to save and load strings with keys from disk
//! It works as a map and actually wraps a map
//! It keeps the ownership model as you know it
//! but you may always call ```create_or_load_map_env("somename")```
//! at any place in your code, with the same name it was first being called
//! This makes it possible to leak data through places where the data should not be
//! as you can imagine, you should be careful to not trying to use multiple environments concurrently
//!
//! # Example
//! ```
//! use easy_safe::{create_or_load_map_env, MapEnv};
//!
//! let mut  map_env: MapEnv = create_or_load_map_env("somename");
//! map_env.put("somekey".to_string(), "somevalue".to_string());
//! let value = map_env.get("somekey".to_string()).unwrap();
//! assert_eq!(value, "somevalue");
//! ```
mod mapenv;
pub use crate::mapenv::disk_pers::{create_or_load_map_env, MapEnv};

extern crate core;

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::mapenv::disk_pers::{create_or_load_map_env, MapEnv};

    fn clear_files(name: &str) {
        if fs::remove_file(name).is_ok() {};
    }

    #[test]
    fn create_env_and_check_name() {
        let map_env: MapEnv = create_or_load_map_env("name1");
        assert_eq!(map_env.get_name(), "name1");
        clear_files("name1");
    }

    #[test]
    fn create_env_put_and_get() {
        let mut map_env: MapEnv = create_or_load_map_env("name2");
        map_env.put("somekey".to_string(), "somevalue".to_string());
        let value = map_env.get("somekey".to_string()).unwrap();
        assert_eq!(value, "somevalue");
        clear_files("name2");
    }

    #[test]
    fn create_env_put_and_delete() {
        let mut map_env: MapEnv = create_or_load_map_env("name4");
        map_env.put("somekey".to_string(), "somevalue".to_string());
        map_env.delete("somekey".to_string());
        let value = map_env.get("somekey".to_string());
        assert_eq!(value, None);
        clear_files("name4");
    }
}


