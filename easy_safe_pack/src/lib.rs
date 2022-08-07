extern crate core;

mod persistence;

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::persistence::disk_pers::{create_or_load_map_env, MapEnv};

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
